use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use serde_json::Value;
use time::PrimitiveDateTime;

use crate::{data, helper_func::string_to_time};

#[derive(Serialize, Deserialize)]
pub struct Likes {
    pub count_liked_vids: usize
}

#[derive(Serialize, Deserialize)]
pub struct Video {
    pub date: PrimitiveDateTime,
    pub link: String,
}

impl Likes {
    pub fn new(data: &Value) -> Self {
        let liked_vids = get_liked_videos(data);
        Self {
            count_liked_vids: liked_vids.len()
        }
    }
}

fn get_liked_videos(data: &Value) -> HashMap<PrimitiveDateTime, Video> {
    let mut liked_videos = HashMap::new();

    let vec_liked_vids = data.get("Likes and Favorites")
        .and_then(|v| v.get("Like List"))
        .and_then(|v| v.get("ItemFavoriteList"))
        .and_then(|v| v.as_array());

    if let Some(array) = vec_liked_vids {
        for vid in array {

            if let Some(curr) = vid.get("date").and_then(|v| v.as_str()) {
                let curr_date = string_to_time(curr);

                if let Some(link) = vid.get("link").and_then(|v| v.as_str()) {
                    let new_liked = Video {
                        date: curr_date,
                        link: link.to_string()
                    };
                    // Das ist irgendwie goofy, aber ig wir können nach dem Datum indexen
                    liked_videos.insert(curr_date, new_liked); 
                }
            }
        }
    }
    liked_videos
}