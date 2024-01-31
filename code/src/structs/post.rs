use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub date: u64,
    pub content: String,
    pub slug: String
}