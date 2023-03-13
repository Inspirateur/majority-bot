use majority::{Poll, Polls};

pub struct Majority {
    polls: Polls
}

impl Majority {
    pub fn new() -> Self {
        Majority { polls: Polls::new("polls.db").unwrap() }
    }
}
