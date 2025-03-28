use std::path::PathBuf;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use log::{debug, trace};
use sea_orm::DatabaseConnection;
use serde_json::{to_value, Value};
use tauri::{State, Window};

use app::dto::file::{
    CopyBody as FileCopyBody, CreateBody as FileCreateBody, GeneralBody as FileGeneralBody,
    ListByPageBody as FileListByPageBody, ListByPidBody as FileListByPidBody,
    ListGeneralBody as FileListGeneralBody, UpdateBody as FileUpdateBody,
    UpdateContentBody as FileUpdateContentBody, UpdateNameBody as FileUpdateNameBody,
};
use app::service::ai_chat_service::{
    create as chat_create, delete as chat_delete, list as chat_list,
    message_edit as chat_message_edit, message_list as chat_message_list,
    message_regenerate as chat_message_regenerate, message_request_stream as chat_message_request,
    model_list as chat_model_list, update_name as chat_update_name, CommonBody as ChatCommonBody,
    CreateBody as ChatCreateBody, EditBody as ModelMessageEditBody, ListBody as ChatListBody,
    RegenerateBody as ModelMessageRegenerateBody, RequestBody as ChatRequestBody,
    UpdateNameBody as ChatUpdateNameBody,
};
use app::service::file_service::{
    copy_file, create_file, delete_file, get_file, get_path, get_workspace_files,
    get_workspace_files_by_page, get_workspace_files_by_pid, update_file, update_file_content,
    update_file_name,
};

use app::service::ai_source_service::{
    create as ai_source_create, delete as ai_source_delete, enable as ai_source_enable,
    list as ai_source_list, list_enable as ai_source_list_enable, update as ai_source_update,
};

use app::service::ai_model_service::{
    create as ai_model_create, delete as ai_model_delete, enable as ai_model_enable,
    list as ai_model_list, list_enable as ai_model_list_enable,
};

use app::dto::ai_model::{
    CommonBody as AiModelCommonBody, CreateBody as AiModelCreateBody,
    EnableBody as AiModelEnableBody, ListBody as AiModelListBody,
};
use app::dto::ai_source::{
    CommonBody as AiSourceCommonBody, CreateBody as AiSourceCreateBody,
    EnableBody as AiSourceEnableBody, UpdateBody as AiSourceUpdateBody,
};
use app::dto::chat::ChunkPayload;
use app::service::user_service::{
    get_access_codes, get_user_info, login, logout, refresh_token, register, LoginBody,
    RegisterBody,
};
use app::service::workspace_service::{
    create_workspace, delete_workspace, get_workspace, list_workspaces,
    CreateBody as WorkspaceCreateBody, GeneralBody as WorkspaceGeneralBody,
};
use app::{AppResponse, AppState, LoginInfo};

#[tauri::command]
pub async fn route_cmd(
    window: Window,
    state: State<'_, AppState>,
    command: String,
    access_token: Option<String>,
    args: Value,
) -> Result<Value, ()> {
    if command.is_empty() {
        return Ok(to_value(&AppResponse::error(None::<String>, "Command is empty")).unwrap());
    }
    // Pre-processing or logging logic
    let db = &state.conn;
    let user_path = &state.user_path;
    return if command.starts_with("user") {
        Ok(invoke_user_cmd(db, command, access_token, args).await)
    } else if command.starts_with("chat") {
        Ok(invoke_chat_cmd(window, db, user_path, command, access_token, args).await)
    } else if command.starts_with("workspace") {
        Ok(invoke_workspace_cmd(db, command, access_token, args).await)
    } else if command.starts_with("file") {
        Ok(invoke_file_cmd(db, user_path, command, access_token, args).await)
    } else if command.starts_with("ai_source") {
        Ok(invoke_ai_source_cmd(db, user_path, command, access_token, args).await)
    } else if command.starts_with("ai_model") {
        Ok(invoke_ai_model_cmd(db, user_path, command, access_token, args).await)
    } else {
        let response =
            AppResponse::error(None::<String>, &format!("Command {:?} not found", command));
        Ok(to_value(&response).unwrap())
    };
}

