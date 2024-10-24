use anyhow::Error;
use futures::FutureExt;
use std::sync::{LockResult, TryLockResult};
use tauri::State;

use app::adapter::user_adapter::{
    get_access_codes, get_user_info, login, refresh_token, register, LoginBody, LoginInfo,
    RefreshTokenResult, RegisterBody, UserInfo,
};
use app::entity::user::Model;
use app::{get_user_id, set_user_id, AppState};
use app::{AppResponse, RESPONSE_CODE_ERROR, RESPONSE_CODE_SUCCESS};

#[tauri::command]
pub async fn user_register_cmd(
    state: State<'_, AppState>,
    body: RegisterBody,
) -> Result<AppResponse<Option<Model>>, ()> {
    register(&state.conn, &body)
        .map(|response| return Ok(response))
        .await
}

#[tauri::command]
pub async fn user_login_cmd(
    state: State<'_, AppState>,
    body: LoginBody,
) -> Result<AppResponse<Option<LoginInfo>>, ()> {
    // Attempt login
    let login_response = login(&state.conn, &body).await;

    if login_response.code.clone() != RESPONSE_CODE_SUCCESS {
        return Ok(login_response);
    }
    let login_result = login_response.result.unwrap();
    return match set_user_id(&state, &login_result.user_id) {
        Ok(_) => Ok(AppResponse::success(Some(login_result))),
        Err(err) => Ok(AppResponse::error(None, &err.to_string())),
    };
}

#[tauri::command]
pub async fn user_logout_cmd(
    state: State<'_, AppState>,
) -> Result<AppResponse<Option<String>>, ()> {
    return match set_user_id(&state, "") {
        Ok(_) => Ok(AppResponse::success(Some(String::new()))),
        Err(err) => Ok(AppResponse::error(None, &err.to_string())),
    };
}

#[tauri::command]
pub async fn get_user_info_cmd(
    state: State<'_, AppState>,
) -> Result<AppResponse<Option<UserInfo>>, ()> {
    let result = get_user_id(&state);
    match result {
        Ok(user_id) => {
            if user_id.is_empty() {
                return Ok(AppResponse::error(None,"User not login"))
            }
            get_user_info(&state.conn, &user_id)
                .map(|response| return Ok(response))
                .await
        }
        Err(err) => Ok(AppResponse::error(None, &err.to_string())),
    }
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
