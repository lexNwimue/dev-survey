use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

pub fn survey_routes() -> Router {
    Router::new().route("/surveys", get(get_surveys))
}

pub async fn get_surveys() -> Json<Value> {
    Json(json!({"hello": "lex survey"}))
}

// pub async fn create_survey() -> Json<Value> {

// }
