use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use serde_json::Value;
use time::{Date, Duration, PrimitiveDateTime};

use crate::helper_func::{string_to_time, date_to_nice_date};

#[derive(Serialize, Deserialize)]
pub struct Likes {
    pub count_liked_vids: usize
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Video {
    pub date: PrimitiveDateTime,
    pub date_as_string: String,
    pub link: String,
}

impl Likes {
    pub fn new(data: &Value) -> Self {
        let liked_vids = get_liked_videos(data);
        
        let recent_likes = get_last_year(&liked_vids);
        
        let num_of_liked = recent_likes.len();

        Self {
            count_liked_vids: num_of_liked
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
                        date_as_string: date_to_nice_date(curr_date.date()),
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