use majority::{Poll, Polls};
use crate::{config::CONFIG, utils::lrm};
use itertools::izip;
const VOTE_STR_LEN: usize = 10;
pub struct Majority {
    pub polls: Polls,
}

impl Majority {
    pub fn new() -> Self {
        Majority {
            polls: Polls::new("polls.db").unwrap(),
        }
    }

    pub fn make_messages(&self, poll: Poll) -> Vec<String> {
        izip!(poll.options, poll.votes, poll.ranking).map(|(opt_desc, votes, rank)| {
            let vote_msg: String = if votes.len() > 0 { lrm(VOTE_STR_LEN, &votes)
                .into_iter().enumerate()
                .map(|(i, n)| CONFIG.vote_values[i].repeat(n))
                .collect()
            } else {
                "⬛".repeat(VOTE_STR_LEN)
            };
            format!("{}\n({}) {} ({} votes)", opt_desc, rank, vote_msg, votes.len())
        }).collect()
    }
}
