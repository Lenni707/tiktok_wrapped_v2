use std::collections::HashMap;

use serde_json::Value;
use time::{Date, Duration, PrimitiveDateTime};
use time::macros::{date, time};
use time::OffsetDateTime;


use crate::helper_func::string_to_time;

pub struct Comments {
    pub num_of_comments: usize,
    pub all_comments: HashMap<PrimitiveDateTime, Comment> // Das sind aktuell nur die Comments des letzen Jahres, kann geändert werden
}
#[derive(Clone)]
pub struct Comment {
    pub date: PrimitiveDateTime,
    pub message: String,
}
// The format, idk what urs is...
// "date": "2025-09-28 15:22:14",
// "comment": "🥰🥰🥰",
// "photo": "N/A",
// "video": "N/A",
// "url": ""

impl Comments {
    pub fn new(data: &Value) -> Self {
        let comments = get_comments(data);

        let recent_comments = get_last_year(&comments);

        let num_of_comments = recent_comments.len();

        Self {
            num_of_comments: num_of_comments,
            all_comments: recent_comments,
        }
    }
}
fn get_last_year(comments: &HashMap<PrimitiveDateTime, Comment>) -> HashMap<PrimitiveDateTime, Comment> {
    let one_year_ago = OffsetDateTime::now_utc() - Duration::days(365);
    let cutoff = PrimitiveDateTime::new(one_year_ago.date(), time!(00:00:00));

    comments
        .iter()
        .filter(|(date, _)| **date >= cutoff)
        .map(|(date, comment)| (*date, comment.clone()))
        .collect()
}

fn get_comments(data: &Value) -> HashMap<PrimitiveDateTime, Comment> {
    let mut comments: HashMap<PrimitiveDateTime, Comment> = HashMap::new();

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
                    let new_comment = Comment {
                        date: date,
                        message: message.to_string()
                    };
                    comments.insert(date, new_comment);
                }
            }
        }

    }
    comments
}

