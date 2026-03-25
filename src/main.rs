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

    taunt(user);

    // println!("{:?}", &data["Your Activity"]["Watch History"]["VideoList"][1]); // for later expansion of data that gets processed

    drop(data); // disposes of the data to safe memory. Maybe dumm aber ich kopiere für mein eigenes immer aus dem originalen raus

    Ok(())
}

fn get_keys(v: &Value) -> HashSet<String> {
    let map = v.as_object().expect("expect so abfuck");
    map.keys().cloned().collect() // .cloned macht aus &String -> String, .collect ist goated und bildet aus igerator (map.keys()), was festes wie HashSet (HashSet wegen Return type)
}

fn taunt(user: User) {
    println!("You had {:?} individual watch sessions.", user.activity.num_watch_sessions_one_year);

    println!("You spend approximently {:?} Days on TikTok.", (user.activity.watch_time_secs / 60. / 60. / 24.) as u32);

    println!("That are {} hours per day... Fuck bro get a life.", ((user.activity.watch_time_secs / 60. / 60. / 24.) / 365.)* 24.);

    println!("During this time you watched {} Videos.", user.activity.vids_watched);

    println!("At your peak you watched TikTok for a straight {} hours. This atrocity started at {} and ended at {}.", user.activity.longest_watch_session.duration.as_seconds_f32() / 60. / 60., user.activity.longest_watch_session.start, user.activity.longest_watch_session.end);

    println!("Thats really much, keeping in mind that your average session length is {} minutes.", (user.activity.watch_time_secs as usize / user.activity.num_watch_sessions_one_year) / 60);

    println!("Look how cooked your attentionspan is, you spend only around {} seconds per video.", user.activity.average_time_per_vid as u32);
}