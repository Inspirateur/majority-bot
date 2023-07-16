use majority::Polls;
use serenity_utils::DBMap;
use crate::pollopt_to_sql::PollOption;

pub struct Majority {
    pub polls: Polls,
    pub msg_map: DBMap<PollOption, u64>
}

impl Majority {
    pub fn new() -> Self {
        Majority {
            polls: Polls::new("polls.db").unwrap(),
            msg_map: DBMap::new("msg_map.db").unwrap()
        }
    }
}
