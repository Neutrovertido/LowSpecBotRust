use dotenvy::dotenv;
use std::env;

use serenity::async_trait;
// use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    println!("🦀 LowSpecBot: Rust Edition");
    println!("⚙ Configuring environment variables...");

    // This block attempts to use .env first and regular environment variables if it fails
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
    
    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("❗Error creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
