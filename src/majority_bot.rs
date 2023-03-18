use majority::{Poll, Polls};
use serenity::builder::EditMessage;

pub struct Majority {
    pub polls: Polls,
}

impl Majority {
    pub fn new() -> Self {
        Majority {
            polls: Polls::new("polls.db").unwrap(),
        }
    }

    pub fn make_message(&self, poll: Poll, msg: &mut EditMessage) -> &mut EditMessage {
        todo!()
    }
}
