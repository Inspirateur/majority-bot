use anyhow::{Context as ContextErr, Result};
use serenity::{
    async_trait,
    http::Http,
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction,
            InteractionResponseType::ChannelMessageWithSource,
        },
        prelude::{Channel, ChannelId, Message},
    },
    prelude::Context,
};

type Command = ApplicationCommandInteraction;
pub struct Attachment {
    pub file: Vec<u8>,
    pub filename: String,
}

#[async_trait]
pub trait Bot {
    async fn answer(&self, command: &Command, content: &str, files: Vec<Attachment>) -> Result<Message>;

    async fn followup(
        &self,
        command: &Command,
        content: &str,
        files: Vec<Attachment>,
    ) -> Result<()>;
}

#[async_trait]
impl Bot for Http {
    async fn answer(&self, command: &Command, content: &str, files: Vec<Attachment>) -> Result<Message> {
        (command
            .create_interaction_response(self, |response| {
                response
                    .kind(ChannelMessageWithSource)
                    .interaction_response_data(|answer| {
                        answer.content(content);
                        files.iter().for_each(|Attachment { file, filename }| {
                            answer.add_file((file.as_slice(), filename.as_str()));
                        });
                        answer
                    })
            })
            .await)
            .context("Command create response failed")?;
        Ok(command.get_interaction_response(self).await?)
    }

    async fn followup(
        &self,
        command: &Command,
        content: &str,
        files: Vec<Attachment>,
    ) -> Result<()> {
        (command
            .create_followup_message(self, |answer| {
                answer.content(content);
                files.iter().for_each(|Attachment { file, filename }| {
                    answer.add_file((file.as_slice(), filename.as_str()));
                });
                answer
            })
            .await)
            .context("Command create followup failed")?;
        Ok(())
    }
}

pub async fn is_writable(ctx: &Context, channel_id: ChannelId) -> bool {
    if let Ok(Channel::Guild(channel)) = channel_id.to_channel(&ctx.http).await {
        if let Ok(me) = ctx.http.get_current_user().await {
            if let Ok(perms) = channel.permissions_for_user(&ctx.cache, me.id) {
                return perms.send_messages();
            }
        }
    }
    false
}
