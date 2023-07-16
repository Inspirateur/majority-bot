use crate::{majority_bot::Majority, config::CONFIG, poll_display::PollDisplay, pollopt_to_sql::{PollOptionVote, PollOption}};
use serenity_utils::{Bot, Button, MessageBuilder};
use anyhow::{Ok, Result};
use itertools::Itertools;
use log::{trace, warn};
use majority::DefaultVote;
use serenity::{
    http::Http,
    model::prelude::{
        command::CommandOptionType,
        interaction::{
            application_command::{ApplicationCommandInteraction, CommandDataOptionValue}, 
            message_component::MessageComponentInteraction,
            InteractionResponseType::UpdateMessage
        },
        GuildId, Message, component::ButtonStyle,
    },
    prelude::Context,
};
use std::sync::Arc;


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
        let answer = ctx
            .http
            .answer(
                &command,
                MessageBuilder::new(format!(
                    "{}\n*Reply to this message with 1 poll option per line*",
                    desc
                ))
            )
            .await?;
        self.polls.add_poll(
            answer.id,
            command.member.unwrap().user.id,
            desc,
            Vec::<String>::new(),
            DefaultVote::IGNORE
        )?;
        Ok(())
    }

    pub async fn add_options_command(
        &self,
        ctx: Context,
        poll_msg: Message,
        options_msg: String,
    ) -> Result<()> {
        let options = options_msg
            .lines()
            .map(|opt| opt.trim())
            .filter(|opt| opt.len() > 0)
            .collect_vec();
        let n = options.len();
        let poll = self.polls.add_options(poll_msg.id, options)?;
        let from = poll.options.len()-n;
        for opt_id in from..poll.options.len() {
            let msg = ctx.http.send(
                poll_msg.channel_id,
                MessageBuilder::new(poll.option_display(opt_id)).buttons(
                    CONFIG.vote_values.iter().enumerate().map(|(value, label)| Button {
                        custom_id: String::from(PollOptionVote {poll_id: poll_msg.id.0, opt_id, value}),
                        style: ButtonStyle::Secondary,
                        label: label.to_string()
                    }).collect_vec()
                )
            ).await?;
            self.msg_map.insert(PollOption {poll_id: poll_msg.id.0, opt_id}, msg.id.0)?;
        }
        Ok(())
    }

    pub async fn vote_command(&self, ctx: Context, command: MessageComponentInteraction) -> Result<()> {
        let PollOptionVote { poll_id, opt_id, value } = PollOptionVote::try_from(command.data.custom_id.clone())?;
        let _poll = self.polls.get_poll(poll_id.clone())?;
        let last_ranking = _poll.ranking;
        let poll = self.polls.vote(poll_id.clone(), opt_id, command.user.id, value)?;
        // update the option message that recieved the vote
        command.create_interaction_response(
            &ctx.http, 
            |response| response.kind(UpdateMessage)
            .interaction_response_data(|data| data.content(poll.option_display(opt_id)))
        ).await?;
        // we also need to update the messages of other options that changed ranks after this vote 
        // TODO: this doesn't scale well, edit are heavily rate limited, and older edits call can be played after newer ones, erasing votes in the display !
        // the only real solution is a buffer that recieve edits calls on messages, discard the previous ones and apply 1 edit every X seconds with the latest one only
        let to_update = last_ranking.into_iter().zip(&poll.ranking).enumerate()
        .filter_map(|(i, (old_rank, new_rank))| {
            if poll.votes[i].len() > 0 && (old_rank < 3 || *new_rank < 3) && old_rank != *new_rank && i != opt_id { Some(i) } else { None }
        });
        for opt_id in to_update {
            let msg_id: u64 = self.msg_map.get(PollOption { poll_id: poll_id, opt_id })?;
            let mut msg = ctx.http.get_message(command.channel_id.0, msg_id).await?;
            msg.edit(&ctx.http, |msg| msg.content(poll.option_display(opt_id))).await?;    
        }
        Ok(())
    }

    pub async fn close_command(
        &self, ctx: Context, command: ApplicationCommandInteraction
    ) -> Result<()> {
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
                MessageBuilder::new(
                    "Made with ❤️ by Inspi#8989\n
                    Repository: <https://github.com/Inspirateur/majority-bot>\n\n
                    More info on Majority Judgement Polls:\n
                    <https://electowiki.org/wiki/Majority_Judgment>"
                )
            )
            .await?;
        Ok(())
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
                /* TODO: this command, but idk if i can get a message replied to with a command
                .create_application_command(|command| {
                    command.name("close")
                        .description("Closes the Poll that is replied to.")
                }) */
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
