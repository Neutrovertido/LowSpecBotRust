use crate::{phrases, Context, Error};
use poise::serenity_prelude::Permissions;

/// Delete a phrase by its number (use /list_phrases to see numbers)
#[poise::command(slash_command)]
pub async fn delete_phrase(ctx: Context<'_>, number: usize) -> Result<(), Error> {
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

    if number == 0 {
        ctx.say("⚠️ Phrase numbers start at 1!").await?;
        return Ok(());
    }

    let index = number - 1;
    
    let (removed_phrase, save_result) = {
        let mut phrases_lock = ctx.data().phrases.write().await;
        
        if index >= phrases_lock.len() {
            let len = phrases_lock.len();
            drop(phrases_lock);
            ctx.say(format!("⚠️ Phrase #{} doesn't exist! Total phrases: {}", number, len)).await?;
            return Ok(());
        }

        let removed_phrase = phrases_lock.remove(index);
        let save_result = phrases::save_phrases(&phrases_lock);
        (removed_phrase, save_result)
    };
    
    if let Err(e) = save_result {
        ctx.say(format!("⚠️ Failed to save changes: {}", e)).await?;
        println!("❌ Failed to save after deletion: {}", e);
        return Ok(());
    }
    
    ctx.say(format!("✅ Deleted phrase #{}: '{}'", number, removed_phrase)).await?;
    println!("🗑️ {} deleted phrase #{}: '{}'", ctx.author().name, number, removed_phrase);
    
    Ok(())
}
