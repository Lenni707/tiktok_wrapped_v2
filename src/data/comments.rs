use std::collections::HashMap;

use serde_json::Value;
use time::{Date, Duration, PrimitiveDateTime};
use time::macros::{date, time};


use crate::helper_func::string_to_time;

pub struct Comments {
    num_of_comments: usize,
    all_comments: HashMap<PrimitiveDateTime, Comment>
}

struct Comment {
    date: PrimitiveDateTime,
    message: String,
}
// The format, idk what urs is...
// "date": "2025-09-28 15:22:14",
// "comment": "🥰🥰🥰",
// "photo": "N/A",
// "video": "N/A",
// "url": ""

impl Comments {
    pub fn new(data: &Value)  {

    }
}

fn get_comments(data: &Value) -> HashMap<PrimitiveDateTime, Comment> {
    let mut comments: HashMap<PrimitiveDateTime, Comment> = HashMap::new();
    let mut last_comment: Vec<PrimitiveDateTime> = Vec::new();

    let all_comments = data
        .get("Comment")
        .and_then(|v| v.get("Comments"))
        .and_then(|v| v.get("CommentsList"))
        .and_then(|v| v.as_array());

    if let Some(array) = all_comments {
        for item in array {
            // let mut date: Option<PrimitiveDateTime> = None;
            // let mut message: Option<String> = None;

            if let Some(curr) = item.get("date").and_then(|v| v.as_str()) {
                let date = string_to_time(curr);

                if let Some(message) = item.get("comment").and_then(|v| v.as_str()) {
                    let new_Comment = Comment {
                        date: date,
                        message: message.to_string()
                    };
                }
            }
        }

    }
    comments
}