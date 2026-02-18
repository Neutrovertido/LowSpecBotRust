use crate::{phrases, Context, Error};

/// Add a new phrase to the 8ball
#[poise::command(slash_command)]
pub async fn add_phrase(ctx: Context<'_>, phrase: String) -> Result<(), Error> {
    if phrase.trim().is_empty() {
        ctx.say("⚠️ Phrase cannot be empty!").await?;
        return Ok(());
    }

    let save_result = {
        let mut phrases_lock = ctx.data().phrases.write().await;
        phrases_lock.push(phrase.clone());
        phrases::save_phrases(&phrases_lock)
    };
    
    if let Err(e) = save_result {
        ctx.say(format!("⚠️ Failed to save phrase: {}", e)).await?;
        println!("❌ Failed to save phrase: {}", e);
        return Ok(());
    }
    
    ctx.say(format!("✅ Added phrase: '{}'", phrase)).await?;
    println!("➕ {} added phrase: '{}'", ctx.author().name, phrase);
    
    Ok(())
}
