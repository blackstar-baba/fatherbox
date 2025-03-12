use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::{AppResponse, CHAT_API_SETTING_KEY, DEEP_SEEK, DEEPSEEK_BASE_URL, OLLAMA_BASE_URL, OLLAMA_NAME, OPENAI_BASE_URL, OPENAI_NAME};
use crate::dao::setting_dao::SettingService;
use crate::dto::setting::CreateOrUpdateBody;
use crate::entity::setting::{ActiveModel, Model};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GeneralBody {
    pub key: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ChatApiSetting {
    pub name:  String,
    pub url:   String,
    pub key:   String,
    pub enable: bool,
    pub is_sync: bool,
}

async fn create_setting(
    db: &DatabaseConnection,
    body: &CreateOrUpdateBody,
) -> AppResponse<Option<Model>> {
    let active_model = ActiveModel {
        key: Set(body.key.clone()),
        value: Set(body.value.clone().into_bytes()),
        create_time: Set(Utc::now().timestamp()),
        update_time: Set(Utc::now().timestamp()),
        state: Set(1),
        ..Default::default()
    };
    match SettingService::create_setting(db, active_model)
        .await {
        Ok(model) => {
            AppResponse::success(Some(model))
        }
        Err(err) => {
            AppResponse::error(None, &err.to_string())
        }
    }
}


async fn get_setting(db: &DatabaseConnection, key: &str) -> AppResponse<Option<Model>> {
    match SettingService::get_setting_by_key(db, key).await {
        Ok(model) => {
            AppResponse::success(model)
        }
        Err(err) => {
            AppResponse::error(None, &err.to_string())
        }
    }
}

async fn insert_or_update_chat_api_setting(
    db: &DatabaseConnection,
    settings: &Vec<ChatApiSetting>,
) -> AppResponse<Option<bool>> {
    let result = serde_json::to_string(settings);
    if result.is_err() {
        return AppResponse::error(None, &result.err().unwrap().to_string());
    }
    let body = CreateOrUpdateBody {
        key: CHAT_API_SETTING_KEY.to_string(),
        value: result.unwrap(),
    };
    // todo need transaction
    let response = get_setting(db, CHAT_API_SETTING_KEY).await;
    if response.is_error() {
        return AppResponse::error(None, &response.message.to_string());
    }
    let model = response.result;
    if model.is_none() {
        let create_response = create_setting(db, &body).await;
        if create_response.is_error() {
            return AppResponse::error(None, &create_response.message.to_string());
        }
        return AppResponse::success(Some(true))
    }
    let update_response = update_setting(db, &body).await;
    if update_response.is_error() {
        return AppResponse::error(None, &update_response.message.to_string());
    }
    return AppResponse::success(Some(true))
}

pub async fn get_chat_api_setting(db: &DatabaseConnection) -> AppResponse<Vec<ChatApiSetting>>{
    match SettingService::get_setting_by_key(db, CHAT_API_SETTING_KEY).await {
        Ok(model) => {
            if model.is_none() {
                let mut settings = vec![];
                settings.push(ChatApiSetting {
                    name: OLLAMA_NAME.to_string(),
                    url: OLLAMA_BASE_URL.to_string(),
                    key: "".to_string(),
                    enable: false,
                    is_sync: true,
                });
                settings.push(ChatApiSetting {
                    name: DEEP_SEEK.to_string(),
                    url: DEEPSEEK_BASE_URL.to_string(),
                    key: "".to_string(),
                    enable: false,
                    is_sync: true,
                });
                settings.push(ChatApiSetting {
                    name: OPENAI_NAME.to_string(),
                    url: OPENAI_BASE_URL.to_string(),
                    key: "".to_string(),
                    enable: false,
                    is_sync: true,
                });
                return AppResponse::success(settings)
            }
            let model = model.unwrap();
            let settings: Vec<ChatApiSetting> = serde_json::from_slice(&model.value).unwrap();
            AppResponse::success(settings)
        }
        Err(err) => {
            AppResponse::error(vec![], &err.to_string())
        }
    }
}


pub async fn update_setting(db: &DatabaseConnection, body: &CreateOrUpdateBody) -> AppResponse<Option<Model>> {
    match SettingService::update_setting(db, ActiveModel{
        key: Set(body.key.clone()),
        value: Set(body.value.clone().into_bytes()),
        create_time: Set(Utc::now().timestamp()),
        update_time: Set(Utc::now().timestamp()),
        state: Set(1),
    }).await {
        Ok(model) => {
            AppResponse::success(model)
        }
        Err(err) => {
            AppResponse::error(None, &err.to_string())
        }
    }
}

pub async fn delete_setting(db: &DatabaseConnection, key: &str) -> AppResponse<Option<Model>> {
    match SettingService::delete_setting(db, key).await {
        Ok(()) => {
            AppResponse::success(None)
        }
        Err(err) => {
            AppResponse::error(None, &err.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env::temp_dir;

    use sea_orm::{ConnectionTrait, Schema};
    use tauri::Manager;
    use uuid::Uuid;

    use crate::entity;
    use crate::service::setting_service::{ChatApiSetting, create_setting, CreateOrUpdateBody, delete_setting, get_chat_api_setting, get_setting, insert_or_update_chat_api_setting, update_setting};
    use crate::util::db_util::{drop_database_file, exist_database_file, init_connection};

    #[tokio::test]
    async fn test_setting() {
        let temp_dir = temp_dir();
        let base_path = temp_dir.join(".fatherbox");
        let file_path = &base_path.join("test-setting.sqlite");
        if exist_database_file(file_path) {
            drop_database_file(&file_path).unwrap();
        }
        let db = init_connection(&file_path).await.unwrap();
        let builder = db.get_database_backend();
        let schema = Schema::new(builder);
        db.execute(builder.build(&schema.create_table_from_entity(entity::prelude::Setting)))
            .await
            .unwrap();

        let ws_id = Uuid::new_v4().to_string();
        let key = "models";
        let value = "{'a':'b'}";
        // begin invoke
        // 1. test create
        let result = create_setting(&db, &CreateOrUpdateBody {
            key: key.to_string(),
            value: value.to_string(),
        }).await;
        if result.is_error() {
            panic!("{:?}", result.message);
        }
        let model = result.result.unwrap();
        assert_eq!(value, String::from_utf8(model.value).unwrap());
        // 2. test get
        let result = get_setting(&db, key).await;
        if result.is_error() {
            panic!("{:?}", result.message);
        }
        let model = result.result.unwrap();
        assert_eq!(value, String::from_utf8(model.value).unwrap());
        // 3. test update
        let new_value = "{'a':'a1'}";
        let result = update_setting(&db, &CreateOrUpdateBody {
            key: key.to_string(),
            value: new_value.to_string(),
        }).await;
        if result.is_error() {
            panic!("{:?}", result.message);
        }
        let model = result.result.unwrap();
        assert_eq!(new_value, String::from_utf8(model.value).unwrap());
        // 4. test delete
        let result = delete_setting(&db, key).await;
        if result.is_error() {
            panic!("{:?}", result.message);
        }
        let result = get_setting(&db, key).await;
        if result.is_error() {
            panic!("{:?}", result.message);
        }
        assert!(result.result.is_none());
    }

    #[tokio::test]
    async fn test_chat_api_setting() {
        let temp_dir = temp_dir();
        let base_path = temp_dir.join(".fatherbox");
        let file_path = &base_path.join("test-setting-chat-api.sqlite");
        if exist_database_file(file_path) {
            drop_database_file(&file_path).unwrap();
        }
        let db = init_connection(&file_path).await.unwrap();
        let builder = db.get_database_backend();
        let schema = Schema::new(builder);
        db.execute(builder.build(&schema.create_table_from_entity(entity::prelude::Setting)))
            .await
            .unwrap();
        // get setting
        let response = get_chat_api_setting(&db).await;
        if response.is_error() {
            panic!("{:?}", response.message);
        }
        let mut result = response.result;
        assert_eq!(3, result.len());
        result.push(ChatApiSetting{
            name: "test".to_string(),
            url: "http://www.black-star.com".to_string(),
            key: "".to_string(),
            enable: false,
            is_sync: false,
        });
        // update setting
        let response1 = insert_or_update_chat_api_setting(&db, &result).await;
        if response1.is_error() {
            panic!("{:?}", response.message);
        }
        assert!(response1.result.unwrap());
        // get setting again
        let response2 = get_chat_api_setting(&db).await;
        if response2.is_error() {
            panic!("{:?}", response2.message);
        }
        let result = response2.result;
        assert_eq!(4, result.len());
    }
}
