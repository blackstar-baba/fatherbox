use std::cell::OnceCell;
use std::collections::HashMap;
use std::future::Future;
use std::sync::{Mutex, RwLock};
use std::vec;

use anyhow::Context;
use futures::future::ok;
use futures::StreamExt;
use log::{debug, error, info};
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Error};
use tauri::api::http::{Body, ClientBuilder, HttpRequestBuilder, ResponseType};
use tauri::Window;

use app::{AppResponse, RESPONSE_CODE_SUCCESS};

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
pub struct DeepChatRequestBody {
    id: String,
    messages: Vec<DeepChatRequestMessage>,
    model: String,
    stream: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeepChatTextResponse {
    text: String,
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

#[tauri::command]
pub async fn get_chats_cmd() -> Result<Vec<ChatInfo>, ()> {
    // todo write chat info to database
    let vec = GLOBAL_CHATS.read().unwrap();
    Ok(*vec.clone())
}

#[tauri::command]
pub async fn get_chat_history_messages_cmd(id: String) -> Result<Vec<DeepChatRequestMessage>, ()> {
    // todo write chat messages to database
    let mut map = GLOBAL_CHAT_MESSAGES.read().unwrap();
    return match map.get(&id) {
        None => Ok(vec![]),
        Some(messages) => Ok((**messages).clone()),
    };
}

#[tauri::command]
pub async fn chat_request_cmd(body: DeepChatRequestBody) -> Result<DeepChatTextResponse, ()> {
    let history_messages = get_history_message(&body.id, &body.messages[0].text).unwrap();
    // write message
    {
        let mut write_guard = GLOBAL_CHAT_MESSAGES.write().unwrap();
        let vec = write_guard.entry(body.id.clone()).or_default();
        for message in &body.messages {
            vec.push(DeepChatRequestMessage {
                role: message.role.clone(),
                text: message.text.clone(),
            })
        }
    }
    let mut messages: Vec<OllamaMessage> = vec![];
    for message in history_messages {
        messages.push(OllamaMessage {
            role: message.role,
            content: message.text,
        });
    }
    for message in body.messages {
        messages.push(OllamaMessage {
            role: message.role,
            content: message.text,
        })
    }
    let request = OllamaRequest {
        model: body.model,
        messages,
        stream: false,
    };
    let client = ClientBuilder::new().build().unwrap();
    let request = HttpRequestBuilder::new("POST", "http://localhost:11434/api/chat")
        .unwrap()
        .response_type(ResponseType::Json)
        .body(Body::Json(serde_json::to_value(request).unwrap()));
    debug!("request {:?}", request);
    match client.send(request).await {
        Ok(response) => {
            debug!("response {:?}", response);
            match response.read().await {
                Ok(response_data) => {
                    match serde_json::from_value::<OllamaResponse>(response_data.data) {
                        Ok(ollama_response) => {
                            // todo change frontend
                            let ollama_message = ollama_response.message;
                            {
                                let mut write_guard = GLOBAL_CHAT_MESSAGES.write().unwrap();
                                let vec = write_guard.entry(body.id.clone()).or_default();
                                vec.push(DeepChatRequestMessage {
                                    role: ollama_message.role.clone(),
                                    text: ollama_message.content.to_string(),
                                })
                            }
                            return Ok(DeepChatTextResponse {
                                text: ollama_message.content,
                            });
                        }
                        Err(err) => {
                            error!("send chat request error 1 {}", err)
                        }
                    }
                }
                Err(err) => {
                    error!("send chat request error 2 {}", err)
                }
            }
        }
        Err(err) => {
            error!("send chat request error 3 {}", err)
        }
    }
    return Ok(DeepChatTextResponse {
        text: "Sorry, System Error".to_string(),
    });
}

fn get_history_message(id: &str, name: &str) -> Result<Vec<DeepChatRequestMessage>, ()> {
    let exist = {
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
    };
    if !exist {
        let mut write_guard = GLOBAL_CHATS.write().unwrap();
        write_guard.push(ChatInfo {
            id: id.to_string(),
            name: name.to_string(),
        });
        let mut write_messages_guard = GLOBAL_CHAT_MESSAGES.write().unwrap();
        write_messages_guard.insert(id.to_string(), Box::new(vec![]));
    }
    let mut map = GLOBAL_CHAT_MESSAGES.read().unwrap();
    return match map.get(id) {
        None => Ok(vec![]),
        Some(vec) => Ok((**vec).clone()),
    };
}

#[tauri::command]
pub async fn chat_stream_request_cmd(window: Window, request_body: DeepChatRequestBody) {
    // todo
    let client = Client::new();
    let response = client
        .get("http://example.com/stream")
        .send()
        .await
        .unwrap();

    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        match item {
            Ok(chunk) => {
                // 将每个数据块发送到前端
                window.emit("stream-data", chunk.to_vec()).unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}

#[tauri::command]
pub fn chat_files_request_cmd(
    files: Vec<String>,
    form_data: std::collections::HashMap<String, String>,
) -> Result<DeepChatTextResponse, ()> {
    // todo
    return Ok(DeepChatTextResponse {
        text: "".to_string(),
    });
}

#[tauri::command]
pub async fn get_models_cmd() -> Result<ModelData, ()> {
    let client = ClientBuilder::new().build().unwrap();
    let request = HttpRequestBuilder::new("GET", "http://localhost:11434/api/tags")
        .unwrap()
        .response_type(ResponseType::Json);
    if let Ok(response) = client.send(request).await {
        let data = response.read().await.unwrap().data;
        // println!("{}",data);
        let model_data: ModelData = serde_json::from_value(data).unwrap();
        return Ok(model_data);
    }
    let model_data = ModelData { models: vec![] };
    return Ok(model_data);
}

#[cfg(test)]
mod test {
    use crate::model_command::{
        chat_request_cmd, ChatInfo, DeepChatRequestBody, DeepChatRequestMessage, GLOBAL_CHATS,
        GLOBAL_CHAT_MESSAGES,
    };

    #[tokio::test] //由此判断这是一个测试函数
    async fn test_chat_request() {
        let result = chat_request_cmd(DeepChatRequestBody {
            id: "1".to_string(),
            messages: vec![DeepChatRequestMessage {
                role: "user".to_string(),
                text: "who is blackstar".to_string(),
            }],
            model: "llama3:latest".to_string(),
            stream: false,
        })
        .await;
        assert_eq!(result.is_err(), false);
        println!("{:?}", result.unwrap().text);
        // request twice
        let result = chat_request_cmd(DeepChatRequestBody {
            id: "1".to_string(),
            messages: vec![DeepChatRequestMessage {
                role: "user".to_string(),
                text: "oh, no".to_string(),
            }],
            model: "llama3:latest".to_string(),
            stream: false,
        })
        .await;
        assert_eq!(result.is_err(), false);
        println!("{:?}", result.unwrap().text);
    }
    #[tokio::test] //由此判断这是一个测试函数
    async fn test_write_global() {
        {
            let mut read_guard = GLOBAL_CHAT_MESSAGES.read().unwrap();
            println!("{:?}", read_guard.len());
        }

        let mut map = GLOBAL_CHAT_MESSAGES.write().unwrap();
        map.insert(
            "1".to_string(),
            Box::new(vec![DeepChatRequestMessage {
                role: "hello".to_string(),
                text: "world".to_string(),
            }]),
        );

        let result = map.get("1").unwrap();
        assert_eq!(result[0].role, "hello");
        assert_eq!(result[0].text, "world");
    }

    #[tokio::test] //由此判断这是一个测试函数
    async fn test_chats() {
        let mut vec = GLOBAL_CHATS.write().unwrap();
        vec.push(ChatInfo {
            id: "1".to_string(),
            name: "hello,world".to_string(),
        });

        assert_eq!(vec[0].id, "1");
        assert_eq!(vec[0].name, "hello,world");
    }
}
