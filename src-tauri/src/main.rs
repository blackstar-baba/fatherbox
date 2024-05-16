// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use serde::Serialize;
use serde_json::from_slice;
use tauri::api::http::{ClientBuilder, HttpRequestBuilder, ResponseType};
use app::{ModelData, Response};


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

    banner();

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


    tauri::Builder::default()
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
async fn ollama_get_models() -> Response<ModelData> {
    let client = ClientBuilder::new().build().unwrap();
    let request = HttpRequestBuilder::new("GET", "http://localhost:11434/api/tags").unwrap()
        .response_type(ResponseType::Json);
    if let Ok(response) = client.send(request).await {
        let data = response.read().await.unwrap().data;
        // println!("{}",data);
        let model_data: ModelData = serde_json::from_value(data).unwrap();
        return Response{
            code: 0,
            r#type: "".to_string(),
            message: "".to_string(),
            result: model_data,
        };
    }
    let model_data = ModelData {
        models: vec![],
    };
    return Response {
        code: 0,
        r#type: "".to_string(),
        message: "".to_string(),
        result: model_data
    };
}


#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use app::{Model, ModelData};

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
}
