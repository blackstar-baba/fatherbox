use base64::engine::general_purpose::STANDARD_NO_PAD;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tauri::State;

use app::{AppResponse, AppState};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LoginResult {
    access_token: String,
    desc: String,
    real_name: String,
    user_id: String,
    username: String,
    mail: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    id: String,
    password: String,
    real_name: String,
    roles: Vec<String>,
    username: String,
    avatar: String,
    mail: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenResult {
    data: String,
    status: i8,
}

#[tauri::command]
pub async fn user_login_cmd(
    state: State<'_, AppState>,
    username: String,
    password: String,
) -> Result<LoginResult, ()> {
    user_login(&state.conn, &username, &password).await
}

#[tauri::command]
pub async fn get_user_info_cmd(state: State<'_, AppState>) -> Result<UserInfo, ()> {
    get_user_info(&state.conn).await
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

async fn user_login(
    db: &DatabaseConnection,
    username: &str,
    password: &str,
) -> Result<LoginResult, ()> {
    let access_token = BASE64_STANDARD.encode(username);
    let result = LoginResult {
        access_token,
        desc: "".to_owned(),
        real_name: "local user".to_owned(),
        user_id: "0".to_owned(),
        username: username.to_owned(),
        mail: "535650957@qq.com".to_string(),
    };
    return Ok(result);
}

async fn get_user_info(db: &DatabaseConnection) -> Result<UserInfo, ()> {
    let result = UserInfo {
        id: "0".to_owned(),
        password: "123456".to_owned(),
        real_name: "local user".to_owned(),
        username: "fatherbox".to_owned(),
        roles: vec!["super".to_owned()],
        avatar: "/avatar.svg".to_string(),
        mail: "535650957@qq.com".to_string(),
    };
    return Ok(result);
}

async fn refresh_token(db: &DatabaseConnection, access_token: &str) -> Result<RefreshTokenResult, ()> {
    return Ok(RefreshTokenResult{
        data: access_token.to_owned(),
        status: 0,
    })
}



async fn get_access_codes(db: &DatabaseConnection) -> Result<Vec<String>,()> {
    let codes = vec!["AC_100100".to_owned(), "AC_100110".to_owned(), "AC_100120".to_owned(), "AC_100010".to_owned()];
    return Ok(codes);
}

#[cfg(test)]
mod tests {
    use std::env::temp_dir;

    use crate::db_utils::{drop_database_file, exist_database_file, init_connection};
    use crate::user_command::{get_access_codes, get_user_info, user_login};

    #[tokio::test]
    async fn test_user_login() {
        let temp_dir = temp_dir();
        let base_path = temp_dir.join(".fatherbox");
        let file_path = &base_path.join("test-user.sqlite");
        if exist_database_file(file_path) {
            drop_database_file(&file_path).unwrap();
        }
        let db = init_connection(&file_path).await.unwrap();
        let create_result = user_login(&db, "admin", "123456").await.unwrap();
        assert_eq!("admin", create_result.username);
        assert_eq!("0", create_result.user_id);

        let get_user_info_result = get_user_info(&db).await.unwrap();
        assert_eq!("fatherbox", get_user_info_result.username);
        assert_eq!("0", get_user_info_result.id);

        let access_codes = get_access_codes(&db).await.unwrap();
        assert_eq!("AC_100100", access_codes[0]);
    }
}
