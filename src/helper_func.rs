use time::{PrimitiveDateTime, Date};
use time::macros::format_description;

pub fn string_to_time(string_time: &str) -> PrimitiveDateTime { // converts time from a string to a DateTime type which you can do calculations and stuff with
    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"); // high key tuff die crate so geil crazy

    PrimitiveDateTime::parse(string_time, &format)
        .expect("parse failed")
}


pub fn date_to_nice_date(date: Date) -> String {
    let format = format_description!("[month repr:long] [day] [year]");
    let date_as_string = date.format(&format).unwrap();

    date_as_string
}