use std::future::Future;
use anyhow::Context;
use futures::StreamExt;
use log::{debug, error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Error, json};
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
    prompt: String,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OllamaResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
}

#[tauri::command]
pub async fn chat_request_cmd(body: DeepChatRequestBody) -> Result<DeepChatTextResponse, ()> {
    let client = ClientBuilder::new().build().unwrap();
    let request = OllamaRequest {
        model: "llama3:latest".to_string(),
        prompt: body.messages[0].clone().text,
        stream: false,
    };
    let request = HttpRequestBuilder::new("POST", "http://localhost:11434/api/generate")
        .unwrap()
        .response_type(ResponseType::Json)
        .body(Body::Json(serde_json::to_value(request).unwrap()));

    debug!("request {:?}",request);
    match client.send(request).await {
        // todo error
        Ok(response) => {
            debug!("response {:?}",response);
            match response.read().await {
                Ok(response_data) => {
                    match serde_json::from_value::<OllamaResponse>(response_data.data) {
                        Ok(ollama_response) => {
                            return Ok(DeepChatTextResponse {
                                text: ollama_response.response,
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
    use crate::model_command::{chat_request_cmd, DeepChatRequestBody, DeepChatRequestMessage};

    #[tokio::test] //由此判断这是一个测试函数
    async fn test_chat_request() {
        // todo
        let result = chat_request_cmd(DeepChatRequestBody {
            messages: vec![DeepChatRequestMessage {
                role: "user".to_string(),
                text: "who is blackstar".to_string(),
            }],
            model: "ollama:latest".to_string(),
            stream: false,
        }).await;
        assert_eq!(result.is_err(),false);
        println!("{:?}",result.unwrap().text)
    }
}
