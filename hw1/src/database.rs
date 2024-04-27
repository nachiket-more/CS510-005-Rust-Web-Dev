use lazy_static::lazy_static;
use std::sync::RwLock;
use std::fs;

pub mod models {
    // Make the Question struct public here
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Question {
        pub id: String,
        pub title: String,
        pub content: String,
        pub tags: Vec<String>,
    }
}

lazy_static! {
    pub static ref DATABASE: RwLock<Vec<models::Question>> = RwLock::new(Vec::new());
}

pub fn seed_database() {
    let questions_json = fs::read_to_string("questions.json").expect("Failed to read questions.json");
    let questions: Vec<models::Question> = serde_json::from_str(&questions_json).expect("Failed to deserialize questions");

    let mut db = DATABASE.write().unwrap();
    db.extend(questions);
}
