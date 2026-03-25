use time::PrimitiveDateTime;
use time::macros::format_description;

pub fn string_to_time(string_time: &str) -> PrimitiveDateTime { // converts time from a string to a DateTime type which you can do calculations and stuff with
    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"); // high key tuff die crate so geil crazy

    PrimitiveDateTime::parse(string_time, &format)
        .expect("parse failed")
}