fn get_user_info_from_access_token(
    access_token_option: Option<String>,
) -> Result<LoginInfo, anyhow::Error> {
    if access_token_option.is_none() {
        return Err(anyhow::anyhow!("User token is null"));
    }
    let access_token = &access_token_option.unwrap();
    if access_token.is_empty() {
        return Err(anyhow::anyhow!("User token is empty"));
    }
    // todo throw exception if access_token is expired
    let result = BASE64_STANDARD.decode(access_token).unwrap();
    let user_id = String::from_utf8(result).unwrap();
    Ok(LoginInfo {
        access_token: access_token.to_string(),
        desc: "".to_string(),
        real_name: "".to_string(),
        user_id,
        username: "".to_string(),
        mail: None,
    })
}

pub async fn invoke_user_cmd(
    db: &DatabaseConnection,
    command: String,
    access_token: Option<String>,
    args: Value,
) -> Value {
    if command == "user_login" {
        let result: LoginBody = serde_json::from_value(args).unwrap();
        let response = login(db, &result).await;
        return to_value(&response).unwrap();
    } else if command == "user_register" {
        let result: RegisterBody = serde_json::from_value(args).unwrap();
        let response = register(db, &result).await;
        return to_value(&response).unwrap();
    }
    let login_info_result = get_user_info_from_access_token(access_token);
    if login_info_result.is_err() {
        return to_value(&AppResponse::error(
            None::<String>,
            &login_info_result.err().unwrap().to_string(),
        ))
        .unwrap();
    }
    let login_info = login_info_result.unwrap();
    let user_id = &login_info.user_id;
    let access_token_str = &login_info.access_token;
    match command.as_str() {
        "user_get_info" => {
            let response = get_user_info(db, user_id).await;
            to_value(&response).unwrap()
        }
        "user_login" => {
            let result: LoginBody = serde_json::from_value(args).unwrap();
            let response = login(db, &result).await;
            to_value(&response).unwrap()
        }
        "user_logout" => {
            let response = logout().await;
            to_value(&response).unwrap()
        }
        "user_refresh_token" => {
            let response = refresh_token(db, access_token_str).await;
            to_value(&response).unwrap()
        }
        "user_get_access_codes" => {
            let response = get_access_codes(db).await;
            to_value(&response).unwrap()
        }
        _ => to_value(&AppResponse::error(
            None::<String>,
            "User command not found",
        ))
        .unwrap(),
    }
}

pub async fn invoke_chat_cmd(
    window: Window,
    db: &DatabaseConnection,
    user_path: &PathBuf,
    command: String,
    access_token: Option<String>,
    args: Value,
) -> Value {
    if access_token.is_none() {
        return to_value(&AppResponse::error(None::<String>, "User token is null")).unwrap();
    }
    let access_token = access_token.unwrap();
    let result = BASE64_STANDARD.decode(&access_token).unwrap();
    let user_id = String::from_utf8(result).unwrap();
    match command.as_str() {
        "chat_get_models" => {
            let response = chat_model_list().await;
            to_value(&response).unwrap()
        }
        "chat_list" => {
            let body: ChatListBody = serde_json::from_value(args).unwrap();
            let response = chat_list(db, &user_id, &body.wid).await;
            to_value(&response).unwrap()
        }
        "chat_create" => {
            let body: ChatCreateBody = serde_json::from_value(args).unwrap();
            let response = chat_create(db, &user_id, &body).await;
            to_value(&response).unwrap()
        }
        "chat_delete" => {
            let body: ChatCommonBody = serde_json::from_value(args).unwrap();
            let response = chat_delete(db, &user_id, &body.id).await;
            to_value(&response).unwrap()
        }
        "chat_update_name" => {
            let body: ChatUpdateNameBody = serde_json::from_value(args).unwrap();
            let response = chat_update_name(db, &user_id, &body).await;
            to_value(&response).unwrap()
        }
        "chat_model_list" => {
            let response = chat_model_list().await;
            to_value(&response).unwrap()
        }
        "chat_message_list" => {
            let body: ChatCommonBody = serde_json::from_value(args).unwrap();
            let response = chat_message_list(db, user_path, &user_id, &body.id).await;
            to_value(&response).unwrap()
        }
        "chat_message_request" => {
            let body: ChatRequestBody = serde_json::from_value(args).unwrap();
            let callback_wrapper = |content: Option<String>, status: i8| {
                window
                    .emit(
                        &body.request_id,
                        ChunkPayload {
                            chunk: content,
                            status,
                        },
                    )
                    .unwrap();
            };
            debug!("request body: {:?}", body);
            let response =
                chat_message_request(callback_wrapper, db, user_path, &user_id, &body).await;
            to_value(&response).unwrap()
        }
        "chat_message_regenerate" => {
            let body: ModelMessageRegenerateBody = serde_json::from_value(args).unwrap();
            let callback_wrapper = |content: Option<String>, status: i8| {
                window
                    .emit(
                        &body.request_id,
                        ChunkPayload {
                            chunk: content,
                            status,
                        },
                    )
                    .unwrap();
            };
            let response =
                chat_message_regenerate(callback_wrapper, db, user_path, &user_id, &body).await;
            to_value(&response).unwrap()
        }
        "chat_message_edit" => {
            let body: ModelMessageEditBody = serde_json::from_value(args).unwrap();
            let callback_wrapper = |content: Option<String>, status: i8| {
                window
                    .emit(
                        &body.request_id,
                        ChunkPayload {
                            chunk: content,
                            status,
                        },
                    )
                    .unwrap();
            };
            let response =
                chat_message_edit(callback_wrapper, db, user_path, &user_id, &body).await;
            to_value(&response).unwrap()
        }
        _ => to_value(&AppResponse::error(
            None::<String>,
            "Chat command not found",
        ))
        .unwrap(),
    }
}

