use serde_json::Value;
use serde::{Serialize, Deserialize};

use crate::data::profile::Profile;
use crate::data::likes::Likes;
use crate::data::activity::Activity;
use crate::data::comments::Comments;
use crate::data::shares::Shares;


#[derive(Serialize, Deserialize)]
pub struct User {
    pub profile: Profile,
    pub likes: Likes,
    pub comments: Comments,
    pub activity: Activity,
    pub shares: Shares,
}

impl User {
    pub fn new(data: &Value) -> Self {
        User {
            profile: Profile::new(data),
            likes: Likes::new(data),
            comments: Comments::new(data),
            activity: Activity::new(data),
            shares: Shares::new(data),
        }
    }
}