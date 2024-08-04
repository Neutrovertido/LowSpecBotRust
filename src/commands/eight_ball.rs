use std::fs::File;
use std::io::{self, BufRead};

use rand::Rng;
use serenity::all::{CreateCommand, ResolvedOption};

pub fn get_phrases() -> io::Result<Vec<String>> {
    let mut phrases = Vec::new();
    
    let path = "../../phrases.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        phrases.push(line.unwrap().to_string());
    }
    Ok(phrases)
}

pub fn run(_options: &[ResolvedOption]) -> String {
    let phrases = get_phrases().unwrap();
    let mut rng = rand::thread_rng();

    let seed: usize = (rng.gen::<f32>() * phrases.len() as f32) as usize;

    phrases[seed].clone()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("8ball").description("Iluminates your soul with some random generated phrases")
}