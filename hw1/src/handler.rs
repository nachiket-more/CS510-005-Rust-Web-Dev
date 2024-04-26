// handler.rs
use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

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

pub async fn get_question_by_id_handler(Path(id): Path<String>) -> impl IntoResponse {
    let db = crate::database::DATABASE.read().unwrap();

    // Find the question by ID
    let question = db.iter().find(|item| item.id == id).cloned();
    
    match question {
        Some(question) => Ok(Json(question)),
        None => {
            let json_response = serde_json::json!({
                "error": "Question not found"
            });
            Err((StatusCode::NOT_FOUND, Json(json_response)))
        }
    }
      
}