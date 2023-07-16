use anyhow::{anyhow, Result};
use itertools::Itertools;
use rusqlite::{ToSql, types::ToSqlOutput, types::Value};

pub struct PollOption {
    pub poll_id: u64,
    pub opt_id: usize
}

impl ToSql for PollOption {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Text(
            format!("{}-{}", self.poll_id, self.opt_id)
        )))
    }
}

pub struct PollOptionVote {
    pub poll_id: u64,
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
            poll_id: poll_id.parse()?,
            opt_id: option_id.parse()?,
            value: vote.parse()?
        })
    }
}
