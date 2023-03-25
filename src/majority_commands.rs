use crate::{majority_bot::Majority, config::CONFIG, poll_display::PollDisplay};
use serenity_utils::{Bot, Button};
use anyhow::{Ok, Result, Error, anyhow};
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
            InteractionResponseType::DeferredUpdateMessage
        },
        GuildId, Message, component::ButtonStyle,
    },
    prelude::Context,
};
use std::sync::Arc;

struct PollOptionVote {
    poll_id: String,
    opt_id: usize,
    value: usize
}


impl From<PollOptionVote> for String {
    fn from(poll_opt: PollOptionVote) -> Self {
        format!("{}-{}-{}", poll_opt.poll_id, poll_opt.opt_id, poll_opt.value)
    }
}

impl TryFrom<String> for PollOptionVote {
    type Error = Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
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
                &format!(
                    "{}\n\n<Reply to this message with 1 poll option per line>",
                    desc
                ),
                vec![],
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
                &poll.option_display(opt_id), 
                CONFIG.vote_values.iter().enumerate().map(|(value, label)| Button {
                    custom_id: String::from(PollOptionVote {poll_id: poll_msg.id.to_string(), opt_id, value}),
                    style: ButtonStyle::Secondary,
                    label: label.to_string()
                }).collect_vec()
            ).await?;
            self.msg_map.insert(opt_id, msg.id)?;
        }
        Ok(())
    }

    pub async fn vote_command(&self, ctx: Context, command: MessageComponentInteraction) -> Result<()> {
        let PollOptionVote { poll_id, opt_id, value } = PollOptionVote::try_from(command.data.custom_id.clone())?;
        command.create_interaction_response(&ctx.http, |response| response.kind(DeferredUpdateMessage)).await?;
        let _poll = self.polls.get_poll(poll_id.clone())?;
        let last_ranking = _poll.ranking;
        let poll = self.polls.vote(poll_id, opt_id, command.user.id, value)?;
        // update the option message that recieved the vote
        let msg_id = self.msg_map.get_single(opt_id)?;
        let mut msg = ctx.http.get_message(command.channel_id.0, msg_id.parse()?).await?;
        msg.edit(&ctx.http, |msg| msg.content(poll.option_display(opt_id))).await?;
        // we also need to update the messages of options that changed ranks after this vote 
        let to_update = last_ranking.into_iter().zip(&poll.ranking).enumerate().filter_map(|(i, (old_rank, new_rank))| {
            if old_rank != *new_rank { Some(i) } else { None }
        });
        for opt_id in to_update {
            let msg_id = self.msg_map.get_single(opt_id)?;
            let mut msg = ctx.http.get_message(command.channel_id.0, msg_id.parse()?).await?;
            msg.edit(&ctx.http, |msg| msg.content(poll.option_display(opt_id))).await?;    
        }
        Ok(())
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
