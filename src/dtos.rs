use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct PollOption {
    pub poll_id: String,
    pub opt_id: usize
}

impl ToString for PollOption {
    fn to_string(&self) -> String {
        format!("{}-{}", self.poll_id, self.opt_id)
    }
}

impl TryFrom<String> for PollOption {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        let (option_id, poll_id) = value.rsplitn(2, "-").collect_tuple().ok_or(
            anyhow!("'{}' is not a PollOption. Expecting <poll_id>-<option_id>", value)
        )?;
        Ok(PollOption {
            poll_id: poll_id.to_string(),
            opt_id: option_id.parse()?,
        })
    }
}

pub struct PollOptionVote {
    pub poll_id: String,
    pub opt_id: usize,
    pub value: usize
}


impl From<PollOptionVote> for String {
    fn from(poll_opt: PollOptionVote) -> Self {
        format!("{}-{}-{}", poll_opt.poll_id, poll_opt.opt_id, poll_opt.value)
    }
}

impl TryFrom<String> for PollOptionVote {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        let (vote, option_id, poll_id) = value.rsplitn(3, "-").collect_tuple().ok_or(
            anyhow!("'{}' is not a PollOptionVote. Expecting <poll_id>-<option_id>-<vote>", value)
        )?;
        Ok(PollOptionVote {
            poll_id: poll_id.to_string(),
            opt_id: option_id.parse()?,
            value: vote.parse()?
        })
    }
}
