use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Blog {
    pub title: String,
    pub sub_title: String
}