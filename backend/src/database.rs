use sqlx::{PgPool, Pool, Postgres};
use std::env;

pub mod models {
    use serde::{Deserialize, Serialize};
    use sqlx::FromRow;

    #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
    pub struct Question {
        pub id: i32,
        pub title: String,
        pub content: String,
        pub tags: Vec<String>,
    }
}

pub async fn connect() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}
