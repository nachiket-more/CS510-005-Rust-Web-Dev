mod database;
mod handler;
mod routes;

use routes::create_router;

#[tokio::main]
async fn main() {
    let app = create_router();

    println!("Server started successfully at localhost:8000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}