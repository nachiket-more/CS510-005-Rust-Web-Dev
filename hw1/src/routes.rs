//! This module defines the routes for the CRUD API and maps them to the corresponding handler functions.
// Importing necessary modules and items from the Axum framework
use axum::{
    // routing::{delete, get, patch, post},
    routing::{get, post},
    Router,
};

// Importing necessary items from other modules in the crate
use crate::handler::{
    get_question_by_id_handler,
    // update_question_handler,
    // delete_question_handler,
    // Importing handler module and its functions
    get_questions_handler,
    health_checker_handler,
    insert_question_handler,
};

use sqlx::PgPool;
use std::sync::Arc;

/// Creates the router and defines the routes for the CRUD API.
/// This function sets up the routes and maps them to the corresponding handler functions.
pub fn create_router(pool: Arc<PgPool>) -> Router {
    // database::connect();
    // Creating a new router and defining routes
    Router::new()
        .route("/", get(health_checker_handler))
        .route("/questions", get(get_questions_handler))
        .route("/question/:id", get(get_question_by_id_handler))
        .route("/question", post(insert_question_handler))
        // .route("/question/:id", delete(delete_question_handler))
        // .route("/question/:id", patch(update_question_handler))
        .with_state(pool)
}
