use crate::{phrases, Context, Error};
use poise::serenity_prelude::Permissions;
use std::collections::HashSet;

/// Delete phrase(s) by their numbers (use /list_phrases to see numbers), use spaces or commas for bulk deletion
#[poise::command(slash_command)]
pub async fn delete_phrase(ctx: Context<'_>, numbers: String) -> Result<(), Error> {
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

    let parsed_numbers: Vec<usize> = numbers
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    if parsed_numbers.is_empty() {
        ctx.say("⚠️ Please provide valid phrase numbers (e.g., `1` or `1 5 10`)").await?;
        return Ok(());
    }

    if parsed_numbers.iter().any(|&n| n == 0) {
        ctx.say("⚠️ Phrase numbers start at 1!").await?;
        return Ok(());
    }

    let mut unique_numbers: Vec<usize> = parsed_numbers.into_iter().collect::<HashSet<_>>().into_iter().collect();
    unique_numbers.sort_by(|a, b| b.cmp(a));

    let (deleted_phrases, save_result) = {
        let mut phrases_lock = ctx.data().phrases.write().await;
        let total_phrases = phrases_lock.len();
        let mut deleted = Vec::new();
        let mut invalid = Vec::new();

        for &number in &unique_numbers {
            let index = number - 1;
            if index >= phrases_lock.len() {
                invalid.push(number);
            } else {
                let removed = phrases_lock.remove(index);
                deleted.push((number, removed));
            }
        }

        if !invalid.is_empty() {
            drop(phrases_lock);
            let invalid_str = invalid.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ");
            ctx.say(format!("⚠️ Invalid phrase number(s): {}. Total phrases: {}", invalid_str, total_phrases)).await?;
            return Ok(());
        }

        let save_result = phrases::save_phrases(&phrases_lock);
        (deleted, save_result)
    };

    if let Err(e) = save_result {
        ctx.say(format!("⚠️ Failed to save changes: {}", e)).await?;
        println!("❌ Failed to save after deletion: {}", e);
        return Ok(());
    }

    let response = if deleted_phrases.len() == 1 {
        let (num, phrase) = &deleted_phrases[0];
        format!("✅ Deleted phrase #{}: '{}'", num, phrase)
    } else {
        let mut msg = format!("✅ Deleted {} phrases:\n", deleted_phrases.len());
        for (num, phrase) in &deleted_phrases {
            let line = format!("#{}: '{}'\n", num, phrase);
            if msg.len() + line.len() > 1900 {
                msg.push_str("... (truncated)");
                break;
            }
            msg.push_str(&line);
        }
        msg
    };

    ctx.say(response).await?;
    
    let deleted_nums: Vec<String> = deleted_phrases.iter().map(|(n, _)| n.to_string()).collect();
    println!("🗑️ {} deleted phrase(s) #{}", ctx.author().name, deleted_nums.join(", "));

    Ok(())
}
