use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use sea_orm::DatabaseConnection;
use serde_json::{to_value, Value};
use std::path::PathBuf;
use tauri::State;

use app::service::file_service::{create_file, delete_file, get_file, get_path, get_workspace_files, get_workspace_files_by_pid, update_file_content, update_file_name, CreateBody as FileCreateBody, GeneralBody as FileGeneralBody, ListByPageBody as FileListByPageBody, ListByPidBody as FileListByPidBody, ListGeneralBody as FileListGeneralBody, UpdateContentBody as FileUpdateContentBody, UpdateNameBody as FileUpdateNameBody, get_workspace_files_by_page};
use app::service::model_service::{
    chat_request, get_chat_history_messages, get_chats, get_models, ChatRequestBody,
    DeepChatRequestBody,
};
use app::service::user_service::{
    get_access_codes, get_user_info, login, logout, refresh_token, register, LoginBody,
    RegisterBody,
};
use app::service::workspace_service::{
    create_workspace, delete_workspace, get_workspace, list_workspaces,
    CreateBody as WorkspaceCreateBody, GeneralBody as WorkspaceGeneralBody,
};
use app::{AppResponse, AppState};

#[tauri::command]
pub async fn route_cmd(
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
    let file_db = &state.file_conn;
    let user_path = &state.user_path;
    return if command.starts_with("user") {
        Ok(invoke_user_cmd(db, command, access_token, args).await)
    } else if command.starts_with("model") {
        Ok(invoke_model_cmd(db, command, access_token, args).await)
    } else if command.starts_with("workspace") {
        Ok(invoke_workspace_cmd(file_db, command, access_token, args).await)
    } else if command.starts_with("file") {
        Ok(invoke_file_cmd(file_db, user_path, command, access_token, args).await)
    } else {
        let response =
            AppResponse::error(None::<String>, &format!("Command {:?} not found", command));
        Ok(to_value(&response).unwrap())
    };
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
    if access_token.is_none() {
        return to_value(&AppResponse::error(None::<String>, "User token is null")).unwrap();
    }
    let access_token = access_token.unwrap();
    let result = BASE64_STANDARD.decode(&access_token).unwrap();
    let user_id = String::from_utf8(result).unwrap();
    return match command.as_str() {
        "user_get_info" => {
            let response = get_user_info(db, &user_id).await;
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
            let response = refresh_token(db, &access_token).await;
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
    };
}

pub async fn invoke_model_cmd(
    db: &DatabaseConnection,
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
    return match command.as_str() {
        "model_get_models" => {
            let response = get_models().await;
            to_value(&response).unwrap()
        }
        "model_get_chats" => {
            let response = get_chats().await;
            to_value(&response).unwrap()
        }
        "model_get_chat_history_messages" => {
            let body: ChatRequestBody = serde_json::from_value(args).unwrap();
            let response = get_chat_history_messages(body.id).await;
            to_value(&response).unwrap()
        }
        "model_chat_request" => {
            let body: DeepChatRequestBody = serde_json::from_value(args).unwrap();
            let response = chat_request(body).await;
            to_value(&response).unwrap()
        }
        "model_chat_stream_request" => {
            // todo
            Value::Null
        }
        "modeL_chat_files_request" => {
            // todo
            Value::Null
        }
        _ => to_value(&AppResponse::error(
            None::<String>,
            "Model command not found",
        ))
        .unwrap(),
    };
}

pub async fn invoke_workspace_cmd(
    db: &DatabaseConnection,
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
    return match command.as_str() {
        "workspace_list" => {
            let response = list_workspaces(db).await;
            to_value(&response).unwrap()
        }
        "workspace_create" => {
            let body: WorkspaceCreateBody = serde_json::from_value(args).unwrap();
            let response = create_workspace(db, &body.name).await;
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
    };
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
    return match command.as_str() {
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
        "file_create" => {
            let body: FileCreateBody = serde_json::from_value(args).unwrap();
            let response = create_file(db, user_path, &body).await;
            to_value(&response).unwrap()
        }
        "file_update_content" => {
            let body: FileUpdateContentBody = serde_json::from_value(args).unwrap();
            let response = update_file_content(db, user_path, &body).await;
            to_value(&response).unwrap()
        }
        "file_update_name" => {
            let body: FileUpdateNameBody = serde_json::from_value(args).unwrap();
            let response = update_file_name(db, user_path, &body).await;
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
    };
}
