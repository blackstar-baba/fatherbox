use std::cell::OnceCell;
use std::collections::HashMap;
use std::future::Future;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::{Mutex, RwLock};
use std::{fs, vec};

use crate::entity::file::{
    ActiveModel as FileActiveModel, Column, Entity as File, Model as FileModel,
};
use anyhow::Context;
use async_openai::config::OpenAIConfig;
use async_openai::types::Role::{System, User};
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessage,
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};
use async_openai::Client;
use chrono::Utc;
use futures::future::ok;
use futures::StreamExt;
use log::{debug, error, info, trace};
use once_cell::sync::Lazy;
use sea_orm::{DatabaseConnection, Set};
use serde::{Deserialize, Serialize};
use serde_json::{json, Error};
use tauri::api::http::{Body, ClientBuilder, HttpRequestBuilder, ResponseType};
use tauri::{App, Window};
use uuid::Uuid;

use crate::dao::file_dao::FileService;
use crate::dto::file::ListGeneralBody;
use crate::entity::file::ActiveModel;
use crate::service::ai_model_service::get as get_ai_model;
use crate::service::ai_source_service::get as get_ai_source;
use crate::{AppResponse, CHAT_ZONE, FILE_TYPE};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelData {
    pub models: Vec<Model>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatInfo {
    pub id: String,
    pub name: String,
    pub create_time: i64,
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
    content: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ListBody {
    pub wid: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateBody {
    name: String,
    wid: String,
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
    pub model_id: String,
    pub source_id: String,
    pub request_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RegenerateBody {
    id: String,
    index: usize,
    model_id: String,
    source_id: String,
    pub request_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EditBody {
    id: String,
    index: usize,
    prompt: String,
    model_id: String,
    source_id: String,
    pub request_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    id: String, // chat id
    text: Option<String>,
    index: usize,
    error: Option<String>,
}

// static API_ADDRESS: &str = https://api.deepseek.com/v1
static API_ADDRESS: &str = "http://localhost:11434/v1";

pub async fn list(
    db: &DatabaseConnection,
    user_id: &str,
    wid: &str,
) -> AppResponse<Option<Vec<ChatInfo>>> {
    match FileService::list_files(
        db,
        &ListGeneralBody {
            wid: wid.to_string(),
            zone: CHAT_ZONE.to_string(),
            r#type: Some(FILE_TYPE.to_string()),
        },
    )
    .await
    {
        Ok(result) => {
            let mut chat_infos = vec![];
            for model in result {
                chat_infos.push(ChatInfo {
                    id: model.id,
                    name: model.name,
                    create_time: model.create_time,
                })
            }
            return AppResponse::success(Some(chat_infos));
        }
        Err(err) => AppResponse::error(None, &err.to_string()),
    }
}

pub async fn create(
    db: &DatabaseConnection,
    user_id: &str,
    body: &CreateBody,
) -> AppResponse<Option<ChatInfo>> {
    match FileService::create_file(
        db,
        ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            name: Set(body.name.clone()),
            r#type: Set(FILE_TYPE.to_string()),
            pid: Set(body.wid.to_string()),
            wid: Set(body.wid.to_string()),
            zone: Set(CHAT_ZONE.to_string()),
            size: Set(0),
            create_time: Set(Utc::now().timestamp()),
            update_time: Set(Utc::now().timestamp()),
            state: Set(1),
            ..Default::default()
        },
    )
    .await
    {
        Ok(model) => AppResponse::success(Some(ChatInfo {
            id: model.id,
            name: model.name,
            create_time: model.create_time,
        })),
        Err(err) => AppResponse::error(None::<ChatInfo>, &err.to_string()),
    }
}

pub async fn delete(db: &DatabaseConnection, user_id: &str, id: &str) -> AppResponse<String> {
    match FileService::delete_file(db, &id).await {
        Ok(_) => AppResponse::success("".to_string()),
        Err(err) => AppResponse::error("".to_string(), &err.to_string()),
    }
}

pub async fn update_name(
    db: &DatabaseConnection,
    user_id: &str,
    body: &UpdateNameBody,
) -> AppResponse<Option<ChatInfo>> {
    let get_result = FileService::get_file(db, &body.id).await;
    if get_result.is_err() {
        return AppResponse::error(None, &get_result.err().unwrap().to_string());
    }
    let option_model = get_result.unwrap();
    if option_model.is_none() {
        return AppResponse::error(None, "chat not found");
    }
    let model = option_model.unwrap();
    match FileService::update_file_name(db, &body.id, &body.name).await {
        Ok(_) => AppResponse::success(Some(ChatInfo {
            id: model.id,
            name: model.name,
            create_time: model.create_time,
        })),
        Err(err) => AppResponse::error(None, &err.to_string()),
    }
}

pub async fn message_list(
    db: &DatabaseConnection,
    user_path: &PathBuf,
    user_id: &str,
    id: &str,
) -> AppResponse<Option<Vec<Message>>> {
    let get_result = FileService::get_file(db, id).await;
    if get_result.is_err() {
        return AppResponse::error(None, &get_result.err().unwrap().to_string());
    }
    let option_model = get_result.unwrap();
    if option_model.is_none() {
        return AppResponse::error(None, "chat not found in db");
    }
    let mut model = option_model.unwrap();
    let file_path = &user_path.join(&model.wid).join(&model.id);
    if !file_path.exists() {
        return AppResponse::error(None, "chat not found in file system");
    }
    // todo catch error
    // 读取文件内容到字符串
    let file_content = fs::read_to_string(file_path).unwrap();
    // 将 JSON 字符串反序列化为结构体
    let data: Vec<Message> = serde_json::from_str(&file_content).unwrap();
    AppResponse::success(Some(data))
}

async fn get_chat(db: &DatabaseConnection, id: &str) -> AppResponse<Option<FileModel>> {
    let get_result = FileService::get_file(db, id).await;
    if get_result.is_err() {
        return AppResponse::error(None, &get_result.err().unwrap().to_string());
    }
    let option_model = get_result.unwrap();
    if option_model.is_none() {
        return AppResponse::error(None, "chat not found in db");
    }
    AppResponse::success(Some(option_model.unwrap()))
}

pub async fn message_request(
    db: &DatabaseConnection,
    user_path: &PathBuf,
    user_id: &str,
    body: &RequestBody,
) -> AppResponse<Option<Response>> {
    let app_response = get_chat(db, &body.id).await;
    if app_response.is_error() {
        return AppResponse::error(None, &app_response.message);
    }
    let model = app_response.result.unwrap();
    let app_response = get_ai_source(db, &body.source_id).await;
    if app_response.is_error() {
        return AppResponse::error(None, &app_response.message);
    }
    let ai_source = app_response.result.unwrap();
    let app_response = get_ai_model(db, &body.model_id).await;
    if app_response.is_error() {
        return AppResponse::error(None, &app_response.message);
    }
    let ai_model = app_response.result.unwrap();
    let file_path = &user_path.join(&model.wid).join(&model.id);
    if !file_path.exists() {
        let result = fs::File::create(file_path);
        if result.is_err() {
            error!(
                "create chat file on disk failed, err: {}",
                result.err().unwrap()
            );
            return AppResponse::error(
                None,
                "create chat file on disk failed, please check your disk space and permissions",
            );
        }
    }
    // todo catch error
    // 读取文件内容到字符串
    let file_content = fs::read_to_string(file_path).unwrap();
    // 将 JSON 字符串反序列化为结构体
    let mut messages = Vec::new();
    if !file_content.is_empty() {
        messages = serde_json::from_str(&file_content).unwrap();
    }
    // add user message
    messages.push(Message {
        role: User.to_string(),
        content: body.prompt.clone(),
    });
    // add system message
    let text = do_openai_request(&messages, &ai_source.url, &ai_source.key, &ai_model.name).await;
    messages.push(Message {
        role: System.to_string(),
        content: text.clone(),
    });
    // save to file
    let json_str = serde_json::to_string(&messages).unwrap();
    fs::write(file_path, json_str).unwrap();
    // return
    AppResponse::success(Some(Response {
        id: body.id.clone(),
        index: messages.len() - 1,
        text: Some(text),
        error: None,
    }))
}

pub async fn message_request_stream<F>(
    callback: F,
    db: &DatabaseConnection,
    user_path: &PathBuf,
    user_id: &str,
    body: &RequestBody,
) -> AppResponse<Option<Response>>
where
    F: Fn(Option<String>, i8),
{
    let app_response = get_chat(db, &body.id).await;
    if app_response.is_error() {
        return AppResponse::error(None, &app_response.message);
    }
    let model = app_response.result.unwrap();
    let app_response = get_ai_source(db, &body.source_id).await;
    if app_response.is_error() {
        return AppResponse::error(None, &app_response.message);
    }
    let ai_source = app_response.result.unwrap();
    let app_response = get_ai_model(db, &body.model_id).await;
    if app_response.is_error() {
        return AppResponse::error(None, &app_response.message);
    }
    let ai_model = app_response.result.unwrap();
    let file_path = &user_path.join(&model.wid).join(&model.id);
    if !file_path.exists() {
        let result = fs::File::create(file_path);
        if result.is_err() {
            error!(
                "create chat file on disk failed, err: {}",
                result.err().unwrap()
            );
            return AppResponse::error(
                None,
                "create chat file on disk failed, please check your disk space and permissions",
            );
        }
    }
    // todo catch error
    // 读取文件内容到字符串
    let file_content = fs::read_to_string(file_path).unwrap();
    // 将 JSON 字符串反序列化为结构体
    let mut messages = Vec::new();
    if !file_content.is_empty() {
        messages = serde_json::from_str(&file_content).unwrap();
    }
    // add user message
    messages.push(Message {
        role: User.to_string(),
        content: body.prompt.clone(),
    });
    messages.push(Message {
        role: System.to_string(),
        content: String::new(),
    });
    let callback_wrapper = |content: Option<String>, status: i8| {
        let mut messages = messages.clone();
        let len = messages.len() - 1;
        match messages.get_mut(len) {
            None => {}
            Some(message) => {
                if content.is_some() {
                    message.content = format!("{}{}", &message.content, content.clone().unwrap());
                } else {
                    let json_str = serde_json::to_string(&messages).unwrap();
                    fs::write(file_path, json_str).unwrap();
                }
                // invoke callback
                callback(content, status);
            }
        }
    };
    // add system message
    do_openai_request_stream(
        callback_wrapper,
        &messages,
        &ai_source.url,
        &ai_source.key,
        &ai_model.name,
    )
    .await;
    // return
    AppResponse::success(Some(Response {
        id: body.id.clone(),
        index: messages.len() - 1,
        text: Some(String::new()),
        error: None,
    }))
}

pub async fn message_regenerate<F>(
    callback: F,
    db: &DatabaseConnection,
    user_path: &PathBuf,
    user_id: &str,
    body: &RegenerateBody,
) -> AppResponse<Option<Response>>
where
    F: Fn(Option<String>, i8),
{
    let app_response = get_chat(db, &body.id).await;
    if app_response.is_error() {
        return AppResponse::error(None, &app_response.message);
    }
    let model = app_response.result.unwrap();
    let app_response = get_ai_source(db, &body.source_id).await;
    if app_response.is_error() {
        return AppResponse::error(None, &app_response.message);
    }
    let ai_source = app_response.result.unwrap();
    let app_response = get_ai_model(db, &body.model_id).await;
    if app_response.is_error() {
        return AppResponse::error(None, &app_response.message);
    }
    let ai_model = app_response.result.unwrap();
    let file_path = &user_path.join(&model.wid).join(&model.id);
    if !file_path.exists() {
        return AppResponse::error(None, "chat not found in file system");
    }
    // todo catch error
    // 读取文件内容到字符串
    let file_content = fs::read_to_string(file_path).unwrap();
    // 将 JSON 字符串反序列化为结构体
    let mut messages: Vec<Message> = serde_json::from_str(&file_content).unwrap();
    if body.index < messages.len() {
        messages.truncate(body.index);
    }
    // add system message
    messages.push(Message {
        role: System.to_string(),
        content: String::new(),
    });
    let callback_wrapper = |content: Option<String>, status: i8| {
        let mut messages = messages.clone();
        let len = messages.len() - 1;
        match messages.get_mut(len) {
            None => {}
            Some(message) => {
                if content.is_some() {
                    message.content = format!("{}{}", &message.content, content.clone().unwrap());
                } else {
                    let json_str = serde_json::to_string(&messages).unwrap();
                    fs::write(file_path, json_str).unwrap();
                }
                // invoke callback
                callback(content, status);
            }
        }
    };
    // add system message
    do_openai_request_stream(
        callback_wrapper,
        &messages,
        &ai_source.url,
        &ai_source.key,
        &ai_model.name,
    )
    .await;
    // return
    AppResponse::success(Some(Response {
        id: body.id.clone(),
        index: body.index.clone(),
        text: Some(String::new()),
        error: None,
    }))
}

pub async fn message_edit<F>(
    callback: F,
    db: &DatabaseConnection,
    user_path: &PathBuf,
    user_id: &str,
    body: &EditBody,
) -> AppResponse<Option<Response>>
where
    F: Fn(Option<String>, i8),
{
    let app_response = get_chat(db, &body.id).await;
    if app_response.is_error() {
        return AppResponse::error(None, &app_response.message);
    }
    let model = app_response.result.unwrap();
    let app_response = get_ai_source(db, &body.source_id).await;
    if app_response.is_error() {
        return AppResponse::error(None, &app_response.message);
    }
    let ai_source = app_response.result.unwrap();
    let app_response = get_ai_model(db, &body.model_id).await;
    if app_response.is_error() {
        return AppResponse::error(None, &app_response.message);
    }
    let ai_model = app_response.result.unwrap();
    let file_path = &user_path.join(&model.wid).join(&model.id);
    if !file_path.exists() {
        return AppResponse::error(None, "chat not found in file system");
    }
    // todo catch error
    // 读取文件内容到字符串
    let file_content = fs::read_to_string(file_path).unwrap();
    // 将 JSON 字符串反序列化为结构体
    let mut messages: Vec<Message> = serde_json::from_str(&file_content).unwrap();
    if body.index < messages.len() {
        messages.truncate(body.index);
    }
    // add user message
    messages.push(Message {
        role: User.to_string(),
        content: body.prompt.clone(),
    });
    // add system message
    messages.push(Message {
        role: System.to_string(),
        content: String::new(),
    });
    let callback_wrapper = |content: Option<String>, status: i8| {
        let mut messages = messages.clone();
        let len = messages.len() - 1;
        match messages.get_mut(len) {
            None => {}
            Some(message) => {
                if content.is_some() {
                    message.content = format!("{}{}", &message.content, content.clone().unwrap());
                } else {
                    let json_str = serde_json::to_string(&messages).unwrap();
                    fs::write(file_path, json_str).unwrap();
                }
                // invoke callback
                callback(content, status);
            }
        }
    };
    // add system message
    do_openai_request_stream(
        callback_wrapper,
        &messages,
        &ai_source.url,
        &ai_source.key,
        &ai_model.name,
    )
    .await;
    // return
    AppResponse::success(Some(Response {
        id: body.id.clone(),
        index: body.index.clone(),
        text: Some(String::new()),
        error: None,
    }))
}

async fn do_openai_request_stream<F>(
    mut callback: F,
    messages: &Vec<Message>,
    url: &str,
    key: &str,
    model: &str,
) where
    F: FnMut(Option<String>, i8),
{
    // new openai client// This secret could be from a file, or environment variable.
    let config = OpenAIConfig::new().with_api_base(url).with_api_key(key);
    let client = Client::with_config(config);
    // todo convert message to chat message
    let mut request_messages = vec![];
    for msg in messages.iter() {
        request_messages.push(
            ChatCompletionRequestUserMessageArgs::default()
                .name(msg.role.clone())
                .content(msg.content.clone())
                .build()
                .unwrap()
                .into(),
        )
    }
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
                                debug!("stream body: {:?}", content);
                                callback(Some(content.to_string()), 0);
                            }
                        });
                    }
                    Err(err) => {
                        error!("stream err: {:?}", err);
                        callback(None, -1);
                    }
                }
            }
            callback(None, 1)
        }
        Err(err) => {
            error!("stream err: {:?}", err);
            callback(None, 1)
        }
    }
}

