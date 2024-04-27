use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};

use serde_json::Value;

/// Health check handler.
/// This function returns a JSON response indicating that the API is running.
pub async fn health_checker_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "success",
        "message": "Build Simple CRUD API in Rust using Axum"
    }))
}

/// Get all questions handler.
/// This function retrieves all questions from the database and returns them as a JSON response.
pub async fn get_questions_handler() -> impl IntoResponse {
    // Acquire a read lock on the database
    let db = crate::database::DATABASE.read().unwrap();
    // Return the entire database as a JSON response
    Json(db.clone())
}

/// Get a question by ID handler.
/// This function finds a question by its ID and returns it as a JSON response.
pub async fn get_question_by_id_handler(Path(id): Path<String>) -> impl IntoResponse {
    let db = crate::database::DATABASE.read().unwrap();
    let question = db.iter().find(|item| item.id == id).cloned();

    // If the question is found, return it as a JSON response
    // Otherwise, return a 404 Not Found error
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

/// Insert a new question handler.
/// This function creates a new question based on the provided payload and adds it to the database.
pub async fn insert_question_handler(Json(payload): Json<Value>) -> impl IntoResponse {
    let mut db = crate::database::DATABASE.write().unwrap();
    // Check if the payload contains the required fields
    if payload.get("id").is_none()
        || payload.get("title").is_none()
        || payload.get("content").is_none()
        || payload.get("tags").is_none()
    {
        let json_response = serde_json::json!({
            "error": "Invalid payload. Required fields: id, title, content, tags"
        });
        return Err((StatusCode::BAD_REQUEST, Json(json_response)));
    }

    // Extract the required fields from the payload
    let id = payload["id"].as_str().unwrap().to_string();
    let title = payload["title"].as_str().unwrap().to_string();
    let content = payload["content"].as_str().unwrap().to_string();
    let tags: Vec<String> = payload["tags"]
        .as_array()
        .unwrap()
        .iter()
        .map(|tag| tag.as_str().unwrap().to_string())
        .collect();

    // Check if a question with the given ID already exists
    if db.iter().any(|item| item.id == id) {
        let json_response = serde_json::json!({
            "error": "Question with the given ID already exists"
        });
        return Err((StatusCode::CONFLICT, Json(json_response)));
    }

    // Create a new question and add it to the database
    let new_question = crate::database::models::Question {
        id,
        title,
        content,
        tags,
    };
    db.push(new_question);
    let json_response = serde_json::json!({
        "message": "Question created successfully"
    });
    Ok(Json(json_response))
}

/// Delete a question handler.
/// This function deletes a question from the database by its ID.
pub async fn delete_question_handler(Path(id): Path<String>) -> impl IntoResponse {
    let mut db = crate::database::DATABASE.write().unwrap();
    let id = db.iter().position(|item| item.id == id);

    match id {
        Some(id) => {
            // Remove the question from the database
            db.remove(id);

            let json_response = serde_json::json!({
                "message": "Question deleted successfully"
            });
            Ok(Json(json_response))
        }
        None => {
            let json_response = serde_json::json!({
                "error": "Question not found"
            });
            Err((StatusCode::NOT_FOUND, Json(json_response)))
        }
    }
}

/// Update a question handler.
/// This function updates an existing question in the database based on the provided payload.
pub async fn update_question_handler(
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let mut db = crate::database::DATABASE.write().unwrap();
    let question_index = db.iter().position(|item| item.id == id);

    match question_index {
        Some(index) => {
            let mut updated_question = db[index].clone();
            // Update the fields based on the payload
            if let Some(title) = payload.get("title") {
                updated_question.title = title.as_str().unwrap().to_string();
            }
            if let Some(content) = payload.get("content") {
                updated_question.content = content.as_str().unwrap().to_string();
            }
            if let Some(tags) = payload.get("tags") {
                updated_question.tags = tags
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|tag| tag.as_str().unwrap().to_string())
                    .collect();
            }

            // Replace the original question with the updated question
            db[index] = updated_question;

            let updated_question_response = db[index].clone();
            let json_response = serde_json::json!({
                "message": "Question updated successfully",
                "updated_question": updated_question_response
            });
            Ok(Json(json_response))
        }
        None => {
            let json_response = serde_json::json!({
                "error": "Question not found"
            });
            Err((StatusCode::NOT_FOUND, Json(json_response)))
        }
    }
}