use crate::{Context, Error};
use poise::serenity_prelude::Permissions;
use std::sync::atomic::Ordering;

/// Toggles random automatic bot replies
#[poise::command(slash_command)]
pub async fn toggle_replies(ctx: Context<'_>) -> Result<(), Error> {
    let author = match ctx.author_member().await {
        Some(member) => member.into_owned(),
        None => {
            ctx.say("⚠️ This command can only be used in a guild.").await?;
            return Ok(());
        }
    };

    let permissions = match author.permissions {
        Some(permissions) => permissions,
        None => {
            ctx.say("⚠️ Unable to verify permissions.").await?;
            return Ok(());
        }
    };

    if !permissions.contains(Permissions::ADMINISTRATOR) {
        ctx.say("⚠️ You need administrator permission to do that!").await?;
        return Ok(());
    }

    let was_enabled = ctx.data().auto_reply_enabled.fetch_xor(true, Ordering::Relaxed);
    let enabled = !was_enabled;

    if enabled {
        ctx.say("✅ Random replies are now enabled.").await?;
        println!("🔁 Random replies enabled by {}", ctx.author().name);
    } else {
        ctx.say("⛔ Random replies are now disabled.").await?;
        println!("🔁 Random replies disabled by {}", ctx.author().name);
    }

    Ok(())
}
