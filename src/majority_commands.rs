use std::sync::Arc;

use crate::{discord_utils::Bot, majority_bot::Majority};
use anyhow::Result;
use log::{trace, warn};
use serenity::{
    http::Http,
    model::prelude::{
        command::CommandOptionType,
        interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
        GuildId,
    },
    prelude::Context,
};

impl Majority {
    pub async fn poll_command(
        &self,
        ctx: Context,
        command: ApplicationCommandInteraction,
    ) -> Result<()> {
        let desc = if let CommandDataOptionValue::String(value) = command
            .data
            .options
            .get(0)
            .expect("Expected a description of the poll")
            .resolved
            .as_ref()
            .expect("Expected a string")
        {
            value.clone()
        } else {
            String::new()
        };
        todo!()
    }

    pub async fn info_command(
        &self,
        ctx: Context,
        command: ApplicationCommandInteraction,
    ) -> Result<()> {
        ctx.http
            .answer(
                &command,
                "Made with ❤️ by Inspi#8989\n
            	Repository: <https://github.com/Inspirateur/majority-bot>\n\n
                More info on Majority Judgement Polls:\n
                <https://electowiki.org/wiki/Majority_Judgment>",
                vec![],
            )
            .await
    }

    pub async fn register_commands(&self, http: Arc<Http>, guild_id: GuildId) {
        trace!(target: "majority-bot", "Registering slash commands for Guild {}", guild_id);
        if let Err(why) = GuildId::set_application_commands(&guild_id, http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("poll").description(
                        "Create a Majority Judgement Poll, add options by replying to the Poll.",
                    ).create_option(|option| {
                        option
                            .name("description")
                            .description("What the poll is about.")
                            .kind(CommandOptionType::String)
                            .required(true)
                    })
                })
                .create_application_command(|command| {
                    command
                        .name("info")
                        .description("Information about this bot.")
                })
        })
        .await
        {
            warn!(target: "majority-bot", "Couldn't register slash commmands: {}", why);
        };
    }
}
