use dotenvy::dotenv;
use std::env;

fn main() {
    println!("🦀 LowSpecBotRust");
    println!("⚙ Configuring environment variables...");
    match dotenv() {
        Ok(_) => {
            println!("✅.env found! Utilizing its environment variables!")
        },
        Err(_) => {
            println!("⚠️ .env not found!");
            println!("⚠️ Attempting to use environment variables...");
        }
    }
    let token = env::var("TOKEN").expect("❗Discord token environment variable not found! Quitting program...\n");
    println!("Token: {}", token);
}
