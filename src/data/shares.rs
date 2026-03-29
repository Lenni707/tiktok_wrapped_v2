use std::collections::HashMap;

use serde_json::Value;
use time::PrimitiveDateTime;

use crate::{helper_func::string_to_time};

pub struct Shares{
    pub count_shared_vids: usize
}

pub struct Video {
    pub date: PrimitiveDateTime,
    pub link: String,
}

impl Shares {
    pub fn new(data: &Value) -> Self {
        let shared_vids = get_shared_videos(data);
        Self {
            count_shared_vids: shared_vids.len()
        }
    }
}

fn get_shared_videos(data: &Value) -> HashMap<PrimitiveDateTime, Video> {
    let mut liked_videos = HashMap::new();

    let vec_shared_vids = data.get("Your Activity")
        .and_then(|v| v.get("Share History"))
        .and_then(|v| v.get("ShareHistoryList"))
        .and_then(|v| v.as_array());

    if let Some(array) = vec_shared_vids {
        for vid in array {

            if let Some(curr) = vid.get("Date").and_then(|v| v.as_str()) {
                let curr_date = string_to_time(curr);

                if let Some(link) = vid.get("Link").and_then(|v| v.as_str()) {
                    let new_shared = Video {
                        date: curr_date,
                        link: link.to_string()
                    };
                    // Das ist irgendwie goofy, aber ig wir können nach dem Datum indexen
                    liked_videos.insert(curr_date, new_shared); 
                }
            }
        }
    }
    liked_videos
}
