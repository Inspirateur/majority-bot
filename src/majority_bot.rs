use majority::{Polls};
use serenity_utils::DBMap;

pub struct Majority {
    pub polls: Polls,
    pub msg_map: DBMap
}

impl Majority {
    pub fn new() -> Self {
        Majority {
            polls: Polls::new("polls.db").unwrap(),
            msg_map: DBMap::new("msg_map.db").unwrap()
        }
    }
}
