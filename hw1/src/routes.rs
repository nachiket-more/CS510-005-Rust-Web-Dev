// routes.rs
use axum::{
    http::StatusCode,
    routing::{get},
    Router,
};

use crate::{
    handler::{health_checker_handler, get_questions_handler},
    database,  // Importing the database module
};

pub fn create_router() -> Router {
    database::seed_database();

    Router::new()
        .route("/", get(health_checker_handler))
        .route("/questions", get(get_questions_handler))
}
