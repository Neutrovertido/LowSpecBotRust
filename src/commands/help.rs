use crate::{Context, Error};
use poise::CreateReply;
use serenity::builder::CreateEmbed;
use serenity::model::Timestamp;

/// View available commands
#[poise::command(slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    //ctx.say("Pong!").await?;
    let commands = vec![
        ("ping", "A ping command"),
        ("8ball", "Iluminates your soul with some random generated phrases")
    ];

    let mut embed = CreateEmbed::default()
        .title("Current Commands")
        .description("Here are the available commands:")
        .color(0)
        .timestamp(Timestamp::now());

    for (name, description) in commands {
        embed = embed.field(name, description, false);
    }

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
