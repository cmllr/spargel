use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Blog {
    pub title: String,
    pub sub_title: String,
    pub meta: HashMap<String, String>,
    pub url: String
}