use std::sync::RwLock;
use lazy_static::lazy_static;


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
    let mut db = DATABASE.write().unwrap();
    db.push(models::Question {
        id: "1".to_string(),
        title: "First Question".to_string(),
        content: "Content of question".to_string(),
        tags: vec!["faq".to_string()],
    });
    
    db.push(models::Question {
        id: "2".to_string(),
        title: "Second Question".to_string(),
        content: "Content of question".to_string(),
        tags: vec!["faq".to_string()],
    });
}
