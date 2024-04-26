use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::health_checker_handler,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(health_checker_handler))
}
