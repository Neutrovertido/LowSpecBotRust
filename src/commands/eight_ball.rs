use std::fs::File;
use std::io::{self, BufRead};

use serenity::all::{CreateCommand, ResolvedOption};

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

pub fn run(_options: &[ResolvedOption]) -> String {
    get_random_phrase()
}

pub fn get_random_phrase() -> String{
    let phrases = get_phrases().unwrap();
    let seed: usize = rand::random::<usize>() * phrases.len();
    phrases[seed].clone()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("8ball").description("Iluminates your soul with some random generated phrases")
}