use serde_json::Value;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub follower_count: u32,
    pub pfp: String
}

impl Profile {
    pub fn new(data: &Value) -> Self {
        let profile_map = data
            .get("Profile And Settings")
            .and_then(|v| v.get("Profile Info"))
            .and_then(|v| v.get("ProfileMap"));

        let name = profile_map
            .and_then(|v| v.get("displayName"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let follower_count = profile_map
            .and_then(|v| v.get("followerCount"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;

        let pfp = profile_map
            .and_then(|v| v.get("profilePhoto"))
            .and_then(|v| v.as_str())
            .unwrap_or("no photo")
            .to_string();

        Profile {
            name,
            follower_count,
            pfp
        }
    }
}