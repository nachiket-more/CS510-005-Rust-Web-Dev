//! This module manages the database of questions, including seeding the database from a JSON file

use lazy_static::lazy_static;
use std::fs;
use std::sync::RwLock;

pub mod models {
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

/// This function reads the `questions.json` file, deserializes the JSON data into a vector of `Question` structs
pub fn seed_database() {
    // Read the questions.json file
    let questions_json = fs::read_to_string("questions.json").expect("Failed to read questions.json");
    // Deserialize the JSON data into a vector of Question structs
    let questions: Vec<models::Question> = serde_json::from_str(&questions_json).expect("Failed to deserialize questions");
    
    let mut db = DATABASE.write().unwrap();
    db.extend(questions);
}