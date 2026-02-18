use crate::{Context, Error};
use poise::CreateReply;
use poise::serenity_prelude as serenity;
use serenity::builder::CreateEmbed;

/// Display user avatar
#[poise::command(slash_command)]
pub async fn avatar(ctx: Context<'_>, user: Option<serenity::User>) -> Result<(), Error> {
    let target = user.unwrap_or_else(|| ctx.author().clone());
    let display_name = target.global_name.as_deref().unwrap_or(&target.name);

    let avatar_url = target
        .avatar_url()
        .unwrap_or_else(|| target.default_avatar_url());

    let avatar_url = format!("{}", avatar_url);

    let embed = CreateEmbed::default()
        .title(format!("{}'s avatar:", display_name))
        .color(0xFB3B5E)
        .image(avatar_url);

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

    println!("🖼 {}'s avatar displayed successfully!", display_name);

    Ok(())
}
