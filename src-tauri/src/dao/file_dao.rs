use chrono::Utc;
use sea_orm::prelude::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait,
    PaginatorTrait, QueryFilter, QuerySelect, Value,
};

use crate::dto::file_dto::{ListByPageBody, ListByPidBody, ListGeneralBody, PageResult};
use crate::entity::file::{
    ActiveModel as FileActiveModel, Column, Entity as File, Model as FileModel, Model,
};

pub struct FileService;

impl FileService {
    pub async fn create_file(
        db: &DatabaseConnection,
        file: FileActiveModel,
    ) -> Result<FileModel, DbErr> {
        file.insert(db).await
    }

    pub async fn get_file(db: &DatabaseConnection, id: &str) -> Result<Option<Model>, DbErr> {
        File::find().filter(Column::Id.eq(id)).one(db).await
    }

    pub async fn delete_file(db: &DatabaseConnection, id: &str) -> Result<(), DbErr> {
        if let Some(file) = File::find_by_id(id.to_string()).one(db).await? {
            file.delete(db).await?;
        }
        Ok(())
    }

    pub async fn list_files_by_pid(
        db: &DatabaseConnection,
        body: &ListByPidBody,
    ) -> Result<Vec<FileModel>, DbErr> {
        let mut select = File::find().filter(Column::Pid.eq(&body.pid));
        if body.r#type.is_some() {
            let file_type = body.r#type.clone().unwrap();
            select = select.filter(Column::Type.eq(file_type));
        }
        select.all(db).await
    }

    pub async fn list_files(
        db: &DatabaseConnection,
        body: &ListGeneralBody,
    ) -> Result<Vec<FileModel>, DbErr> {
        let mut select = File::find()
            .filter(Column::Wid.eq(&body.wid))
            .filter(Column::Zone.eq(&body.zone));
        if body.r#type.is_some() {
            let file_type = body.r#type.clone().unwrap();
            select = select.filter(Column::Type.eq(file_type));
        }
        select.all(db).await
    }

    pub async fn list_files_by_page(
        db: &DatabaseConnection,
        body: &ListByPageBody,
    ) -> Result<PageResult, DbErr> {
        let paginator = File::find()
            .filter(Column::Pid.eq(&body.pid))
            .filter(Column::Type.eq(&body.r#type))
            .filter(Column::Name.like(body.name.to_owned() + "%"))
            .paginate(db, body.page_size);
        let total_result = paginator.num_items().await;
        if total_result.is_err() {
            return Err(total_result.err().unwrap());
        }
        let total = total_result.unwrap();
        let page_result = paginator.fetch_page(body.page_num).await;
        if page_result.is_err() {
            return Err(page_result.err().unwrap());
        }
        Ok(PageResult {
            total,
            items: page_result.unwrap(),
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
            .col_expr(
                Column::Name,
                Expr::value(Value::String(Some(Box::from(name.to_string())))),
            )
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

    pub async fn list_workspace_zones(
        db: &DatabaseConnection,
        wid: &str,
    ) -> Result<Vec<String>, DbErr> {
        File::find()
            .select_only()
            .columns([Column::Zone])
            .filter(Column::Wid.eq(wid))
            .distinct()
            .into_tuple()
            .all(db)
            .await
    }
}
