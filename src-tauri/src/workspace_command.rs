use chrono::Utc;
use log::info;
use sea_orm::ActiveValue::Set;
use sea_orm::ConnectionTrait;
use sea_orm::DatabaseConnection;
use std::fs;
use std::path::PathBuf;
use tauri::State;
use uuid::Uuid;

use app::entity::workspace;
use app::entity::workspace::Model;
use app::service::workspace_service::WorkspaceService;
use app::AppState;
use app::{AppResponse, DIR_TYPE, RESPONSE_CODE_SUCCESS};

use crate::file_command::create_workspace_file_inner;

#[tauri::command]
pub async fn create_workspace_cmd(
    state: State<'_, AppState>,
    workspace: String,
) -> Result<AppResponse<Model>, ()> {
    let workspace_path = &state.workspace_path.join(workspace.clone());
    if !workspace_path.exists() {
        // create workspace
        info!("Create workspace: {}", workspace_path.display());
        fs::create_dir(workspace_path).unwrap();
    }
    let result = create_workspace_inner(&state.conn, workspace.clone()).await;
    if result.is_ok() {
        let app_result = result.clone().unwrap().result;
        let _ = create_workspace_file_inner(
            &state.conn,
            app_result.id,
            "".to_owned(),
            DIR_TYPE.to_owned(),
            app_result.name,
        )
        .await;
    };
    result
}

#[tauri::command]
pub async fn get_workspace_cmd(
    state: State<'_, AppState>,
    id: &str,
) -> Result<AppResponse<Option<Model>>, ()> {
    get_workspace_inner(&state.conn, id).await
}

#[tauri::command]
pub async fn list_workspaces_cmd(
    state: State<'_, AppState>,
) -> Result<AppResponse<Vec<Model>>, ()> {
    list_workspaces_inner(&state.conn).await
}

#[tauri::command]
pub async fn delete_workspace_cmd(
    state: State<'_, AppState>,
    id: &str,
) -> Result<AppResponse<String>, ()> {
    delete_workspace_inner(&state.conn, id).await
}

pub async fn create_workspace_inner(
    db: &DatabaseConnection,
    workspace: String,
) -> Result<AppResponse<Model>, ()> {
    let active_model = workspace::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        name: Set(workspace),
        create_time: Set(Utc::now().timestamp()),
        update_time: Set(Utc::now().timestamp()),
        state: Set(1),
        ..Default::default()
    };
    let model = WorkspaceService::create_workspace(db, active_model)
        .await
        .unwrap();
    Ok(AppResponse {
        code: RESPONSE_CODE_SUCCESS,
        r#type: "".to_string(),
        message: "".to_string(),
        result: model,
    })
}

pub async fn list_workspaces_inner(db: &DatabaseConnection) -> Result<AppResponse<Vec<Model>>, ()> {
    let vec = WorkspaceService::list_workspaces(db).await.unwrap();
    Ok(AppResponse {
        code: RESPONSE_CODE_SUCCESS,
        r#type: "".to_string(),
        message: "".to_string(),
        result: vec,
    })
}

pub async fn get_workspace_inner(
    db: &DatabaseConnection,
    id: &str,
) -> Result<AppResponse<Option<Model>>, ()> {
    let model = WorkspaceService::get_workspace(db, id).await.unwrap();
    Ok(AppResponse {
        code: RESPONSE_CODE_SUCCESS,
        r#type: "".to_string(),
        message: "".to_string(),
        result: model,
    })
}

// pub async fn get_workspace_by_name(
//     db: &DatabaseConnection,
//     name: &str,
// ) -> Result<AppResponse<Option<Model>>, ()> {
//     let model = WorkspaceService::get_workspace_by_name(db, name)
//         .await
//         .unwrap();
//     Ok(AppResponse {
//         code: RESPONSE_CODE_SUCCESS,
//         r#type: "".to_string(),
//         message: "".to_string(),
//         result: model,
//     })
// }

pub async fn delete_workspace_inner(
    db: &DatabaseConnection,
    id: &str,
) -> Result<AppResponse<String>, ()> {
    let model = get_workspace_inner(db, id).await.unwrap().result;
    match model {
        None => Ok(AppResponse {
            code: RESPONSE_CODE_SUCCESS,
            r#type: "".to_string(),
            message: "".to_string(),
            result: "".to_string(),
        }),
        Some(_) => {
            let _ = WorkspaceService::delete_workspace(db, id).await.unwrap();
            Ok(AppResponse {
                code: RESPONSE_CODE_SUCCESS,
                r#type: "".to_string(),
                message: "".to_string(),
                result: "".to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env::temp_dir;

    use sea_orm::{ConnectionTrait, Schema};
    use tauri::Manager;

    use app::entity;

    use crate::db_utils::{drop_database_file, exist_database_file, init_connection};
    use crate::workspace_command::{
        create_workspace_inner, delete_workspace_inner, get_workspace_inner, list_workspaces_inner,
    };

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
        let create_result = create_workspace_inner(&db, "default".to_owned())
            .await
            .unwrap()
            .result;
        assert_eq!("default", create_result.name);
        // 2. test get
        let default_workspace_result = get_workspace_inner(&db, &create_result.id)
            .await
            .unwrap()
            .result;
        // 3. test list
        create_workspace_inner(&db, "abc".to_owned())
            .await
            .unwrap()
            .result;
        let workspaces = list_workspaces_inner(&db).await.unwrap().result;
        assert_eq!(2, workspaces.len());
        assert_eq!("default", workspaces[0].name);
        assert_eq!("abc", workspaces[1].name);
        // 4. test delete
        delete_workspace_inner(&db, &workspaces[0].id)
            .await
            .unwrap();
        let deleted_workspace_result = get_workspace_inner(&db, &workspaces[0].id)
            .await
            .unwrap()
            .result;
        assert_eq!(true, deleted_workspace_result.is_none())
    }
}
