use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use serde::{Deserialize, Serialize};

use crate::dao::ai_source_dao::AiConnectionService;
use crate::dto::ai_source::{CreateBody, EnableBody, UpdateBody};
use crate::entity::ai_source::{ActiveModel, Model};
use crate::{AppResponse, BUILD_IN_CONNECTION_NAMES};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GeneralBody {
    pub key: String,
}

fn is_build_in(name: &str) -> bool {
    BUILD_IN_CONNECTION_NAMES.contains(&name)
}

pub async fn create(
    db: &DatabaseConnection,
    body: &CreateBody,
) -> AppResponse<Option<Model>> {
    // is build-in
    if is_build_in(&body.name) {
        return AppResponse::error(None, "build-in connection cannot be create");
    }
    let active_model = ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        name: Set(body.name.clone()),
        build_in: Set(false),
        url: Set(body.url.clone()),
        key: Set(body.key.clone()),
        enable: Set(true),
        sync: Set(false),
        create_time: Set(Utc::now().timestamp()),
        update_time: Set(Utc::now().timestamp()),
        state: Set(1),
        ..Default::default()
    };
    match AiConnectionService::create(db, active_model).await {
        Ok(model) => AppResponse::success(Some(model)),
        Err(err) => AppResponse::error(None, &err.to_string()),
    }
}

pub async fn enable(
    db: &DatabaseConnection,
    body: &EnableBody,
) -> AppResponse<Option<Model>> {
    // is build-in
    let active_model = ActiveModel {
        id: Set(body.id.clone()),
        enable: Set(body.enable.clone()),
        ..Default::default()
    };
    match AiConnectionService::update(db, active_model).await {
        Ok(model) => AppResponse::success(Some(model)),
        Err(err) => AppResponse::error(None, &err.to_string()),
    }
}

pub async fn list(db: &DatabaseConnection) -> AppResponse<Vec<Model>> {
    match AiConnectionService::list(db).await {
        Ok(model) => AppResponse::success(model),
        Err(err) => AppResponse::error(vec![], &err.to_string()),
    }
}

pub async fn get(db: &DatabaseConnection, id: &str) -> AppResponse<Option<Model>> {
    match AiConnectionService::get(db, id).await {
        Ok(model) => AppResponse::success(model),
        Err(err) => AppResponse::error(None, &err.to_string()),
    }
}

pub async fn update(
    db: &DatabaseConnection,
    body: &UpdateBody,
) -> AppResponse<Option<Model>> {
    let mut active_model = ActiveModel {
        id: Set(body.id.clone()),

        key: Set(body.key.clone()),
        ..Default::default()
    };
    // is build-in
    if !is_build_in(&body.name) {
        active_model.name = Set(body.name.clone());
        active_model.url = Set(body.url.clone());
    }
    match AiConnectionService::update(db, active_model).await {
        Ok(model) => AppResponse::success(Some(model)),
        Err(err) => AppResponse::error(None, &err.to_string()),
    }
}

pub async fn delete(db: &DatabaseConnection, id: &str) -> AppResponse<Option<Model>> {
    match AiConnectionService::delete(db, id).await {
        Ok(()) => AppResponse::success(None),
        Err(err) => AppResponse::error(None, &err.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use std::env::temp_dir;

    use crate::dto::ai_source::{CreateBody, UpdateBody};
    use crate::entity;
    use crate::service::ai_source_service::{
        create, delete, get, update,
    };
    use crate::util::db_util::{drop_database_file, exist_database_file, init_connection};
    use sea_orm::{ConnectionTrait, Schema};
    use tauri::Manager;

    #[tokio::test]
    async fn test_ai_source() {
        let temp_dir = temp_dir();
        let base_path = temp_dir.join(".fatherbox");
        let file_path = &base_path.join("test-ai-source.sqlite");
        if exist_database_file(file_path) {
            drop_database_file(&file_path).unwrap();
        }
        let db = init_connection(&file_path).await.unwrap();
        let builder = db.get_database_backend();
        let schema = Schema::new(builder);
        db.execute(builder.build(&schema.create_table_from_entity(entity::prelude::AiSource)))
            .await
            .unwrap();

        let name = "connection1";
        let key = "sn-aaa";
        let url = "https://xxxx.aaa.com";
        // begin invoke
        // 1. test create
        let result = create(
            &db,
            &CreateBody {
                name: name.to_string(),
                key: key.to_string(),
                url: url.to_string(),
            },
        )
        .await;
        if result.is_error() {
            panic!("{:?}", result.message);
        }
        let model = result.result.unwrap();
        let id = &model.id;
        assert_eq!(name, model.name);
        assert_eq!(key, model.key);
        assert_eq!(url, model.url);
        // 2. test get
        let result = get(&db, id).await;
        if result.is_error() {
            panic!("{:?}", result.message);
        }
        let model = result.result.unwrap();
        assert_eq!(name, model.name);
        assert_eq!(key, model.key);
        assert_eq!(url, model.url);
        // 3. test update
        let new_name = "connection2";
        let new_key = "sn-bbb";
        let new_url = "https://xxxx.bbb.com";

        let result = update(
            &db,
            &UpdateBody {
                id: id.to_string(),
                name: new_name.to_string(),
                key: new_key.to_string(),
                url: new_url.to_string(),
            },
        )
        .await;
        if result.is_error() {
            panic!("{:?}", result.message);
        }
        let model = result.result.unwrap();
        assert_eq!(new_name, model.name);
        assert_eq!(new_key, model.key);
        assert_eq!(new_url, model.url);
        // 4. test delete
        let result = delete(&db, id).await;
        if result.is_error() {
            panic!("{:?}", result.message);
        }
        let result = get(&db, id).await;
        if result.is_error() {
            panic!("{:?}", result.message);
        }
        assert!(result.result.is_none());
    }

    #[tokio::test]
    async fn test_build_in_ai_connection() {
        let temp_dir = temp_dir();
        let base_path = temp_dir.join(".fatherbox");
        let file_path = &base_path.join("test-build-in-ai-connection.sqlite");
        if exist_database_file(file_path) {
            drop_database_file(&file_path).unwrap();
        }
        let db = init_connection(&file_path).await.unwrap();
        let builder = db.get_database_backend();
        let schema = Schema::new(builder);
        db.execute(builder.build(&schema.create_table_from_entity(entity::prelude::AiSource)))
            .await
            .unwrap();

        let name = "DeepSeek";
        let key = "sn-aaa";
        let url = "https://xxxx.aaa.com";
        // begin invoke
        // 1. test create
        let result = create(
            &db,
            &CreateBody {
                name: name.to_string(),
                key: key.to_string(),
                url: url.to_string(),
            },
        )
            .await;
        assert!(result.is_error());
    }
}
