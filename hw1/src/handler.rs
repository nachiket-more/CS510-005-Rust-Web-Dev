// handler.rs
use axum::{
    response::IntoResponse,
    Json,
};

// use crate::database::models::Question;


pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Build Simple CRUD API in Rust using Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn get_questions_handler() -> impl IntoResponse {
    let db = crate::database::DATABASE.read().unwrap();
    Json(db.clone())
}
