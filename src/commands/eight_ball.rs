use crate::{Context, Error};

use std::fs::File;
use std::io::{self, BufRead};

/// Iluminates your soul with some random generated phrases
#[poise::command(slash_command, rename = "8ball")]
pub async fn eight_ball(
  ctx: Context<'_>,
) -> Result<(), Error> {
  let content = get_random_phrase(&ctx.data().phrases).unwrap_or("No 8ball phrases available right now.");
  ctx.say(content).await?;
  Ok(())
}

pub fn get_phrases() -> io::Result<Vec<String>> {
    let mut phrases = Vec::new();
    
    let path = "phrases.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
      phrases.push(line?);
    }
    Ok(phrases)
}

  pub fn get_random_phrase(phrases: &[String]) -> Option<&str> {
    if phrases.is_empty() {
      return None;
    }

    let seed: usize = rand::random::<usize>() % phrases.len();
    Some(phrases[seed].as_str())
}