pub async fn invoke_ai_source_cmd(
    db: &DatabaseConnection,
    user_path: &PathBuf,
    command: String,
    access_token: Option<String>,
    args: Value,
) -> Value {
    if access_token.is_none() {
        return to_value(&AppResponse::error(None::<String>, "User token is null")).unwrap();
    }
    let access_token = access_token.unwrap();
    let result = BASE64_STANDARD.decode(&access_token).unwrap();
    let user_id = String::from_utf8(result).unwrap();
    match command.as_str() {
        "ai_source_list" => {
            let response = ai_source_list(db).await;
            to_value(&response).unwrap()
        }
        "ai_source_list_enable" => {
            let response = ai_source_list_enable(db).await;
            to_value(&response).unwrap()
        }
        "ai_source_create" => {
            let body: AiSourceCreateBody = serde_json::from_value(args).unwrap();
            let response = ai_source_create(db, &body).await;
            to_value(&response).unwrap()
        }
        "ai_source_delete" => {
            let body: AiSourceCommonBody = serde_json::from_value(args).unwrap();
            let response = ai_source_delete(db, &body.id).await;
            to_value(&response).unwrap()
        }
        "ai_source_update" => {
            let body: AiSourceUpdateBody = serde_json::from_value(args).unwrap();
            let response = ai_source_update(db, &body).await;
            to_value(&response).unwrap()
        }
        "ai_source_enable" => {
            let body: AiSourceEnableBody = serde_json::from_value(args).unwrap();
            let response = ai_source_enable(db, &body).await;
            to_value(&response).unwrap()
        }
        _ => to_value(&AppResponse::error(
            None::<String>,
            "Ai source command not found",
        ))
        .unwrap(),
    }
}

pub async fn invoke_ai_model_cmd(
    db: &DatabaseConnection,
    user_path: &PathBuf,
    command: String,
    access_token: Option<String>,
    args: Value,
) -> Value {
    if access_token.is_none() {
        return to_value(&AppResponse::error(None::<String>, "User token is null")).unwrap();
    }
    let access_token = access_token.unwrap();
    let result = BASE64_STANDARD.decode(&access_token).unwrap();
    let user_id = String::from_utf8(result).unwrap();
    match command.as_str() {
        "ai_model_list" => {
            let body: AiModelListBody = serde_json::from_value(args).unwrap();
            let response = ai_model_list(db, &body.source_id).await;
            to_value(&response).unwrap()
        }
        "ai_model_list_enable" => {
            let body: AiModelListBody = serde_json::from_value(args).unwrap();
            let response = ai_model_list_enable(db, &body.source_id).await;
            to_value(&response).unwrap()
        }
        "ai_model_create" => {
            let body: AiModelCreateBody = serde_json::from_value(args).unwrap();
            let response = ai_model_create(db, &body).await;
            to_value(&response).unwrap()
        }
        "ai_model_delete" => {
            let body: AiModelCommonBody = serde_json::from_value(args).unwrap();
            let response = ai_model_delete(db, &body.id).await;
            to_value(&response).unwrap()
        }
        // "ai_model_sync" => {
        //     let response = chat_model_list().await;
        //     to_value(&response).unwrap()
        // }
        "ai_model_enable" => {
            let body: AiModelEnableBody = serde_json::from_value(args).unwrap();
            let response = ai_model_enable(db, &body).await;
            to_value(&response).unwrap()
        }
        _ => to_value(&AppResponse::error(
            None::<String>,
            "Ai model command not found",
        ))
        .unwrap(),
    }
}

