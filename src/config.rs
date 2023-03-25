use confy;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub vote_values: Vec<String>,
    pub vote_display: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vote_values: ["😣", "😕", "🙂", "🤩"].map(String::from).to_vec(),
            vote_display: ['🟥', '🟧', '🟨', '🟩'].map(String::from).to_vec()
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = confy::load_path("./config.toml").unwrap();
}
