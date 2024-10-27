use axum::response;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use futures::FutureExt;
use sea_orm::DatabaseConnection;
use serde_json::{to_string, to_value, Value};
use tauri::State;

use app::adapter::user_adapter::{get_access_codes, get_user_info, login, refresh_token, register, LoginBody, LoginInfo, RefreshTokenResult, RegisterBody, UserInfo, logout};
use app::entity::user::Model;
use app::AppState;
use app::{AppResponse, RESPONSE_CODE_SUCCESS};

#[tauri::command]
pub async fn intercepted_command(
    state: State<'_, AppState>,
    command: String,
    access_token: Option<String>,
    args: Value,
) -> Result<Value, ()> {
    // Pre-processing or logging logic
    let db = &state.conn;
    return match command.as_str() {
        "" => Ok(to_value(&AppResponse::error(None::<String>, "Command is empty")).unwrap()),
        "get_user_info" => {
            if access_token.is_none() {
                return Ok(
                    to_value(&AppResponse::error(None::<String>, "User token is null")).unwrap(),
                );
            }
            let result = BASE64_STANDARD.decode(access_token.unwrap()).unwrap();
            let user_id = String::from_utf8(result).unwrap();
            let response = get_user_info(db, &user_id).await;
            Ok(to_value(&response).unwrap())
        }
        "user_login" => {
            let result:LoginBody = serde_json::from_value(args).unwrap();
            let response = login(db, &result).await;
            Ok(to_value(&response).unwrap())
        }
        "user_logout" => {
            let response = logout().await;
            Ok(to_value(&response).unwrap())
        }
        "user_register" => {
            let result:RegisterBody = serde_json::from_value(args).unwrap();
            let response = register(&state.conn, &result)
                .await;
            Ok(to_value(&response).unwrap())
        }
        _ => Ok(to_value(&AppResponse::error(None::<String>, "Command not found")).unwrap()),
    };
}


#[tauri::command]
pub async fn get_access_codes_cmd(state: State<'_, AppState>) -> Result<Vec<String>, ()> {
    get_access_codes(&state.conn).await
}

#[tauri::command]
pub async fn refresh_token_cmd(
    state: State<'_, AppState>,
    access_token: String,
) -> Result<RefreshTokenResult, ()> {
    refresh_token(&state.conn, &access_token).await
}
