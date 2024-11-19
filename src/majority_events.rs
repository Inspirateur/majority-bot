use crate::majority_bot::Majority;
use anyhow::anyhow;
use log::{info, warn};
use serenity::{
    all::{CreateInteractionResponse, CreateInteractionResponseMessage}, async_trait, model::prelude::{Guild, Interaction, Message, Ready}, prelude::{Context, EventHandler}
};

#[async_trait]
impl EventHandler for Majority {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!(target: "majority-bot", "{} is connected!", ready.user.name);
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: Option<bool>) {
        self.register_commands(ctx.http.clone(), guild.id).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let can_see_channel = interaction.app_permissions().is_some_and(|p| p.view_channel());
        match interaction {
            Interaction::Command(command) => {
                let command_name = command.data.name.to_string();
                // only answer if the bot has access to the channel
                if can_see_channel {
                    if let Err(why) = match command_name.as_str() {
                        "poll" => self.poll_command(ctx, command).await,
                        "close" => self.close_command(ctx, command).await,
                        "info" => self.info_command(ctx, command).await,
                        _ => Err(anyhow!("Unknown command")),
                    } {
                        warn!(target: "majority-bot", "\\{}: {:?}", command_name, why);
                    }
                } else {
                    if let Err(why) = command
                        .create_response(
                            &ctx.http,
                            CreateInteractionResponse::UpdateMessage(
                                CreateInteractionResponseMessage::new().content("Sorry, I only answer to commands in the channels that I can write to.")
                            )
                        )
                        .await
                    {
                        warn!(target: "majority-bot", "\\{} in non writable channel: {:?}", command_name, why);
                    }
                }
            },
            Interaction::Component(command) => {
                if let Err(why) = self.vote_action(ctx, command).await {
                    warn!(target: "majority-bot", "{}: {:?}", "vote", why);
                }
            },
            _ => {}
        }
    }

    async fn message(&self, ctx: Context, message: Message) {
        if let Some(referenced) = message.referenced_message {
            if let Ok(me) = ctx.http.get_current_user().await {
                if referenced.author.id == me.id {
                    if let Err(why) = self
                        .add_options_command(ctx, *referenced, message.content)
                        .await
                    {
                        warn!(target: "majority-bot", "{}: {:?}", "add_options", why);
                    }
                }
            }
        }
    }
}
