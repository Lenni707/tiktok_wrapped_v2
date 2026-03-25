use std::fs::File;
use std::io::BufReader;
use std::collections::HashSet;
use serde_json::Value;

mod data;
use data::user::User;

mod helper_func;
use crate::helper_func::string_to_time;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("user_data_tiktok.json")?;
    // iwie schneller oder
    let reader = BufReader::new(file);

    let data: Value = serde_json::from_reader(reader)?;

    let user = User::new(&data);

    let test = data["Your Activity"]["Watch History"]["VideoList"][0]["Date"].as_str().expect("didnt work");

    let converted_test = string_to_time(test);

    println!("{}", converted_test);

    // println!("{:?}", &data["Your Activity"]["Watch History"]["VideoList"][0]);

    drop(data); // disposes of the data to safe memory. Maybe dumm aber ich kopiere für mein eigenes immer aus dem originalen raus

    Ok(())
}

fn get_keys(v: &Value) -> HashSet<String> {
    let map = v.as_object().expect("expect so abfuck");
    map.keys().cloned().collect() // .cloned macht aus &String -> String, .collect ist goated und bildet aus igerator (map.keys()), was festes wie HashSet (HashSet wegen Return type)
}
