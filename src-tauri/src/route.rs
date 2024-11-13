use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use sea_orm::DatabaseConnection;
use serde_json::{to_value, Value};
use tauri::State;
use app::{AppResponse, AppState};
use app::service::user_service::{get_access_codes, get_user_info, login, LoginBody, logout, refresh_token, register, RegisterBody};

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
    return if command.starts_with("user") {
        Ok(invoke_user_cmd(&db, command, access_token, args).await)
    } else if command.starts_with("workspace"){
        // todo
        Ok(Value::Null)
    } else if command.starts_with("model") {
        // todo
        Ok(Value::Null)
    } else {
        let response = AppResponse::error(None::<String>, &format!("Command {:?} not found", command));
        Ok(to_value(&response).unwrap())
    }
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
        return to_value(&response).unwrap()
    } else if command == "user_register" {
        let result: RegisterBody = serde_json::from_value(args).unwrap();
        let response = register(db, &result).await;
        return to_value(&response).unwrap()
    }
    if access_token.is_none() {
        return to_value(&AppResponse::error(None::<String>, "User token is null")).unwrap()
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
        _ => to_value(&AppResponse::error(None::<String>, "Command not found")).unwrap(),
    };
}