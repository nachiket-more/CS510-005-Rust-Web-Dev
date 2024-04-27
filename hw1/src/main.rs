//! This is the entry point of the application, which sets up the server and starts the Axum application.

mod database;
mod handler;
mod routes;

use routes::create_router;

/// This function is marked as `#[tokio::main]` to enable async execution using the Tokio runtime.
#[tokio::main]
async fn main() {
    // Creating router
    let app = create_router();

    println!("Server started successfully at localhost:8000");

    // Binding TCP listener to port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    // Serving the application using Axum
    axum::serve(listener, app).await.unwrap();
}