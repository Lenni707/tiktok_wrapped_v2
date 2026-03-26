use std::default;
use std::collections::HashMap;

use serde_json::Value;
use time::{Date, Duration, PrimitiveDateTime};
use time::macros::{date, time};

use crate::helper_func::string_to_time;

pub struct Activity {
    pub watch_sessions_overall: HashMap<PrimitiveDateTime, WatchSession>,
    pub num_watch_sessions_one_year: usize,
    pub longest_watch_session: WatchSession,
    pub watch_time_secs: f32,
    pub vids_watched: usize,
    pub average_time_per_vid: f32,
    pub most_watch_sessions_per_day: (Date, usize),
    pub most_time_spend_on_tiktok_day: (Date, Duration)
}

#[derive(Debug, Clone)]
pub struct WatchSession {
    pub duration: Duration,
    pub start: PrimitiveDateTime, // for on which day u watched tiktok the most
    pub end: PrimitiveDateTime, // for coolness when debuggin
    pub vids_watched: usize,
    pub average_time_per_vid:f32,
}

impl WatchSession {
    fn new(session: Vec<PrimitiveDateTime>) -> Self {
        let start = session[0];
        let end = session[session.len() - 1];
        let duration = end - start;
        let vids_watched = session.len();
        let average_time_per_vid = duration.as_seconds_f32() / vids_watched as f32;

        WatchSession { duration, start, end, vids_watched, average_time_per_vid }
    }
}

impl Activity {
    pub fn new(data: &Value) -> Self {
        let watch_sessions = get_watch_sessions(data);

        let mut watch_time: Duration = Duration::new(0, 0);
        let mut vids_watched: usize = 0;
        let mut longest_watch: WatchSession = WatchSession { duration: Duration::default(), start: PrimitiveDateTime::new(date!(2067-01-01), time!(0:00)), end: PrimitiveDateTime::new(date!(2067-01-01), time!(0:00)), vids_watched: 0, average_time_per_vid: 0.}; // so dumm
        let mut num_watch_sessions_one_year: usize = 0;

        let mut watch_sessions_per_day_hashmap: HashMap<Date, Vec<&WatchSession>> = HashMap::new();

        let mut added_avg_of_time_per_vide_per_watch_session: f32 = 0.;

        let cutoff = watch_sessions.values()
            .map(|s| s.start)
            .max()
            .unwrap() - Duration::days(365);

        for (date, watch_session) in &watch_sessions {
            if *date < cutoff { // if its longe ago than year: problem data has to be fresh or the current date doesnt work
                continue;
            }

            let just_date = date.date();

            if watch_sessions_per_day_hashmap.get(&just_date).is_none() {
                watch_sessions_per_day_hashmap.insert(just_date, vec![watch_session]);
            } else {
                if let Some(watch_session_per_specific_day) = watch_sessions_per_day_hashmap.get_mut(&just_date) {
                    watch_session_per_specific_day.push(watch_session);
                }
            }
            
            num_watch_sessions_one_year += 1;
            watch_time += watch_session.duration;
            vids_watched += watch_session.vids_watched;
            added_avg_of_time_per_vide_per_watch_session += watch_session.average_time_per_vid;
            if watch_session.duration > longest_watch.duration {
                longest_watch = watch_session.clone(); // weil ich ein fauler sack bin
            }
        }

        let mut most_watch_sessions_per_day: (Date, usize) = (date!(2026-01-01), 0); // davor dummy value // rest selbst erklärend
        let mut most_time_spend_on_tiktok_day: (Date, Duration) = (date!(2026-01-01), Duration::new(1, 0));

        for (date, watch_sessions) in watch_sessions_per_day_hashmap {

            let mut time_spend_on_tiktok_that_day = Duration::new(0, 0);
            for session in &watch_sessions {
                time_spend_on_tiktok_that_day += session.duration;
            }
            if time_spend_on_tiktok_that_day > most_time_spend_on_tiktok_day.1 {
                most_time_spend_on_tiktok_day = (date, time_spend_on_tiktok_that_day);
            }

            if watch_sessions.len() > most_watch_sessions_per_day.1 {
                most_watch_sessions_per_day = (date, watch_sessions.len())
            }
        }

        Activity {
            watch_sessions_overall: watch_sessions,
            num_watch_sessions_one_year,
            longest_watch_session: longest_watch,
            watch_time_secs: watch_time.as_seconds_f32(),
            vids_watched,
            average_time_per_vid: added_avg_of_time_per_vide_per_watch_session / num_watch_sessions_one_year as f32,
            most_watch_sessions_per_day,
            most_time_spend_on_tiktok_day
        }
    }
}

pub fn get_watch_sessions(data: &Value)-> HashMap<PrimitiveDateTime, WatchSession> { 
        let mut watch_history_hash: HashMap<PrimitiveDateTime, WatchSession> = HashMap::new();
        let mut last_session: Vec<PrimitiveDateTime> = Vec::new();

        let watch_history = data // type Value::Array
            .get("Your Activity")
            .and_then(|v| v.get("Watch History"))
            .and_then(|v| v.get("VideoList"))
            .and_then(|v| v.as_array());


        if let Some(array) = watch_history {
            let mut prev_date: Option<PrimitiveDateTime> = None;

            for item in array {
                if let Some(curr) = item.get("Date").and_then(|v| v.as_str()) {
                    let curr_date = string_to_time(curr);

                    match prev_date {
                        None => { last_session.push(curr_date); }
                        Some(prev) => {
                            if (curr_date - prev).abs() < Duration::new(240, 0) { // adjust for break between vids watched to count as new watch session
                                last_session.push(curr_date)
                            }
                            else {
                                if !last_session.is_empty() {
                                    watch_history_hash.insert(last_session[0],WatchSession::new(std::mem::take(&mut last_session)));
                                }
                            }
                        }
                    }
                    prev_date = Some(curr_date);
                }
            }
            if !last_session.is_empty() {
                watch_history_hash.insert(last_session[0],WatchSession::new(std::mem::take(&mut last_session)));
            }
        }

        watch_history_hash
}
