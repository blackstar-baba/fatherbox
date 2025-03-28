use std::env::temp_dir;
use std::fs::File;
use std::path::PathBuf;
use std::{fs, path};

use sea_orm::entity::prelude::*;
use sea_orm::{Database, DatabaseConnection, Schema};
use crate::entity;

pub async fn init_connection(file_path: &PathBuf) -> Result<DatabaseConnection, DbErr> {
    if !exist_database_file(file_path) {
        let parent_path = file_path.parent().unwrap();
        if !parent_path.exists() {
            fs::create_dir_all(parent_path).unwrap();
        }
        if !file_path.exists() {
            File::create(file_path.clone()).unwrap();
        }
    }
    let db_url = format!("sqlite://{}", file_path.to_string_lossy());
    Database::connect(&db_url).await
}

pub async fn close_connection(db: DatabaseConnection) -> Result<(), DbErr> {
    db.close().await
}

pub fn drop_database_file(file_path: &PathBuf) -> std::io::Result<()> {
    fs::remove_file(file_path)
}

pub fn exist_database_file(file_path: &PathBuf) -> bool {
    file_path.exists()
}

pub async fn init_test_database(db_name: &str, table_names: &Vec<String>) -> Result<DatabaseConnection, DbErr>  {
    let temp_dir = temp_dir();
    let base_path = temp_dir.join(".fatherbox");
    let file_path = &base_path.join(format!("{}.sqlite", db_name));
    println!("file path is: {:?}", file_path);
    if exist_database_file(file_path) {
        drop_database_file(&file_path).unwrap();
    }
    let db = init_connection(&file_path).await?;
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    for tableName in table_names {
        if tableName.eq("ai_connection") {
            db.execute(builder.build(&schema.create_table_from_entity(entity::prelude::AiModel)))
                .await?;
        } else if tableName.eq("ai_model") {
            db.execute(builder.build(&schema.create_table_from_entity(entity::prelude::AiModel)))
                .await?;
        }
    }
    Ok(db)
}


#[cfg(test)]
mod tests {
    use crate::util::db_util::{
        close_connection, drop_database_file, exist_database_file, init_connection,
    };
    use std::env::temp_dir;

    #[tokio::test]
    async fn database_test() {
        let temp_dir = temp_dir();
        let base_path = temp_dir.join(".fatherbox");
        let file_path = &base_path.join("test-db-utils.sqlite");
        println!("file path is: {:?}", file_path);
        if exist_database_file(file_path) {
            drop_database_file(&file_path).unwrap();
        }
        let connection = init_connection(&file_path).await;
        match connection {
            Ok(conn) => {
                println!("connect success: {:?}", conn);
                match conn.ping().await {
                    Ok(_) => {
                        println!("ping success")
                    }
                    Err(err) => {
                        panic!("{}", err)
                    }
                }
                close_connection(conn).await.unwrap();
                println!("close success");
                drop_database_file(&file_path).unwrap();
                assert_eq!(false, exist_database_file(&file_path));
                println!("drop database success");
            }
            Err(err) => {
                panic!("can not get connection:{:?}", err)
            }
        }
    }
}
