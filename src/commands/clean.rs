use crate::{Context, Error};
use poise::serenity_prelude::{Permissions, MessageId};

/// Erases the amount messages specified with a maximum of 100
#[poise::command(slash_command)]
pub async fn clean(ctx: Context<'_>, count: u8) -> Result<(), Error> {
    //ctx.say("Pong!").await?;

    let cow = ctx.author_member().await;

    let author = match cow {
        Some(cow) => cow.into_owned(), // Convert Cow<Member> into Member
        None => {
            ctx.say("⚠️ This command can only be used in a guild.").await?;
            return Ok(());
        }
    };

    let permissions = author.permissions.unwrap();

    if !permissions.contains(Permissions::MANAGE_MESSAGES) && !permissions.contains(Permissions::ADMINISTRATOR) {
        ctx.say("⚠️ You don't have sufficient permission to do that!").await?;
        return Ok(());
    }

    if count == 0 || count > 100 {
        ctx.say("⚠️ Please specify an amount between 1 and 100").await?;
        return Ok(());
    }

    let cid = ctx.channel_id();

    let messages = cid
        .messages(ctx, poise::serenity_prelude::GetMessages::default().limit(count))
        .await;

    let mut message_ids: Vec<MessageId> = Vec::new();

    for message in messages {
        for msg in message {
            message_ids.push(msg.id);
        }
    }

    cid.delete_messages(ctx, message_ids).await?;

    let deleted = format!("♻️ Deleted {} messages!", count);

    ctx.say(format!("{}", deleted)).await?;

    println!("{}", deleted);

    Ok(())
}