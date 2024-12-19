use crate::entity::file::{ActiveModel as FileActiveModel, Column, DataTransModel as FileDataTransModel, Entity as File, Model as FileModel, Model};
use chrono::Utc;
use sea_orm::prelude::Expr;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, ModelTrait, PaginatorTrait, QueryFilter, Set, Statement, UpdateResult, Value};
use serde::{Deserialize, Serialize};

pub struct FileService;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PageResult {
    pub total: u64,
    pub items: Vec<FileModel>,
}

impl FileService {
    pub async fn create_file(
        db: &DatabaseConnection,
        file: FileActiveModel,
    ) -> Result<FileModel, DbErr> {
        file.insert(db).await
    }

    pub async fn get_file(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<Model>, DbErr> {
         File::find()
            .filter(Column::Id.eq(id))
            .one(db)
            .await
    }

    pub async fn get_file_include_name(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<FileDataTransModel>, DbErr> {
        let query = Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT file.id, file.name, file.type, file.pid, pfile.name AS parent_file_name,
            file.wid, ws.name AS workspace_name, file.size, file.create_time, file.update_time, file.state
            FROM file
            JOIN workspace ws on file.wid = ws.id
            LEFT JOIN file pfile on file.pid = pfile.id
            WHERE file.id = ?",
            vec![Value::String(Some(Box::new(id.to_owned())))]
        );
        let results = db.query_all(query).await?;
        if results.len() == 0 {
            return Ok(None);
        }
        let model = FileDataTransModel {
            id: results[0].try_get("", "id").unwrap(),
            name: results[0].try_get("", "name").unwrap(),
            r#type: results[0].try_get("", "type").unwrap(),
            pid: results[0].try_get("", "pid").unwrap(),
            parent_file_name: results[0].try_get("", "parent_file_name").unwrap(),
            wid: results[0].try_get("", "wid").unwrap(),
            workspace_name: results[0].try_get("", "workspace_name").unwrap(),
            size: results[0].try_get("", "size").unwrap(),
            create_time: results[0].try_get("", "create_time").unwrap(),
            update_time: results[0].try_get("", "update_time").unwrap(),
            state: results[0].try_get("", "state").unwrap(),
        };
        Ok(Some(model))
    }

    pub async fn update_file(
        db: &DatabaseConnection,
        file: FileActiveModel,
    ) -> Result<Option<FileModel>, DbErr> {
        if let Some(existing_file) = File::find_by_id(file.id.clone().unwrap()).one(db).await? {
            let mut active_model = existing_file.into_active_model();

            active_model.name = file.name.clone();
            active_model.r#type = file.r#type.clone();
            active_model.pid = file.pid.clone();
            active_model.size = file.size.clone();
            active_model.create_time = file.create_time.clone();
            active_model.update_time = file.update_time.clone();
            active_model.state = file.state.clone();

            let updated_file = active_model.update(db).await?;
            Ok(Some(updated_file))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_file(db: &DatabaseConnection, id: &str) -> Result<(), DbErr> {
        if let Some(file) = File::find_by_id(id.to_string()).one(db).await? {
            file.delete(db).await?;
        }
        Ok(())
    }

    pub async fn list_files_by_wid(db: &DatabaseConnection, wid: &str) -> Result<Vec<FileModel>, DbErr> {
        File::find()
            .filter(Column::Wid.eq(wid))
            .all(db)
            .await
    }

    pub async fn list_files_by_wid_and_type(
        db: &DatabaseConnection,
        wid: &str,
        r#type: &str,
    ) -> Result<Vec<FileModel>, DbErr> {
        File::find()
            .filter(Column::Type.eq(r#type))
            .filter(Column::Wid.eq(wid))
            .all(db)
            .await
    }

    pub async fn list_files_by_wid_and_pid(
        db: &DatabaseConnection,
        wid: &str,
        pid: &str,
    ) -> Result<Vec<FileModel>, DbErr> {
        File::find()
            .filter(Column::Wid.eq(wid))
            .filter(Column::Pid.eq(pid))
            .all(db)
            .await
    }

    pub async fn list_files_by_wid_and_pid_and_type(
        db: &DatabaseConnection,
        wid: &str,
        pid: &str,
        r#type: &str,
    ) -> Result<Vec<FileModel>, DbErr> {
        File::find()
            .filter(Column::Wid.eq(wid))
            .filter(Column::Pid.eq(pid))
            .filter(Column::Type.eq(r#type))
            .all(db)
            .await
    }

    pub async fn list_files_by_page(
        db: &DatabaseConnection,
        page_size: u64,
        page_num: u64,
        wid: &str,
        pid: &str,
        r#type: &str,
        name: &str
    ) -> Result<PageResult, DbErr> {
       let paginator=  File::find()
            .filter(Column::Wid.eq(wid))
            .filter(Column::Pid.eq(pid))
            .filter(Column::Type.eq(r#type))
            .filter(Column::Name.like(name.to_owned() + "%"))
            .paginate(db, page_size);
        let total_result = paginator.num_items().await;
        if total_result.is_err() {
            return Err(total_result.err().unwrap())
        }
        let total = total_result.unwrap();
        let page_result = paginator.fetch_page(page_num).await;
        if page_result.is_err() {
            return Err(page_result.err().unwrap())
        }
        Ok(PageResult{
            total,
            items: page_result.unwrap()
        })
    }

    pub async fn update_file_size(
        db: &DatabaseConnection,
        id: &str,
        size: i64,
    ) -> Result<u64, DbErr> {
        match File::update_many()
            .col_expr(Column::Size, Expr::value(Value::BigInt(Some(size))))
            .col_expr(
                Column::UpdateTime,
                Expr::value(Value::BigInt(Some(Utc::now().timestamp()))),
            )
            .filter(Column::Id.eq(id))
            .exec(db)
            .await
        {
            Ok(result) => Ok(result.rows_affected),
            Err(err) => Err(err),
        }
    }

    pub async fn update_file_name(
        db: &DatabaseConnection,
        id: &str,
        name: &str,
    ) -> Result<u64, DbErr> {
        match File::update_many()
            .col_expr(Column::Name, Expr::value(Value::String(Some(Box::from(name.to_string())))))
            .col_expr(
                Column::UpdateTime,
                Expr::value(Value::BigInt(Some(Utc::now().timestamp()))),
            )
            .filter(Column::Id.eq(id))
            .exec(db)
            .await {
            Ok(result) => Ok(result.rows_affected),
            Err(err) => Err(err),
        }
    }
}
