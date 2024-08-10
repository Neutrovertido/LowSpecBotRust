use crate::{Context, Error};

use std::fs::File;
use std::io::{self, BufRead};

/// Iluminates your soul with some random generated phrases
#[poise::command(slash_command, rename = "8ball")]
pub async fn eight_ball(
  ctx: Context<'_>,
) -> Result<(), Error> {
  ctx.say(get_random_phrase()).await?;
  Ok(())
}

pub fn get_phrases() -> io::Result<Vec<String>> {
    let mut phrases = Vec::new();
    
    let path = "phrases.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        phrases.push(line.unwrap().to_string());
    }
    Ok(phrases)
}

pub fn get_random_phrase() -> String{
    let phrases = get_phrases().unwrap();
    //println!("{}", phrases[0]);
    let seed: usize = rand::random::<usize>() % phrases.len();
    phrases[seed].clone()
}