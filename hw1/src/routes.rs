// routes.rs
use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{health_checker_handler, get_questions_handler, 
        get_question_by_id_handler, insert_question_handler},
    database,
};

pub fn create_router() -> Router {
    database::seed_database();

    Router::new()
        .route("/", get(health_checker_handler))
        .route("/questions", get(get_questions_handler))
        .route("/question/:id", get(get_question_by_id_handler))
        .route("/question", post(insert_question_handler))
        // .route("question/:id",.patch(delete_question_handler))
        // .route("question/:id",.delete(update_question_handler))
}

