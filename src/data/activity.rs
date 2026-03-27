use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use serde_json::Value;
use time::{Date, Duration, PrimitiveDateTime, Weekday};
use time::macros::{date, time};

use crate::helper_func::{string_to_time, date_to_nice_date};

#[derive(Serialize, Deserialize)]
pub struct Activity {
    pub watch_sessions_overall: HashMap<PrimitiveDateTime, WatchSession>,
    pub num_watch_sessions_one_year: usize,
    pub longest_watch_session: BetterFormattedWatchSession,
    pub watch_time_secs: f32,
    pub vids_watched: usize,
    pub average_time_per_vid: f32,
    pub most_watch_sessions_per_day: MostWatchSessionsPerDay,
    pub most_time_spend_on_tiktok_day: TimeSpendOnTiktokDay,
    pub avergae_time_per_weekday: WeekdaysAvgTimeOnTT,
}

#[derive(Serialize, Deserialize)]
pub struct BetterFormattedWatchSession {
    pub date_as_string: String,
    pub duration_as_secs: f32,
    pub session: WatchSession
}

impl BetterFormattedWatchSession {
    fn new(session: WatchSession) -> Self {
        let date_as_string = date_to_nice_date(session.start.date());
        let duration_as_secs = session.duration.as_seconds_f32();

        BetterFormattedWatchSession { 
            date_as_string, 
            duration_as_secs, 
            session 
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MostWatchSessionsPerDay {
    pub date: Date,
    pub date_as_string: String,
    pub count: usize,
}

impl MostWatchSessionsPerDay {
    fn new(date: Date, count: usize) -> Self {
        let date_as_string = date_to_nice_date(date);

        MostWatchSessionsPerDay { 
            date, 
            date_as_string, 
            count 
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TimeSpendOnTiktokDay { // helper struct
    pub date: Date,
    pub date_as_string: String,
    pub duration: Duration,
    pub duration_as_secs: f32
}

impl TimeSpendOnTiktokDay {
    fn new(date: Date, duration: Duration) -> Self {
        let date_as_string = date_to_nice_date(date);

        let duration_as_secs = duration.as_seconds_f32();

        TimeSpendOnTiktokDay {
            date,
            date_as_string,
            duration,
            duration_as_secs,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchSession {
    pub duration: Duration,
    pub duration_as_secs: f32,
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

        WatchSession { duration, duration_as_secs: duration.as_seconds_f32(), start, end, vids_watched, average_time_per_vid }
    }
}

#[derive(Serialize, Deserialize)]
pub struct WeekdaysAvgTimeOnTT {
    pub monday: u32,
    pub tuesday: u32,
    pub wednesday: u32,
    pub thursday: u32,
    pub friday: u32,
    pub saturday: u32,
    pub sunday: u32,

    pub highest_day: String,
    pub highest_value: u32,
}

impl WeekdaysAvgTimeOnTT {
    pub fn new_average_per_weekday( // extreme scheiße, weil das ganze keine hashmap sein kein sondern ein struct sein muss, weil das frontend (js) es sont nicht lesen kann. ich könnte natürlich auch eine funktion machen aber dann müsste ich die exposen und so, kb. Idee von mir, die Ausführung von chatty
        map: HashMap<Weekday, (Duration, u32)>
    ) -> Self {

        let mut result = Self {
            monday: 0,
            tuesday: 0,
            wednesday: 0,
            thursday: 0,
            friday: 0,
            saturday: 0,
            sunday: 0,
            highest_day: String::new(),
            highest_value: 0,
        };

        let mut highest_day = Weekday::Monday;
        let mut highest_value: u32 = 0;

        for (weekday, (duration, count)) in map {
            let avg = if count == 0 {
                0
            } else {
                (duration.as_seconds_f32() / count as f32) as u32
            };

            // assign to struct
            match weekday {
                Weekday::Monday => result.monday = avg,
                Weekday::Tuesday => result.tuesday = avg,
                Weekday::Wednesday => result.wednesday = avg,
                Weekday::Thursday => result.thursday = avg,
                Weekday::Friday => result.friday = avg,
                Weekday::Saturday => result.saturday = avg,
                Weekday::Sunday => result.sunday = avg,
            }

            if avg > highest_value {
                highest_value = avg;
                highest_day = weekday;
            }
        }

        result.highest_day = format!("{:?}", highest_day); // tuffes ding dieses format macht aus jeglichen sachen iwie eine string
        result.highest_value = highest_value;

        result
    }
}

impl Activity {
    pub fn new(data: &Value) -> Self {
        let watch_sessions = get_watch_sessions(data);

        let mut watch_time: Duration = Duration::new(0, 0);
        let mut vids_watched: usize = 0;
        let mut longest_watch: BetterFormattedWatchSession = BetterFormattedWatchSession::new(WatchSession { duration: Duration::default(), duration_as_secs: 0., start: PrimitiveDateTime::new(date!(2067-01-01), time!(0:00)), end: PrimitiveDateTime::new(date!(2067-01-01), time!(0:00)), vids_watched: 0, average_time_per_vid: 0.}); // so dumm
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
            if watch_session.duration > longest_watch.session.duration {
                longest_watch = BetterFormattedWatchSession::new(watch_session.clone()); // weil ich ein fauler sack bin
            }
        }

        let mut most_watch_sessions_per_day: MostWatchSessionsPerDay  = MostWatchSessionsPerDay::new(date!(2026-01-01), 0); // davor dummy value // rest selbst erklärend
        let mut most_time_spend_on_tiktok_day: TimeSpendOnTiktokDay = TimeSpendOnTiktokDay::new(date!(2026-01-01), Duration::new(0, 0));

        let mut watch_sessions_per_weekday_hashmap: HashMap<Weekday, (Duration, u32)> = HashMap::new();

        for (date, watch_sessions) in watch_sessions_per_day_hashmap { // loop through all the dates to get all watchsession for specific date

            let mut time_spend_on_tiktok_that_day = Duration::new(0, 0);
            for session in &watch_sessions { // loop through all sessionsin a day
                time_spend_on_tiktok_that_day += session.duration;
            }

            if time_spend_on_tiktok_that_day > most_time_spend_on_tiktok_day.duration {
                most_time_spend_on_tiktok_day = TimeSpendOnTiktokDay::new(date, time_spend_on_tiktok_that_day);
            }
            if watch_sessions.len() > most_watch_sessions_per_day.count {
                most_watch_sessions_per_day = MostWatchSessionsPerDay::new(date, watch_sessions.len())
            }


            if watch_sessions_per_weekday_hashmap.get(&date.weekday()).is_none() { // sort dates into weekdays with corresponding durration of tiktok watched that day
                watch_sessions_per_weekday_hashmap.insert(date.weekday(), (time_spend_on_tiktok_that_day, 0));
            } else {
                if let Some(watch_session_per_specific_weekday) = watch_sessions_per_weekday_hashmap.get_mut(&date.weekday()) {
                    watch_session_per_specific_weekday.0 += time_spend_on_tiktok_that_day;
                    watch_session_per_specific_weekday.1 += 1;
                }
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
            most_time_spend_on_tiktok_day,
            avergae_time_per_weekday: WeekdaysAvgTimeOnTT::new_average_per_weekday(watch_sessions_per_weekday_hashmap)
        }
    }
}

pub fn get_watch_sessions(data: &Value) -> HashMap<PrimitiveDateTime, WatchSession> { 
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
