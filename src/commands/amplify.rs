use crate::{Context, Error};

fn emojify(input: char) -> String {
    match input {
        'a'..='z' => format!(":regional_indicator_{}:", input),
        '0' => ":zero:".to_string(),
        '1' => ":one:".to_string(),
        '2' => ":two:".to_string(),
        '3' => ":three:".to_string(),
        '4' => ":four:".to_string(),
        '5' => ":five:".to_string(),
        '6' => ":six:".to_string(),
        '7' => ":seven:".to_string(),
        '8' => ":eight:".to_string(),
        '9' => ":nine:".to_string(),
        _ => input.to_string(),
    }
}

/// Translates normal text messages into regional indicator emojis
#[poise::command(slash_command)]
pub async fn amplify(ctx: Context<'_>, message: String) -> Result<(), Error> {
    let actual_message = message.trim();

    if actual_message.is_empty() {
        ctx.say("⚠️ You must specify a message!").await?;
        println!("❌ Message not specified!");
        return Ok(());
    }

    let amplified = actual_message
        .to_lowercase()
        .chars()
        .map(emojify)
        .collect::<Vec<String>>()
        .join(" ");

    let display_name = match ctx.author_member().await {
        Some(member) => match &member.nick {
            Some(nickname) => nickname.clone(),
            None => ctx.author().name.clone(),
        },
        None => ctx
            .author()
            .global_name
            .clone()
            .unwrap_or_else(|| ctx.author().name.clone()),
    };

    let result = format!("**{}:  **{}", display_name, amplified);

    println!("🅰 Emojified result");
    ctx.say(result).await?;

    Ok(())
}
