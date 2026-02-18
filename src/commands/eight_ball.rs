use crate::{Context, Error};

/// Iluminates your soul with some random generated phrases
#[poise::command(slash_command, rename = "8ball")]
pub async fn eight_ball(
  ctx: Context<'_>,
) -> Result<(), Error> {
  let content = {
    let phrases = ctx.data().phrases.read().await;
    get_random_phrase(&phrases).unwrap_or("No 8ball phrases available right now.").to_string()
  };
  ctx.say(content).await?;
  Ok(())
}

pub fn get_random_phrase(phrases: &[String]) -> Option<&str> {
    if phrases.is_empty() {
        return None;
    }

    let seed: usize = rand::random::<usize>() % phrases.len();
    Some(phrases[seed].as_str())
}