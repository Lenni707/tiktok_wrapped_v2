use std::fs::File;
use std::io::BufReader;
use std::collections::HashSet;
use std::time::Duration;
use serde_json::Value;

mod data;
use data::user::User;

mod helper_func;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("user_data_tiktok.json")?;
    // iwie schneller oder
    let reader = BufReader::new(file);

    let data: Value = serde_json::from_reader(reader)?;

    let user = User::new(&data);

    println!("You had {:?} individual watch sessions", user.activity.watch_sessions.len());

    let mut whole_time: Duration = Duration::new(0, 0);

    for i in user.activity.watch_sessions {
        whole_time += i.duration
    }

    println!("You spend approximently {:?} Days on TikTok", whole_time.as_secs_f32() / 60. / 60./ 24.);

    // println!("{:?}", &data["Your Activity"]["Watch History"]["VideoList"][1]);

    drop(data); // disposes of the data to safe memory. Maybe dumm aber ich kopiere für mein eigenes immer aus dem originalen raus

    Ok(())
}

fn get_keys(v: &Value) -> HashSet<String> {
    let map = v.as_object().expect("expect so abfuck");
    map.keys().cloned().collect() // .cloned macht aus &String -> String, .collect ist goated und bildet aus igerator (map.keys()), was festes wie HashSet (HashSet wegen Return type)
}
