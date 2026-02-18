use crate::{Context, Error};
use poise::CreateReply;
use serenity::builder::CreateEmbed;
use serenity::model::Timestamp;

/// List all current 8ball phrases with their numbers
#[poise::command(slash_command)]
pub async fn list_phrases(ctx: Context<'_>) -> Result<(), Error> {
    let (description, total_phrases) = {
        let phrases = ctx.data().phrases.read().await;
        
        if phrases.is_empty() {
            drop(phrases);
            ctx.say("⚠️ No phrases available!").await?;
            return Ok(());
        }

        let mut description = String::new();
        for (i, phrase) in phrases.iter().enumerate() {
            let line = format!("{}. {}\n", i + 1, phrase);
            if description.len() + line.len() > 4000 {
                description.push_str("... (truncated, too many phrases)");
                break;
            }
            description.push_str(&line);
        }
        (description, phrases.len())
    };

    let embed = CreateEmbed::default()
        .title(format!("8ball Phrases ({} total)", total_phrases))
        .description(description)
        .color(0xFB3B5E)
        .timestamp(Timestamp::now());

    let reply = CreateReply {
        content: None,
        embeds: vec![embed],
        attachments: vec![],
        ephemeral: None,
        components: None,
        allowed_mentions: None,
        reply: true,
        __non_exhaustive: (),
    };

    ctx.send(reply).await?;
    Ok(())
}
