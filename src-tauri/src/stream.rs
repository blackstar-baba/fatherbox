use std::path::PathBuf;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use log::{debug, error, info, trace};
use sea_orm::DatabaseConnection;
use serde_json::{to_value, Value};
use tauri::{State, Window};
use bytes::Bytes;

use app::{AppResponse, AppState, LoginInfo};


use std::time::Duration;
use std::error::Error;
use std::sync::atomic::{AtomicU32, Ordering};
use std::collections::HashMap;
use std::fs;
use async_openai::config::OpenAIConfig;
use async_openai::types::{ChatCompletionRequestUserMessageArgs, ChatCompletionResponseStream, CreateChatCompletionRequestArgs};
use async_openai::types::Role::User;
use futures::StreamExt;
use reqwest::Client;
use reqwest::header::{HeaderName, HeaderMap};
use serde::{Deserialize, Serialize};
use app::service::ai_chat_service::Message;
use app::service::user_service::LoginBody;

static REQUEST_COUNTER: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Clone, serde::Serialize)]
pub struct StreamResponse {
    request_id: u32,
    status: u16,
    status_text: String,
    headers: HashMap<String, String>
}

#[derive(Clone, serde::Serialize)]
pub struct EndPayload {
    request_id: u32,
    status: u16,
}

#[derive(Clone, serde::Serialize)]
pub struct ChunkPayload {
    request_id: u32,
    chunk: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    pub id: String,
    pub prompt: String,
    pub model_id: String,
    pub source_id: String,
    pub stream: bool,
}

#[tauri::command]
pub async fn stream_cmd(
    window: Window,
    state: State<'_, AppState>,
    request_id: String,
    command: String,
    access_token: Option<String>,
    args: Value,
) -> Result<Value, ()> {


    let body: RequestBody = serde_json::from_value(args).unwrap();

    // todo:  use body

    let event_name = &request_id;

    let url = "http://localhost:11434/v1";
    let key = "sn-123456";
    let model = "llama3.1:8b";

    let config = OpenAIConfig::new()
        .with_api_base(url)
        .with_api_key(key);
    let client = async_openai::Client::with_config(config);
    // todo convert message to chat message
    let request_messages = vec![ChatCompletionRequestUserMessageArgs::default()
        .content("Write a marketing blog praising and introducing Rust library async-openai")
        .build().unwrap().into()];

    let request = CreateChatCompletionRequestArgs::default()
        .model(model.to_string())
        .messages(request_messages)
        .build()
        .unwrap();
    debug!("request {:?}", request);
    match client.chat().create_stream(request).await {
        Ok(mut stream) => {
            while let Some(result) = stream.next().await {
                match result {
                    Ok(response) => {
                        response.choices.iter().for_each(|chat_choice| {
                            if let Some(ref content) = chat_choice.delta.content {
                                info!("stream body: {:?}", content);
                                window.emit(event_name, ChunkPayload{
                                    request_id: 0,
                                    chunk: content.clone()
                                }).unwrap();
                            }
                        });
                    }
                    Err(err) => {
                        error!("stream err: {:?}", err);
                        window.emit(event_name, EndPayload{
                            request_id: 0,
                            status: 2,
                        }).unwrap();
                    }
                }
            }
            window.emit(event_name, EndPayload{
                request_id: 0,
                status: 1,
            }).unwrap();
        }
        Err(err) => {
            error!( "{:?}",err)
        }
    }

    Ok(Value::Null)
}
