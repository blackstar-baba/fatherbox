// src/service.rs
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, ModelTrait, ColumnTrait, QueryFilter, IntoActiveModel};
use crate::entity::workspace;
use crate::entity::workspace::{ActiveModel as WorkspaceActiveModel, Model as WorkspaceModel, Entity as Workspace};

pub struct WorkspaceService;

impl WorkspaceService {
    pub async fn create_workspace(db: &DatabaseConnection, workspace: WorkspaceActiveModel) -> Result<WorkspaceModel, sea_orm::DbErr> {
        workspace.insert(db).await
    }

    pub async fn get_workspace(db: &DatabaseConnection, id: &str) -> Result<Option<WorkspaceModel>, sea_orm::DbErr> {
        Workspace::find_by_id(id.to_string()).one(db).await
    }

    pub async fn get_workspace_by_name(db: &DatabaseConnection, name: &str) -> Result<Option<WorkspaceModel>, sea_orm::DbErr> {
        Workspace::find().filter(workspace::Column::Name.eq(name)).one(db).await
    }

    pub async fn update_workspace(db: &DatabaseConnection, workspace: WorkspaceActiveModel) -> Result<Option<WorkspaceModel>, sea_orm::DbErr> {
        if let Some(existing_workspace) = Workspace::find_by_id(workspace.id.clone().unwrap()).one(db).await? {
            let mut active_model = existing_workspace.into_active_model();

            active_model.name = workspace.name.clone();
            active_model.create_time = workspace.create_time.clone();
            active_model.update_time = workspace.update_time.clone();
            active_model.state = workspace.state.clone();

            let updated_workspace = active_model.update(db).await?;
            Ok(Some(updated_workspace))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_workspace(db: &DatabaseConnection, id: &str) -> Result<(), sea_orm::DbErr> {
        if let Some(workspace) = Workspace::find_by_id(id.to_string()).one(db).await? {
            workspace.delete(db).await?;
        }
        Ok(())
    }

    pub async fn delete_workspace_by_name(db: &DatabaseConnection, name: &str) -> Result<(), sea_orm::DbErr> {
        if let Some(workspace) = Workspace::find().filter(workspace::Column::Name.eq(name)).one(db).await? {
            workspace.delete(db).await?;
        }
        Ok(())
    }

    pub async fn list_workspaces(db: &DatabaseConnection) -> Result<Vec<WorkspaceModel>, sea_orm::DbErr> {
        Workspace::find().all(db).await
    }
}