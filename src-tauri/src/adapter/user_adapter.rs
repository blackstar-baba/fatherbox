use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use chrono::Utc;
use futures::future::ok;
use futures::FutureExt;
use log::error;
use sea_orm::ActiveValue::Set;
use sea_orm::{DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

use crate::entity::user;
use crate::entity::user::Model;
use crate::service::user_service::UserService;
use crate::{AppResponse, RESPONSE_CODE_ERROR, RESPONSE_CODE_SUCCESS};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LoginInfo {
    pub access_token: String,
    pub desc: String,
    pub real_name: String,
    pub user_id: String,
    pub username: String,
    pub mail: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub id: String,
    pub real_name: String,
    pub roles: Vec<String>,
    pub username: String,
    pub avatar: Option<String>,
    pub mail: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RegisterBody {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
    pub agree_policy: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LoginBody {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenResult {
    pub data: String,
    pub status: i8,
}

pub async fn register(db: &DatabaseConnection, body: &RegisterBody) -> AppResponse<Option<Model>> {
    let active_model = user::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        username: Set(body.username.clone()),
        real_name: Set(body.username.to_string()),
        avatar: Default::default(),
        password: Set(body.password.clone()),
        mail: Default::default(),
        r#type: Set("local".to_string()),
        ref_user_id: Default::default(),
        create_time: Set(Utc::now().timestamp()),
        update_time: Set(Utc::now().timestamp()),
        state: Set(1),
    };
    let result = UserService::create_user(db, active_model).await;
    match result {
        Ok(model) => AppResponse {
            code: RESPONSE_CODE_SUCCESS,
            r#type: "".to_string(),
            message: "".to_string(),
            result: Option::from(model),
        },
        Err(err) => {
            error!("create user failed, err: {}", err);
            AppResponse {
                code: RESPONSE_CODE_ERROR,
                r#type: "".to_string(),
                message: err.to_string(),
                result: None,
            }
        }
    }
}
pub async fn login(db: &DatabaseConnection, body: &LoginBody) -> AppResponse<Option<LoginInfo>> {
    let result = UserService::get_user(db, &body.username, &body.password, "local").await;
    match result {
        Ok(vec) => {
            if vec.is_empty() {
                AppResponse {
                    code: RESPONSE_CODE_ERROR,
                    r#type: "".to_string(),
                    message: "User not found".to_string(),
                    result: None,
                }
            } else {
                // todo generate access token
                let access_token = BASE64_STANDARD.encode(vec[0].id.to_owned());
                let result = LoginInfo {
                    access_token,
                    desc: "".to_owned(),
                    real_name: vec[0].real_name.to_owned(),
                    user_id: vec[0].id.to_owned(),
                    username: vec[0].username.to_owned(),
                    mail: vec[0].mail.clone(),
                };
                AppResponse {
                    code: RESPONSE_CODE_SUCCESS,
                    r#type: "".to_string(),
                    message: "User not found".to_string(),
                    result: Some(result),
                }
            }
        }
        Err(err) => {
            error!("user login, err: {}", err);
            AppResponse {
                code: RESPONSE_CODE_ERROR,
                r#type: "".to_string(),
                message: err.to_string(),
                result: None,
            }
        }
    }
}

pub async fn get_user_info(db: &DatabaseConnection, id: &str) -> AppResponse<Option<UserInfo>> {
    let result = UserService::get_user_by_id(db, id).await;
    match result {
        Ok(vec) => {
            if vec.is_empty() {
                AppResponse {
                    code: RESPONSE_CODE_ERROR,
                    r#type: "".to_string(),
                    message: "User not found".to_string(),
                    result: None,
                }
            } else {
                let avatar = match &vec[0].avatar {
                    None => "".to_string(),
                    Some(byte_array) => String::from_utf8(byte_array.to_owned()).unwrap(),
                };
                let user_info = UserInfo {
                    id: vec[0].id.to_owned(),
                    real_name: vec[0].real_name.to_owned(),
                    username: vec[0].username.to_owned(),
                    roles: vec![],
                    avatar: Some(avatar),
                    mail: vec[0].mail.clone(),
                };
                AppResponse {
                    code: RESPONSE_CODE_SUCCESS,
                    r#type: "".to_owned(),
                    message: "".to_owned(),
                    result: Some(user_info),
                }
            }
        }
        Err(err) => AppResponse {
            code: RESPONSE_CODE_ERROR,
            r#type: "".to_owned(),
            message: err.to_string(),
            result: None,
        },
    }
}

pub async fn refresh_token(
    db: &DatabaseConnection,
    access_token: &str,
) -> Result<RefreshTokenResult, ()> {
    // todo validate token and refresh
    return Ok(RefreshTokenResult {
        data: access_token.to_owned(),
        status: 0,
    });
}

pub async fn get_access_codes(db: &DatabaseConnection) -> Result<Vec<String>, ()> {
    // todo check this
    let codes = vec![
        "AC_100100".to_owned(),
        "AC_100110".to_owned(),
        "AC_100120".to_owned(),
        "AC_100010".to_owned(),
    ];
    return Ok(codes);
}

pub async fn logout(
) -> Result<AppResponse<Option<String>>, ()> {
    return Ok(AppResponse::success(Some(String::new())))
}

#[cfg(test)]
mod tests {
    use std::env::temp_dir;

    use sea_orm::{ConnectionTrait, Schema};

    use crate::adapter::user_adapter::{get_user_info, login, register, LoginBody, RegisterBody};
    use crate::util::db_util::{drop_database_file, exist_database_file, init_connection};
    use crate::{entity, RESPONSE_CODE_SUCCESS};

    #[tokio::test]
    async fn test_user() {
        tracing_subscriber::fmt()
            .pretty()
            .with_line_number(false)
            .with_file(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_env_filter("debug")
            .init();

        let temp_dir = temp_dir();
        let base_path = temp_dir.join(".fatherbox");
        let file_path = &base_path.join("test-user.sqlite");
        if exist_database_file(file_path) {
            drop_database_file(&file_path).unwrap();
        }
        let db = init_connection(&file_path).await.unwrap();
        let builder = db.get_database_backend();
        let schema = Schema::new(builder);
        db.execute(builder.build(&schema.create_table_from_entity(entity::prelude::User)))
            .await
            .unwrap();
        // register
        let username = "admin";
        let password = "123456";
        let register_response = register(
            &db,
            &RegisterBody {
                username: username.to_string(),
                password: password.to_string(),
                confirm_password: password.to_string(),
                agree_policy: true,
            },
        )
        .await;
        assert_eq!(RESPONSE_CODE_SUCCESS, register_response.code);
        let option_model = register_response.result;
        if option_model.is_none() {
            panic!("register failed");
        }
        let model = option_model.unwrap();
        assert_ne!("", model.id);
        // get user
        let id = &model.id;
        let get_response = get_user_info(&db, id).await;
        assert_eq!(RESPONSE_CODE_SUCCESS, get_response.code);
        if get_response.result.is_none() {
            panic!("get user failed");
        }
        let user_info = get_response.result.unwrap();
        assert_eq!(username, user_info.username);
        // login
        let username = model.username.as_str();
        let login_info_response = login(
            &db,
            &LoginBody {
                username: username.to_string(),
                password: password.to_string(),
            },
        )
        .await;
        assert_eq!(RESPONSE_CODE_SUCCESS, login_info_response.code);
        let option_login_info = login_info_response.result;
        if option_login_info.is_none() {
            panic!("login failed");
        }
        let login_info = option_login_info.unwrap();
        assert_ne!("", login_info.access_token);
    }
}
