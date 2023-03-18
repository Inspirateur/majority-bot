use majority::{Poll, Polls};
use serenity::builder::EditMessage;
use crate::{config::CONFIG, utils::lrm};
use itertools::izip;

pub struct Majority {
    pub polls: Polls,
}

impl Majority {
    pub fn new() -> Self {
        Majority {
            polls: Polls::new("polls.db").unwrap(),
        }
    }

    pub fn make_message(&self, poll: Poll, msg: &mut EditMessage) {
        for (opt_desc, votes, rank) in izip!(poll.options, poll.votes, poll.ranking) {
            let votes_char = lrm(10, &votes);
            let vote_msg: String = votes_char.into_iter().enumerate()
                .map(|(i, n)| CONFIG.vote_values[i].repeat(n)).collect();
            msg.content(format!("{}\n({}) {}", opt_desc, rank, vote_msg));
            // TODO: Add discord buttons for voting here
        }
    }
}
