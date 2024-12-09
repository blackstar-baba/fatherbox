use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppResponse;
use crate::dao::workspace_dao::WorkspaceService;
use crate::entity::workspace;
use crate::entity::workspace::Model;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateBody {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GeneralBody {
    pub id: String
}


pub async fn create_workspace(db: &DatabaseConnection, name: &str) -> AppResponse<Model> {
    let active_model = workspace::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        name: Set(name.to_string()),
        create_time: Set(Utc::now().timestamp()),
        update_time: Set(Utc::now().timestamp()),
        state: Set(1),
        ..Default::default()
    };
    let model = WorkspaceService::create_workspace(db, active_model)
        .await
        .unwrap();
    AppResponse::success(model)
}

pub async fn list_workspaces(db: &DatabaseConnection) -> AppResponse<Vec<Model>> {
    let vec = WorkspaceService::list_workspaces(db).await.unwrap();
    AppResponse::success(vec)
}

pub async fn get_workspace(db: &DatabaseConnection, id: &str) -> AppResponse<Option<Model>> {
    let model = WorkspaceService::get_workspace(db, id).await.unwrap();
    AppResponse::success(model)
}

pub async fn get_workspace_by_name(
    db: &DatabaseConnection,
    name: &str,
) -> AppResponse<Option<Model>> {
    let model = WorkspaceService::get_workspace_by_name(db, name)
        .await
        .unwrap();
    AppResponse::success(model)
}

pub async fn delete_workspace(db: &DatabaseConnection, id: &str) -> AppResponse<String> {
    let model = get_workspace(db, id).await.result;
    match model {
        None => AppResponse::success("".to_string()),
        Some(_) => {
            let _ = WorkspaceService::delete_workspace(db, id).await.unwrap();
            AppResponse::success("".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env::temp_dir;

    use sea_orm::{ConnectionTrait, Schema};
    use tauri::Manager;

    use crate::entity;
    use crate::service::workspace_service::{
        create_workspace, delete_workspace, get_workspace, list_workspaces,
    };
    use crate::util::db_util::{drop_database_file, exist_database_file, init_connection};

    #[tokio::test]
    async fn test_workspace() {
        let temp_dir = temp_dir();
        let base_path = temp_dir.join(".fatherbox");
        let file_path = &base_path.join("test-workspace.sqlite");
        if exist_database_file(file_path) {
            drop_database_file(&file_path).unwrap();
        }
        let db = init_connection(&file_path).await.unwrap();
        let builder = db.get_database_backend();
        let schema = Schema::new(builder);
        db.execute(builder.build(&schema.create_table_from_entity(entity::prelude::Workspace)))
            .await
            .unwrap();
        // begin invoke
        // 1. test create
        let create_result = create_workspace(&db, "default").await.result;
        assert_eq!("default", create_result.name);
        // 2. test get
        let default_workspace_result = get_workspace(&db, &create_result.id).await.result;
        assert_eq!(false,default_workspace_result.is_none());
        // 3. test list
        create_workspace(&db, "abc").await.result;
        let workspaces = list_workspaces(&db).await.result;
        assert_eq!(2, workspaces.len());
        assert_eq!("default", workspaces[0].name);
        assert_eq!("abc", workspaces[1].name);
        // 4. test delete
        delete_workspace(&db, &workspaces[0].id).await;
        let deleted_workspace_result = get_workspace(&db, &workspaces[0].id).await.result;
        assert_eq!(true, deleted_workspace_result.is_none())
    }
}
