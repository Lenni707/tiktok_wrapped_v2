use serde_json::Value;
use time::{PrimitiveDateTime, Duration};

use crate::helper_func::string_to_time;

pub struct Activity {
    pub watch_sessions: Vec<WatchSession>
}

#[derive(Debug)]
pub struct WatchSession {
    pub duration: Duration,
    pub start: PrimitiveDateTime, // for on which day u watched tiktok the most
    pub end: PrimitiveDateTime, // for coolness when debuggin
    pub vids_watched: usize
}

impl WatchSession {
    fn new(session: Vec<PrimitiveDateTime>) -> Self {
        let start = session[0];
        let end = session[session.len() - 1];
        let duration = end - start;
        let vids_watched = session.len();

        WatchSession { duration, start, end, vids_watched }
    }
}

impl Activity {
    pub fn new(data: &Value) -> Self {
        let watch_sessions = get_watch_sessions(data);

        Activity {
            watch_sessions
        }
    }
}

pub fn get_watch_sessions(data: &Value)-> Vec<WatchSession> { 
        let mut watch_list: Vec<WatchSession> = Vec::new();
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
                            if curr_date - prev < Duration::new(300, 0) {
                                last_session.push(curr_date)
                            }
                            else {
                                if !last_session.is_empty() {
                                    watch_list.push(WatchSession::new(std::mem::take(&mut last_session)));
                                }
                            }
                        }
                    }
                    prev_date = Some(curr_date);
                }
            }

            if !last_session.is_empty() {
                watch_list.push(WatchSession::new(std::mem::take(&mut last_session)));
            }
        }
        watch_list
}

// -- TODO --
// durch weatch hisotry loopen
// dann die zeit rausfiltern und convertern (parser in helper func)
// anschließend durch alle zeiten loopen und miteinander vergliechen
// wenn abstand zwischen angeschauten videos >5 min oder so endet eine watch session (eignes struct für jede watchsession mit date watched und watch time (alle abstände der zeiten addiert in der session)) und allees gepseichert im einen array