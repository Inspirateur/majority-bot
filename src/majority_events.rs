use crate::{
    discord_utils::{is_writable, Bot},
    majority_bot::Majority,
};
use anyhow::anyhow;
use log::{info, warn};
use serenity::{
    async_trait,
    model::prelude::{interaction::Interaction, Ready},
    prelude::{Context, EventHandler},
};

#[async_trait]
impl EventHandler for Majority {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::ApplicationCommand(command) => {
                let command_name = command.data.name.to_string();
                // only answer if the bot has access to the channel
                if is_writable(&ctx, command.channel_id).await {
                    if let Err(why) = match command_name.as_str() {
                        "poll" => self.poll_command(ctx, command).await,
                        "info" => self.info_command(ctx, command).await,
                        _ => Err(anyhow!("Unknown command")),
                    } {
                        warn!(target: "majority-bot", "\\{}: {:?}", command_name, why);
                    }
                } else {
                    if let Err(why) = ctx
                        .http
                        .answer(
                            &command,
                            "Sorry, I only answer to commands in the channels that I can write to.",
                            vec![],
                        )
                        .await
                    {
                        warn!(target: "majority-bot", "\\{} in non writable channel: {:?}", command_name, why);
                    }
                }
            }
            _ => {}
        }
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!(target: "majority-bot", "{} is connected!", ready.user.name);
    }
}
