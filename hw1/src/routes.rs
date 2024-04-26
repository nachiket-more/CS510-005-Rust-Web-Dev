// routes.rs
use axum::{
    routing::{get},
    Router,
};

use crate::{
    handler::{health_checker_handler, get_questions_handler, get_question_by_id_handler},
    database,
};

pub fn create_router() -> Router {
    database::seed_database();

    Router::new()
        .route("/", get(health_checker_handler))
        .route("/questions", get(get_questions_handler))
        .route("/question/:id", get(get_question_by_id_handler))
}
