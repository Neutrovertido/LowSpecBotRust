use crate::{Context, Error};
use poise::CreateReply;
use serenity::builder::CreateEmbed;
use serenity::model::Timestamp;

/// List all current 8ball phrases with their numbers
#[poise::command(slash_command)]
pub async fn list_phrases(ctx: Context<'_>) -> Result<(), Error> {
    let (phrase_chunks, total_phrases) = {
        let phrases = ctx.data().phrases.read().await;
        
        if phrases.is_empty() {
            drop(phrases);
            ctx.say("⚠️ No phrases available!").await?;
            return Ok(());
        }

        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        
        for (i, phrase) in phrases.iter().enumerate() {
            let line = format!("{}. {}\n", i + 1, phrase);
            
            if current_chunk.len() + line.len() > 4000 {
                chunks.push(current_chunk.clone());
                current_chunk.clear();
            }
            current_chunk.push_str(&line);
        }
        
        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }
        
        (chunks, phrases.len())
    };

    let total_pages = phrase_chunks.len();

    for (page_num, chunk) in phrase_chunks.into_iter().enumerate() {
        let title = if total_pages > 1 {
            format!("8ball Phrases ({} total) - Page {}/{}", total_phrases, page_num + 1, total_pages)
        } else {
            format!("8ball Phrases ({} total)", total_phrases)
        };

        let embed = CreateEmbed::default()
            .title(title)
            .description(chunk)
            .color(0xFB3B5E)
            .timestamp(Timestamp::now());

        if page_num == 0 {
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
        } else {
            ctx.channel_id().send_message(&ctx, poise::serenity_prelude::CreateMessage::new().embed(embed)).await?;
        }
    }

    Ok(())
}
