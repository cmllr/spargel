use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub date: u64,
    pub content: String,
    pub month_group: String,
    pub day_group: String
}