pub async fn invoke_workspace_cmd(
    db: &DatabaseConnection,
    command: String,
    access_token: Option<String>,
    args: Value,
) -> Value {
    let login_info_result = get_user_info_from_access_token(access_token);
    if login_info_result.is_err() {
        return to_value(&AppResponse::error(
            None::<String>,
            &login_info_result.err().unwrap().to_string(),
        ))
        .unwrap();
    }
    let login_info = login_info_result.unwrap();
    let user_id = &login_info.user_id;
    match command.as_str() {
        "workspace_list" => {
            let response = list_workspaces(db).await;
            to_value(&response).unwrap()
        }
        "workspace_create" => {
            let body: WorkspaceCreateBody = serde_json::from_value(args).unwrap();
            let response = create_workspace(db, user_id, &body.name).await;
            to_value(&response).unwrap()
        }
        "workspace_delete" => {
            let body: WorkspaceGeneralBody = serde_json::from_value(args).unwrap();
            let response = delete_workspace(db, &body.id).await;
            to_value(&response).unwrap()
        }
        "workspace_get" => {
            let body: WorkspaceGeneralBody = serde_json::from_value(args).unwrap();
            let response = get_workspace(db, &body.id).await;
            to_value(&response).unwrap()
        }
        _ => to_value(&AppResponse::error(
            None::<String>,
            "Workspace command not found",
        ))
        .unwrap(),
    }
}

pub async fn invoke_file_cmd(
    db: &DatabaseConnection,
    user_path: &PathBuf,
    command: String,
    access_token: Option<String>,
    args: Value,
) -> Value {
    // if access_token.is_none() {
    //     return to_value(&AppResponse::error(None::<String>, "User token is null")).unwrap()
    // }
    // let access_token = access_token.unwrap();
    // let result = BASE64_STANDARD.decode(&access_token).unwrap();
    // let user_id = String::from_utf8(result).unwrap();
    // todo need user_id to query
    match command.as_str() {
        "file_get_all_workspace_files" => {
            let body: FileListGeneralBody = serde_json::from_value(args).unwrap();
            let response = get_workspace_files(db, &body).await;
            to_value(&response).unwrap()
        }
        "file_get_workspace_files_by_id" => {
            let body: FileListByPidBody = serde_json::from_value(args).unwrap();
            let response = get_workspace_files_by_pid(db, &body).await;
            to_value(&response).unwrap()
        }
        "file_get_workspace_files_by_page" => {
            let body: FileListByPageBody = serde_json::from_value(args).unwrap();
            let response = get_workspace_files_by_page(db, &body).await;
            to_value(&response).unwrap()
        }
        "file_get" => {
            let body: FileGeneralBody = serde_json::from_value(args).unwrap();
            let response = get_file(db, &body).await;
            to_value(&response).unwrap()
        }
        "file_get_path" => {
            let body: FileGeneralBody = serde_json::from_value(args).unwrap();
            let response = get_path(user_path, &body).await;
            to_value(&response).unwrap()
        }
        "file_copy" => {
            let body: FileCopyBody = serde_json::from_value(args).unwrap();
            let response = copy_file(db, user_path, &body).await;
            to_value(&response).unwrap()
        }
        "file_create" => {
            let body: FileCreateBody = serde_json::from_value(args).unwrap();
            let response = create_file(db, user_path, &body).await;
            to_value(&response).unwrap()
        }
        "file_update" => {
            let body: FileUpdateBody = serde_json::from_value(args).unwrap();
            let response = update_file(db, &body).await;
            to_value(&response).unwrap()
        }
        "file_update_content" => {
            let body: FileUpdateContentBody = serde_json::from_value(args).unwrap();
            let response = update_file_content(db, user_path, &body).await;
            to_value(&response).unwrap()
        }
        "file_update_name" => {
            let body: FileUpdateNameBody = serde_json::from_value(args).unwrap();
            let response = update_file_name(db, &body).await;
            to_value(&response).unwrap()
        }
        "file_delete" => {
            let body: FileGeneralBody = serde_json::from_value(args).unwrap();
            let response = delete_file(db, &body).await;
            to_value(&response).unwrap()
        }
        _ => to_value(&AppResponse::error(
            None::<String>,
            "File command not found",
        ))
        .unwrap(),
    }
}
