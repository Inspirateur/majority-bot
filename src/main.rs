mod utils;
mod config;
mod pollopt_to_sql;
mod poll_display;
mod majority_bot;
mod majority_commands;
mod majority_events;
use env_logger;
use log::{error, warn, LevelFilter};
use majority_bot::Majority;
use serenity::{http::Http, model::gateway::GatewayIntents, prelude::*};
use std::env;
use std::fs::read_to_string;

fn get_token(name: &str) -> Option<String> {
    if let Ok(token) = env::var(name) {
        Some(token)
    } else {
        warn!(target: "majority-bot", "Couldn't find the 'MAJORITY_TOKEN' environment variable, using token.txt as fallback");
        if let Ok(content) = read_to_string("token.txt") {
            Some(content)
        } else {
            warn!(target: "majority-bot", "Couldn't access token.txt");
            None
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_module("majority-bot", LevelFilter::Trace)
        .filter_module("majority", LevelFilter::Warn)
        .init();
    // Configure the client with your Discord bot token in the environment.
    let token = get_token("MAJORITY_TOKEN").unwrap();
    let http = Http::new(&token);

    // The Application Id is usually the Bot User Id.
    let bot_id = match http.get_current_application_info().await {
        Ok(info) => info.id,
        Err(why) => panic!("Could not access application info: {:?}", why),
    };
    // Build our client.
    let mut client = Client::builder(
        token,
        GatewayIntents::non_privileged()
    )
    .event_handler(Majority::new())
    .application_id(bot_id.into())
    .await
    .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        error!(target: "majority-bot", "Client error: {:?}", why);
    }
}
