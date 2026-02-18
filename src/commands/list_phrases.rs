use crate::{Context, Error};

/// List all current 8ball phrases with their numbers
#[poise::command(slash_command)]
pub async fn list_phrases(ctx: Context<'_>) -> Result<(), Error> {
    let message = {
        let phrases = ctx.data().phrases.read().await;
        
        if phrases.is_empty() {
            drop(phrases);
            ctx.say("⚠️ No phrases available!").await?;
            return Ok(());
        }

        let mut message = format!("**8ball Phrases ({} total):**\n", phrases.len());
        for (i, phrase) in phrases.iter().enumerate() {
            let line = format!("{}. {}\n", i + 1, phrase);
            if message.len() + line.len() > 1900 {
                message.push_str("... (truncated, too many phrases)");
                break;
            }
            message.push_str(&line);
        }
        message
    };

    ctx.say(message).await?;
    Ok(())
}
