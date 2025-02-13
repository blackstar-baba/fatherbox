use std::cell::OnceCell;
use std::collections::HashMap;
use std::future::Future;
use std::ops::Deref;
use std::sync::{Mutex, RwLock};
use std::vec;

use anyhow::Context;
use async_openai::config::OpenAIConfig;
use async_openai::types::Role::{System, User};
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};
use async_openai::Client;
use chrono::Utc;
use futures::future::ok;
use futures::StreamExt;
use log::{debug, error, info};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::{json, Error};
use tauri::api::http::{Body, ClientBuilder, HttpRequestBuilder, ResponseType};
use tauri::Window;
use uuid::Uuid;

use crate::AppResponse;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelData {
    pub models: Vec<Model>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatInfo {
    id: String,
    name: String,
    created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    name: String,
    model: String,
    modified_at: String,
    size: i64,
    digest: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    role: String,
    text: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateBody {
    name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNameBody {
    id: String,
    name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CommonBody {
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    pub id: String,
    pub prompt: String,
    pub model: String,
    pub stream: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RegenerateBody {
    id: String,
    index: usize,
    model: String,
    stream: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EditBody {
    id: String,
    index: usize,
    prompt: String,
    model: String,
    stream: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    id: String, // chat id
    text: Option<String>,
    index: usize,
    error: Option<String>,
}

static GLOBAL_CHATS: Lazy<RwLock<Box<Vec<ChatInfo>>>> = Lazy::new(|| {
    let m = Box::new(vec![]);
    RwLock::new(m)
});

static GLOBAL_CHAT_MESSAGES: Lazy<RwLock<HashMap<String, Box<Vec<Message>>>>> = Lazy::new(|| {
    let m = HashMap::new();
    RwLock::new(m)
});

// static API_ADDRESS: &str = https://api.deepseek.com/v1
static API_ADDRESS: &str = "http://localhost:11434/v1";

pub async fn list(user_id: &str) -> Vec<ChatInfo> {
    let vec = GLOBAL_CHATS.read().unwrap();
    *vec.clone()
}

pub async fn create(user_id: &str, body: &CreateBody) -> ChatInfo {
    let uuid = Uuid::new_v4().to_string();
    let mut write_guard = GLOBAL_CHATS.write().unwrap();
    let chat_info = ChatInfo {
        id: uuid.clone(),
        name: body.name.clone(),
        created_at: Utc::now().timestamp().to_string(),
    };
    write_guard.push(chat_info.clone());
    let mut write_messages_guard = GLOBAL_CHAT_MESSAGES.write().unwrap();
    write_messages_guard.insert(uuid.clone(), Box::new(vec![]));
    chat_info
}

pub async fn delete(user_id: &str, id: &str) {
    let mut write_guard = GLOBAL_CHATS.write().unwrap();
    write_guard.dedup_by_key(|c| c.id == id);
}

pub async fn update_name(user_id: &str, body: &UpdateNameBody) {
    let mut write_guard = GLOBAL_CHATS.write().unwrap();
    // 遍历向量，找到指定 id 的 ChatInfo
    for chat in write_guard.iter_mut() {
        if chat.id == body.id {
            // 修改 name
            chat.name = body.name.clone();
        }
    }
}

pub async fn message_list(user_id: &str, id: &str) -> Vec<ChatCompletionRequestMessage> {
    get_request_history_messages(id)
}

pub async fn message_request(body: &RequestBody) -> Response {
    if !exist(&body.id) {
        return Response {
            id: body.id.clone(),
            index: 0,
            text: None,
            error: Some("Chat not found".to_string()),
        };
    }
    // write message
    {
        let mut write_guard = GLOBAL_CHAT_MESSAGES.write().unwrap();
        let vec = write_guard.entry(body.id.clone()).or_default();
        vec.push(Message {
            role: User.to_string(),
            text: body.prompt.clone(),
        })
    }
    let text = do_openai_request(&body.id, &body.model).await;
    // write response
    let mut index: usize = 0;
    {
        let mut write_guard = GLOBAL_CHAT_MESSAGES.write().unwrap();
        let vec = write_guard.entry(body.id.clone()).or_default();
        vec.push(Message {
            role: System.to_string(),
            text: text.clone(),
        });
        index = vec.len() - 1 ;
    }
    return Response {
        id:  body.id.clone(),
        index,
        text: Some(text),
        error: None,
    };
}

pub async fn message_regenerate(body: &RegenerateBody) -> Response {
    if !exist(&body.id) {
        return Response {
            id: body.id.clone(),
            index: body.index.clone(),
            text: None,
            error: Some("Chat not found".to_string()),
        };
    }
    // truncate old message
    {
        let mut write_guard = GLOBAL_CHAT_MESSAGES.write().unwrap();
        let vec = write_guard.entry(body.id.clone()).or_default();
        if body.index < vec.len() {
            vec.truncate(body.index);
        }
    }
    let text = do_openai_request(&body.id, &body.model).await;
    // write response
    {
        let mut write_guard = GLOBAL_CHAT_MESSAGES.write().unwrap();
        let vec = write_guard.entry(body.id.clone()).or_default();
        vec.push(Message {
            role: System.to_string(),
            text: text.clone(),
        })
    }
    return Response {
        id:  body.id.clone(),
        index: body.index.clone(),
        text: Some(text),
        error: None,
    };
}

pub async fn message_edit(body: &EditBody) -> Response {
    if !exist(&body.id) {
        return Response {
            id: body.id.clone(),
            index: body.index.clone(),
            text: None,
            error: Some("Chat not found".to_string()),
        };
    }
    // truncate old message and add new message
    {
        let mut write_guard = GLOBAL_CHAT_MESSAGES.write().unwrap();
        let vec = write_guard.entry(body.id.clone()).or_default();
        if body.index < vec.len() {
            vec.truncate(body.index);
        }
        vec.push(Message {
            role: User.to_string(),
            text: body.prompt.clone(),
        })
    }
    // request openai api
    let text = do_openai_request(&body.id, &body.model).await;
    // write response
    {
        let mut write_guard = GLOBAL_CHAT_MESSAGES.write().unwrap();
        let vec = write_guard.entry(body.id.clone()).or_default();
        vec.push(Message {
            role: System.to_string(),
            text: text.clone(),
        })
    }
    return Response {
        id:  body.id.clone(),
        index: body.index.clone(),
        text: Some(text),
        error: None,
    };
}

async fn do_openai_request(id: &str, model: &str) -> String {
    // new openai client
    let api_key = "sk-4d1596f494474a3ab21fc674b3cb42b7"; // This secret could be from a file, or environment variable.
    let config = OpenAIConfig::new()
        .with_api_base(API_ADDRESS)
        .with_api_key(api_key);
    let client = Client::with_config(config);

    let request = CreateChatCompletionRequestArgs::default()
        .model(model.to_string())
        .messages(get_request_history_messages(id))
        .build()
        .unwrap();
    debug!("request {:?}", request);
    let result = client.chat().create(request).await;
    if result.is_err() {
        error!("send chat request error: {}", result.err().unwrap());
        return "Sorry, System Error".to_string();
    }
    let response = result.unwrap();
    debug!("response {:?}", response);
    let mut text = String::new();
    if !response.choices.is_empty() {
        let choice = response.choices[0].clone();
        text = choice.message.content.unwrap_or_default();
    }
    text
}

fn get_history_messages(id: &str) -> Vec<Message> {
    let mut map = GLOBAL_CHAT_MESSAGES.read().unwrap();
    return match map.get(id) {
        None => vec![],
        Some(vec) => (**vec).clone(),
    };
}

fn get_request_history_messages(id: &str) -> Vec<ChatCompletionRequestMessage> {
    let history_messages = get_history_messages(id);
    let mut messages: Vec<ChatCompletionRequestMessage> = vec![];
    for message in history_messages {
        if message.role == User.to_string() {
            messages.push(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(message.text.clone())
                    .build()
                    .unwrap()
                    .into(),
            );
        } else {
            messages.push(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(message.text.clone())
                    .build()
                    .unwrap()
                    .into(),
            );
        }
    }
    messages
}

fn exist(id: &str) -> bool {
    get(id).is_some()
}

fn get(id: &str) -> Option<ChatInfo> {
    let read_guard = GLOBAL_CHATS.read().unwrap();
    let chat_infos = read_guard.as_ref();
    for chat_info in chat_infos {
        if chat_info.id == id {
            return Some(ChatInfo{
                id: id.to_string(),
                name: chat_info.name.clone(),
                created_at: chat_info.created_at.clone(),
            })
        }
    }
    None
}

fn init_if_need(id: &str, name: &str) {
    if !exist(id) {
        let mut write_guard = GLOBAL_CHATS.write().unwrap();
        write_guard.push(ChatInfo {
            created_at: Utc::now().timestamp().to_string(),
            id: id.to_string(),
            name: name.to_string(),
        });
        let mut write_messages_guard = GLOBAL_CHAT_MESSAGES.write().unwrap();
        write_messages_guard.insert(id.to_string(), Box::new(vec![]));
    }
}

pub async fn model_list() -> ModelData {
    let client = ClientBuilder::new().build().unwrap();
    let request = HttpRequestBuilder::new("GET", "http://localhost:11434/api/tags")
        .unwrap()
        .response_type(ResponseType::Json);
    if let Ok(response) = client.send(request).await {
        let data_result = response.read().await;
        if data_result.is_err() {
            error!(
                "read model data failed, err: {}",
                data_result.err().unwrap()
            );
            return ModelData { models: vec![] };
        }
        let data = data_result.unwrap().data;
        // println!("{}",data);
        let model_data: ModelData = serde_json::from_value(data).unwrap();
        return model_data;
    }
    let model_data = ModelData { models: vec![] };
    return model_data;
}

#[cfg(test)]
mod test {
    use crate::service::chat_service::{
        message_request, ChatInfo, Message, RequestBody, GLOBAL_CHATS, GLOBAL_CHAT_MESSAGES,
    };

    #[tokio::test] //由此判断这是一个测试函数
    async fn test_chat_request() {
        let result = message_request(&RequestBody {
            id: "1".to_string(),
            prompt: "who is blackstar".to_string(),
            model: "llama3:latest".to_string(),
            stream: false,
        })
        .await;
        println!("{:?}", result.text);
        // request twice
        let result = message_request(&RequestBody {
            id: "1".to_string(),
            prompt: "oh, no".to_string(),
            model: "llama3:latest".to_string(),
            stream: false,
        })
        .await;
        println!("{:?}", result.text);
    }
    #[tokio::test]
    async fn test_write_global() {
        {
            let mut read_guard = GLOBAL_CHAT_MESSAGES.read().unwrap();
            println!("{:?}", read_guard.len());
        }

        let mut map = GLOBAL_CHAT_MESSAGES.write().unwrap();
        map.insert(
            "2".to_string(),
            Box::new(vec![Message {
                role: "hello".to_string(),
                text: "world".to_string(),
            }]),
        );

        let result = map.get("2").unwrap();
        assert_eq!(result[0].role, "hello");
        assert_eq!(result[0].text, "world");
    }

    #[tokio::test]
    async fn test_chats() {
        let mut vec = GLOBAL_CHATS.write().unwrap();
        vec.push(ChatInfo {
            id: "3".to_string(),
            name: "hello,world".to_string(),
        });

        assert_eq!(vec[0].id, "3");
        assert_eq!(vec[0].name, "hello,world");
    }
}
