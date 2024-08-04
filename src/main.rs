mod commands;

use dotenvy::dotenv;
use serenity::all::{Message, Command, CreateInteractionResponse, CreateInteractionResponseMessage, GuildId, Interaction, Ready};
use std::env;

use serenity::async_trait;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // @channel ;)
        let neraiyo: String = msg.content.to_lowercase();
        if neraiyo.contains("nullpo") {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Gah!").await {
                println!("⚠️ Error sending message: {why:?}");
            }
        }

        // Nakanaide 583753611067129856
        if msg.author.id == 327946633499246593 && msg.attachments.len() > 0 {
            if let Err(why) = msg.channel_id.say(&ctx.http, "https://cdn.discordapp.com/attachments/557422582584836109/980875313175232512/unknown.png").await {
                println!("⚠️ Error sending message: {why:?}");
            }
        }
        
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("👀 Received command interaction: {} by user: {}",command.data.name.as_str(), command.user.name);

            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }
}

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
        Client::builder(&token, intents)
            .event_handler(Handler)
            .await
            .expect("❗Error creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
