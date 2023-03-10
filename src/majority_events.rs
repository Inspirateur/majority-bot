use crate::majority_bot::Majority;
use serenity::{async_trait, prelude::EventHandler};

#[async_trait]
impl EventHandler for Majority {}
