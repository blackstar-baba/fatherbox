// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use clap::Parser;
use config::Case::ScreamingSnake;
use config::FileFormat;
use log::info;
use serde::Serialize;
use serde_json::from_slice;
use tauri::api::http::{ClientBuilder, HttpRequestBuilder, ResponseType};

use app::{AppResponse, Config, ModelData, RESPONSE_CODE_SUCCESS};
use app::api::Api;

static DEFAULT_CONFIG: &str = include_str!("../config.toml");

#[derive(Parser)]
#[command(version)]
#[command(name = "fb")]
#[command(about = "Father Box .")]
#[command(author = "blackstar-baba <535650957@qq.com>")]
struct Args {
    /// path to config file
    #[arg(short, long)]
    config: Option<String>,
    /// log level (v: info, vv: debug, vvv: trace)
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
    verbose: u8,
}

fn banner() {
    const B: &str = r"
        Father Box
    ";
    println!("{B}\n");
}

fn main() {
    // show banner
    banner();

    // process args
    let args: Args = Args::parse();

    let level = match args.verbose {
        0 => "info",
        1 => "debug",
        2 => "trace",
        _ => "",
    };

    // init tracing
    tracing_subscriber::fmt()
        .pretty()
        .with_line_number(false)
        .with_file(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_env_filter(level)
        .init();

    // process config
    let mut config_builder = config::Config::builder();

    config_builder = match &args.config {
        Some(config) => config_builder.add_source(config::File::with_name(config)),
        None => {
            info!("System use build-in config");
            config_builder.add_source(config::File::from_str(DEFAULT_CONFIG, FileFormat::Toml))
        }
    };
    let config: Config = config_builder.build().unwrap().try_deserialize().unwrap();

    tauri::Builder::default()
        .setup(|_app|{
            // start api
            let api_settings = config.api.unwrap();
            let mut api = Api::new(api_settings);
            let api_handle = api.start().unwrap();
            // Ok(api_handle.join().unwrap())
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![my_custom_command])
        .invoke_handler(tauri::generate_handler![ollama_get_models])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn my_custom_command() -> String {
    println!("I was invoked from JS!");
    return String::from("Hello, world!");
}

#[tauri::command]
async fn ollama_get_models() -> AppResponse<ModelData> {
    let client = ClientBuilder::new().build().unwrap();
    let request = HttpRequestBuilder::new("GET", "http://localhost:11434/api/tags")
        .unwrap()
        .response_type(ResponseType::Json);
    if let Ok(response) = client.send(request).await {
        let data = response.read().await.unwrap().data;
        // println!("{}",data);
        let model_data: ModelData = serde_json::from_value(data).unwrap();
        return AppResponse {
            code:  RESPONSE_CODE_SUCCESS,
            r#type: "".to_string(),
            message: "".to_string(),
            result: model_data,
        };
    }
    let model_data = ModelData { models: vec![] };
    return AppResponse {
        code:  RESPONSE_CODE_SUCCESS,
        r#type: "".to_string(),
        message: "".to_string(),
        result: model_data,
    };
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[deny()]
    pub struct ModelDataNew {
        pub models: String,
    }

    #[test] //由此判断这是一个测试函数
    fn it_works() {
        // let results = get_images();
        // assert_eq!(2, results.len())
    }

    #[test] //由此判断这是一个测试函数
    fn test_load_file() {

    }
}
