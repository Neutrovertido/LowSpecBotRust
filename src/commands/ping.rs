use crate::{Context, Error};

/// A ping command
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
  ctx.say("Pong!").await?;
  Ok(())
}