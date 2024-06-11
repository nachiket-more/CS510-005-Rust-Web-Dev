//! This is the entry point of the application, which sets up the server and starts the Axum application.

mod database;
mod handler;
mod routes;

use database::connect;
use routes::create_router;
use std::sync::Arc;
// use axum::extract::State;
use dotenv::dotenv;

/// This function is marked as `#[tokio::main]` to enable async execution using the Tokio runtime.
#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables from .env file

    // Create the connection pool
    let pool = Arc::new(connect().await);

    // Create the router with the connection pool as state
    let app = create_router(pool.clone());

    println!("Server started successfully at localhost:5000");

    // Binding TCP listener to port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();

    // Serving the application using Axum
    axum::serve(listener, app).await.unwrap();
}
