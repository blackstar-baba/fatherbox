pub mod api;

use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub api: Option<ApiSettings>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiSettings {
    pub listen: SocketAddr,
    pub db_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response<T> {
    pub code: i32,
    pub r#type: String,
    pub message: String,
    pub result:T
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelData {
    pub models: Vec<Model>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    name: String,
    model: String,
    modified_at: String,
    size: i64,
    digest: String,
}