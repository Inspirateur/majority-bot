use crate::{config::CONFIG, utils::lrm};
use majority::{Poll, MJVotes};
const VOTE_STR_LEN: usize = 8;

/// Convert a collection of vote values to a count of the # of votes of each value
/// input [0, 1, 1, 3, 3, 3, 3, 4, 4, 5]
/// 
/// (index  0  1  2  3  4  5)
/// output [1, 2, 0, 4, 2, 1]
pub fn as_counts(votes: &Vec<usize>) -> Vec<usize> {
    if votes.len() == 0 {
        return Vec::new();
    }
    let max = votes.iter().max().unwrap();
    let mut res = vec![0; *max+1];
    for value in votes {
        res[*value] += 1;
    }
    res
}

pub trait PollDisplay {
    fn option_display(&self, i: usize) -> String;
}

impl PollDisplay for Poll {
    fn option_display(&self, i: usize) -> String {
        let votes = &self.votes[i];
        let opt_desc = &self.options[i];
        let vote_msg: String = if votes.len() > 0 {
            let counts = as_counts(votes); 
            lrm(VOTE_STR_LEN, &counts)
                .into_iter().enumerate()
                .map(|(i, n)| CONFIG.vote_display[i].repeat(n))
                .collect()
        } else {
            "⬛".repeat(VOTE_STR_LEN)
        };
        let rank = &self.ranking[i];
        let rank_str = if votes.len() > 0 {
            match rank {
            1 => " 🥇".to_string(),
            2 => " 🥈".to_string(),
            3 => " 🥉".to_string(),
            _ => String::new()
            }
        } else {
            String::new()
        };
        format!(
            "{}\n{}  |  {} votes{}{}", 
            opt_desc, vote_msg, 
            votes.len(),
            if let Some(med) = votes.nth_median(0) { 
                format!("  |  median: {}", CONFIG.vote_values[med]) 
            } else { 
                String::new() 
            }, rank_str
        )
    }
}