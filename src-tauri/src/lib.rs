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
pub struct AppResponse<T> {
    pub code: i32,
    pub r#type: String,
    pub message: String,
    pub result:T
}

pub static RESPONSE_CODE_SUCCESS: i32 = 0;
pub static RESPONSE_CODE_ERROR: i32 = -1;
pub static RESPONSE_CODE_TIMEOUT: i32 = 401;

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
