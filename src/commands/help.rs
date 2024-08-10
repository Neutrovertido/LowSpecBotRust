use crate::{Context, Error};

/// View available commands
#[poise::command(slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
  //ctx.say("Pong!").await?;
  
  Ok(())
}