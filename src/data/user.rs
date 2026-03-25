use serde_json::Value;

use crate::data::profile::Profile;
use crate::data::likes::Likes;
use crate::data::activity::Activity;
use crate::data::comments::Comments;

pub struct User {
    pub profile: Profile,
    // likes: Likes,
    // comments: Comments,
    pub activity: Activity
}

impl User {
    pub fn new(data: &Value) -> Self {
        User {
            profile: Profile::new(data),
            activity: Activity::new(data)
        }
    }
}