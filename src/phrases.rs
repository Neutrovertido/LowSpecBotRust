use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

const PHRASES_PATH: &str = "phrases.toml";

#[derive(Serialize, Deserialize)]
pub struct PhrasesFile {
    pub phrases: Vec<String>,
}

pub fn load_phrases() -> io::Result<Vec<String>> {
    let contents = match fs::read_to_string(PHRASES_PATH) {
        Ok(contents) => contents,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(Vec::new());
        }
        Err(error) => return Err(error),
    };
    let phrases_data: PhrasesFile = toml::from_str(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(phrases_data.phrases)
}

pub fn save_phrases(phrases: &[String]) -> io::Result<()> {
    let phrases_data = PhrasesFile {
        phrases: phrases.to_vec(),
    };
    let toml_string = toml::to_string_pretty(&phrases_data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    fs::write(PHRASES_PATH, toml_string)
}
