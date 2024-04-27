// routes.rs
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::{
    database,
    handler::{
        delete_question_handler, get_question_by_id_handler, get_questions_handler,
        health_checker_handler, insert_question_handler, update_question_handler,
    },
};

pub fn create_router() -> Router {
    database::seed_database();

    Router::new()
        .route("/", get(health_checker_handler))
        .route("/questions", get(get_questions_handler))
        .route("/question/:id", get(get_question_by_id_handler))
        .route("/question", post(insert_question_handler))
        .route("/question/:id", delete(delete_question_handler))
        .route("/question/:id", patch(update_question_handler))
}
