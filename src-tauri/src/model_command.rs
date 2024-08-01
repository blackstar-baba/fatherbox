use app::{AppResponse, ModelData, RESPONSE_CODE_SUCCESS};
use tauri::api::http::{ClientBuilder, HttpRequestBuilder, ResponseType};

#[tauri::command]
pub async fn ollama_get_models_cmd() -> AppResponse<ModelData> {
    let client = ClientBuilder::new().build().unwrap();
    let request = HttpRequestBuilder::new("GET", "http://localhost:11434/api/tags")
        .unwrap()
        .response_type(ResponseType::Json);
    if let Ok(response) = client.send(request).await {
        let data = response.read().await.unwrap().data;
        // println!("{}",data);
        let model_data: ModelData = serde_json::from_value(data).unwrap();
        return AppResponse {
            code: RESPONSE_CODE_SUCCESS,
            r#type: "".to_string(),
            message: "".to_string(),
            result: model_data,
        };
    }
    let model_data = ModelData { models: vec![] };
    return AppResponse {
        code: RESPONSE_CODE_SUCCESS,
        r#type: "".to_string(),
        message: "".to_string(),
        result: model_data,
    };
}
