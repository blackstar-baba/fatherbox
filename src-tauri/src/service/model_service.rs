use std::cell::OnceCell;
use std::collections::HashMap;
use std::future::Future;
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
pub struct Model {
    name: String,
    model: String,
    modified_at: String,
    size: i64,
    digest: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeepChatRequestMessage {
    role: String,
    text: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ChatRequestBody {
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeepChatRequestBody {
    id: String,
    messages: Vec<DeepChatRequestMessage>,
    model: String,
    stream: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeepChatTextResponse {
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    id: String,
    prompt: String,
    model: String,
    stream: bool,
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
    text: String,
    conversation_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OllamaResponse {
    model: String,
    created_at: String,
    message: OllamaMessage,
    done: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatInfo {
    id: String,
    name: String,
}

static GLOBAL_CHATS: Lazy<RwLock<Box<Vec<ChatInfo>>>> = Lazy::new(|| {
    let m = Box::new(vec![]);
    RwLock::new(m)
});

static GLOBAL_CHAT_MESSAGES: Lazy<RwLock<HashMap<String, Box<Vec<DeepChatRequestMessage>>>>> =
    Lazy::new(|| {
        let m = HashMap::new();
        RwLock::new(m)
    });

// static API_ADDRESS: &str = https://api.deepseek.com/v1
static API_ADDRESS: &str = "http://localhost:11434/v1";

pub async fn get_chats() -> Vec<ChatInfo> {
    // todo write chat info to database
    let vec = GLOBAL_CHATS.read().unwrap();
    *vec.clone()
}

pub async fn get_chat_history_messages(id: String) -> Vec<DeepChatRequestMessage> {
    // todo write chat messages to database
    let mut map = GLOBAL_CHAT_MESSAGES.read().unwrap();
    return match map.get(&id) {
        None => vec![],
        Some(messages) => (**messages).clone(),
    };
}

pub async fn chat_request(body: RequestBody) -> Response {
    init_if_need(&body.id, &body.prompt);
    // write message
    {
        let mut write_guard = GLOBAL_CHAT_MESSAGES.write().unwrap();
        let vec = write_guard.entry(body.id.clone()).or_default();
        vec.push(DeepChatRequestMessage {
            role: User.to_string(),
            text: body.prompt.clone(),
        })
    }
    let text = do_request(&body.id, &body.model).await;
    // write response
    {
        let mut write_guard = GLOBAL_CHAT_MESSAGES.write().unwrap();
        let vec = write_guard.entry(body.id.clone()).or_default();
        vec.push(DeepChatRequestMessage {
            role: System.to_string(),
            text: text.clone(),
        })
    }
    return Response {
        id: Uuid::new_v4().to_string(),
        text,
        conversation_id: body.id.clone(),
    };
}

pub async fn chat_message_regenerate(body: RegenerateBody) -> Response {
    if !exist(&body.id) {
        return Response {
            id: Uuid::new_v4().to_string(),
            text: "Sorry, System Error".to_string(),
            conversation_id: body.id.clone(),
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
    let text = do_request(&body.id, &body.model).await;
    // write response
    {
        let mut write_guard = GLOBAL_CHAT_MESSAGES.write().unwrap();
        let vec = write_guard.entry(body.id.clone()).or_default();
        vec.push(DeepChatRequestMessage {
            role: System.to_string(),
            text: text.clone(),
        })
    }
    return Response {
        id: Uuid::new_v4().to_string(),
        text,
        conversation_id: body.id.clone(),
    };
}

pub async fn chat_message_edit(body: EditBody) -> Response {
    if !exist(&body.id) {
        return Response {
            id: Uuid::new_v4().to_string(),
            text: "Sorry, System Error".to_string(),
            conversation_id: body.id.clone(),
        };
    }
    // truncate old message and add new message
    {
        let mut write_guard = GLOBAL_CHAT_MESSAGES.write().unwrap();
        let vec = write_guard.entry(body.id.clone()).or_default();
        if body.index < vec.len() {
            vec.truncate(body.index);
        }
        vec.push(DeepChatRequestMessage {
            role: User.to_string(),
            text: body.prompt.clone(),
        })
    }
    // request openai api
    let text = do_request(&body.id, &body.model).await;
    // write response
    {
        let mut write_guard = GLOBAL_CHAT_MESSAGES.write().unwrap();
        let vec = write_guard.entry(body.id.clone()).or_default();
        vec.push(DeepChatRequestMessage {
            role: System.to_string(),
            text: text.clone(),
        })
    }
    return Response {
        id: Uuid::new_v4().to_string(),
        text,
        conversation_id: body.id.clone(),
    };
}

async fn do_request(id: &str, model: &str) -> String {
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
        return "Sorry, System Error".to_string()
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

fn get_history_messages(id: &str) -> Vec<DeepChatRequestMessage> {
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
    let read_guard = GLOBAL_CHATS.read().unwrap();
    let mut exist = false;
    let chat_infos = read_guard.as_ref();
    for chat_info in chat_infos {
        if chat_info.id == id {
            exist = true;
            break;
        }
    }
    exist
}

fn init_if_need(id: &str, name: &str) {
    if !exist(id) {
        let mut write_guard = GLOBAL_CHATS.write().unwrap();
        write_guard.push(ChatInfo {
            id: id.to_string(),
            name: name.to_string(),
        });
        let mut write_messages_guard = GLOBAL_CHAT_MESSAGES.write().unwrap();
        write_messages_guard.insert(id.to_string(), Box::new(vec![]));
    }
}

pub fn chat_files_request(
    files: Vec<String>,
    form_data: HashMap<String, String>,
) -> DeepChatTextResponse {
    // todo
    DeepChatTextResponse {
        text: "".to_string(),
    }
}

pub async fn get_models() -> ModelData {
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
    use crate::service::model_service::{
        chat_request, ChatInfo, DeepChatRequestMessage, RequestBody, GLOBAL_CHATS,
        GLOBAL_CHAT_MESSAGES,
    };

    #[tokio::test] //由此判断这是一个测试函数
    async fn test_chat_request() {
        let result = chat_request(RequestBody {
            id: "1".to_string(),
            prompt: "who is blackstar".to_string(),
            model: "llama3:latest".to_string(),
            stream: false,
        })
        .await;
        println!("{:?}", result.text);
        // request twice
        let result = chat_request(RequestBody {
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
            Box::new(vec![DeepChatRequestMessage {
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