async fn do_openai_request(messages: &Vec<Message>, url: &str, key: &str, model: &str) -> String {
    // new openai client// This secret could be from a file, or environment variable.
    let config = OpenAIConfig::new().with_api_base(url).with_api_key(key);
    let client = Client::with_config(config);
    // todo convert message to chat message
    let mut request_messages = vec![];
    for msg in messages.iter() {
        request_messages.push(
            ChatCompletionRequestUserMessageArgs::default()
                .name(msg.role.clone())
                .content(msg.content.clone())
                .build()
                .unwrap()
                .into(),
        )
    }
    let request = CreateChatCompletionRequestArgs::default()
        .model(model.to_string())
        .messages(request_messages)
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

pub async fn model_list() -> AppResponse<Option<ModelData>> {
    let client = ClientBuilder::new().build().unwrap();
    let request = HttpRequestBuilder::new("GET", "http://localhost:11434/api/tags")
        .unwrap()
        .response_type(ResponseType::Json);
    let result = client.send(request).await;
    if result.is_err() {
        return AppResponse::error(None, &result.err().unwrap().to_string());
    }
    let data_result = result.unwrap().read().await;
    if data_result.is_err() {
        let err = data_result.err().unwrap();
        error!("read model data failed, err: {:?}", err);
        return AppResponse::error(None, &err.to_string());
    }
    let data = data_result.unwrap().data;
    // println!("{}",data);
    let model_data: ModelData = serde_json::from_value(data).unwrap();
    AppResponse::success(Some(model_data))
}

#[cfg(test)]
mod test {
    use crate::entity;
    use crate::service::ai_chat_service::{create, message_request, CreateBody, RequestBody};
    use crate::util::db_util::{drop_database_file, exist_database_file, init_connection};
    use sea_orm::{ConnectionTrait, Schema};
    use std::env::temp_dir;
    use std::fs;
    use uuid::Uuid;

    #[tokio::test] //由此判断这是一个测试函数
    async fn test_chat_request() {
        let workspace = "test";
        let temp_dir = temp_dir();
        let base_path = &temp_dir.join(".fatherbox");
        let user_path = base_path;
        let file_path = &base_path.join("test-chat.sqlite");
        if exist_database_file(file_path) {
            drop_database_file(&file_path).unwrap();
        }
        let db = &init_connection(&file_path).await.unwrap();
        let builder = db.get_database_backend();
        let schema = Schema::new(builder);
        db.execute(builder.build(&schema.create_table_from_entity(entity::prelude::File)))
            .await
            .unwrap();
        db.execute(builder.build(&schema.create_table_from_entity(entity::prelude::AiSource)))
            .await
            .unwrap();
        db.execute(builder.build(&schema.create_table_from_entity(entity::prelude::AiModel)))
            .await
            .unwrap();

        // todo new ai source & ai model
        let user_id = Uuid::new_v4().to_string();
        let ws_id = Uuid::new_v4().to_string();
        let ws_path = &user_path.join(&ws_id);
        if !ws_path.exists() {
            fs::create_dir_all(ws_path).unwrap();
        }
        // create chat
        let result = create(
            db,
            &user_id,
            &CreateBody {
                name: "chat1".to_string(),
                wid: ws_id.clone(),
            },
        )
        .await;
        if result.is_error() {
            panic!("{:?}", result.message);
        }
        let chat_info = result.result.clone().unwrap();
        println!("{:?}", chat_info);
        let chat_id = &chat_info.id;
        // todo mock openai to test
        // send message
        let result = message_request(
            db,
            user_path,
            &user_id,
            &RequestBody {
                id: chat_id.to_string(),
                prompt: "who is blackstar".to_string(),
                model_id: "llama3.1:8b".to_string(),
                source_id: "abc".to_string(),
                request_id: "1".to_string(),
            },
        )
        .await;
        if result.is_error() {
            panic!("{:?}", result.message);
        }
        println!("{:?}", result.result.unwrap());
        // request twice
        let result = message_request(
            db,
            user_path,
            &user_id,
            &RequestBody {
                id: chat_id.to_string(),
                prompt: "oh, no".to_string(),
                model_id: "llama3.1:8b".to_string(),
                source_id: "abc".to_string(),
                request_id: "2".to_string(),
            },
        )
        .await;
        if result.is_error() {
            panic!("{:?}", result.message);
        }
        println!("{:?}", result.result.unwrap());
    }
}
