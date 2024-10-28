use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use serde_json::{to_value, Value};
use tauri::State;
use app::{AppResponse, AppState};
use app::adapter::user_adapter::{get_access_codes, get_user_info, login, LoginBody, logout, refresh_token, register, RegisterBody};

#[tauri::command]
pub async fn route_cmd(
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
            let result: LoginBody = serde_json::from_value(args).unwrap();
            let response = login(db, &result).await;
            Ok(to_value(&response).unwrap())
        }
        "user_logout" => {
            let response = logout().await;
            Ok(to_value(&response).unwrap())
        }
        "user_register" => {
            let result: RegisterBody = serde_json::from_value(args).unwrap();
            let response = register(db, &result).await;
            Ok(to_value(&response).unwrap())
        }
        "user_refresh_token" => {
            if access_token.is_none() {
                return Ok(
                    to_value(&AppResponse::error(None::<String>, "User token is null")).unwrap(),
                );
            }
            let response = refresh_token(db, &access_token.unwrap()).await;
            Ok(to_value(&response).unwrap())
        }
        "get_user_access_codes" => {
            let response = get_access_codes(db).await;
            Ok(to_value(&response).unwrap())
        }
        _ => Ok(to_value(&AppResponse::error(None::<String>, "Command not found")).unwrap()),
    };
}