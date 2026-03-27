use serde_json::Value;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Likes {
    count_liked_vids: u32
}