use crate::{Context, Error};
use serenity::model::channel::Message;

/// Erases the amount messages specified with a maximum of 100
#[poise::command(slash_command)]
pub async fn clean(ctx: Context<'_>, count: u64) -> Result<(), Error> {
    //ctx.say("Pong!").await?;

    if count == 0 || count > 100 {
        ctx.say("Please specify an amout between 1 and 100").await?;
        return Ok(());
    }

    let cid = ctx.channel_id();
    let mut msgs: Vec<Message> = Vec::new();

    Ok(())
}