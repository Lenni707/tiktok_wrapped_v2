use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use serde_json::Value;
use time::{Date, Duration, PrimitiveDateTime};


use crate::{helper_func::string_to_time};

#[derive(Serialize, Deserialize)]
pub struct Shares{
    pub count_shared_vids: usize
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Video {
    pub date: PrimitiveDateTime,
    pub link: String,
}

impl Shares {
    pub fn new(data: &Value) -> Self {
        let shared_vids = get_shared_videos(data);
        
        let recent_shares = get_last_year(&shared_vids);

        Self {
            count_shared_vids: recent_shares.len()
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

fn get_last_year(liked: &HashMap<PrimitiveDateTime, Video>) -> HashMap<PrimitiveDateTime, Video> {
    // Das ist doch cooler so oder? ja aber nur für comments, bei videos macht das andere mehr sinn und maybe geht das mit wasm nicht so gut GEHT NICHT FÜR WASM DESWEGEN SO
    let cutoff = liked.values()
            .map(|s| s.date)
            .max()
            .unwrap() - Duration::days(365);

    // let one_year_ago = most_recent_date - Duration::days(365);
    // let cutoff = PrimitiveDateTime::new(one_year_ago.date(), time!(00:00:00));

    liked // holy vibe code aber macht sinn
        .iter()
        .filter(|(date, _)| **date >= cutoff)
        .map(|(date, comment)| (*date, comment.clone()))
        .collect()
}