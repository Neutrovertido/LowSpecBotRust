
mod commands;

use dotenvy::dotenv;
use std::env;

use poise::serenity_prelude as serenity;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {
    //votes: Mutex<HashMap<String, u32>>,
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("❗ Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("❗ Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    println!("🔧 Setting up everything...");
    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
        commands: vec![],//commands::help(), commands::vote(), commands::getvotes()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("~".into()),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            additional_prefixes: vec![],
            ..Default::default()
        },
        // The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        // This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                //println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        // This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                println!("Command {} issued by {}!", ctx.command().qualified_name, ctx.author());
            })
        },
        // Every command invocation must pass this check to continue execution
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
                Ok(true)
            })
        }),
        // Enforce command checks even for owners (enforced by default)
        // Set to true to bypass checks, which is useful for testing
        skip_checks_for_owners: false,
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                /* println!(
                    "Got an event in event handler: {:?}",
                    event.snake_case_name()
                ); */

                // Message loop
                match event {
                    serenity::FullEvent::Message { new_message } => {
                        let content = &new_message.content;
                        if new_message.author.id != _ctx.cache.current_user().id {
                            println!("Message received: '{}' sent by {}", content, new_message.author.name);
                        }

                        // @channel ;)
                        let neraiyo: String = content.to_lowercase();
                        if neraiyo.contains("nullpo") {
                            new_message.channel_id.say(&_ctx.http, "Gah!").await?;
                        }

                        // Nakanaide 583753611067129856
                        if new_message.author.id == 327946633499246593 && new_message.attachments.len() > 0 {
                            new_message.channel_id.say(&_ctx.http, "https://cdn.discordapp.com/attachments/557422582584836109/980875313175232512/unknown.png").await?;
                        }
                    },
                    _ => {}
                }

                Ok(())
            })
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("✅ Bot initialized successfully!\n🔑 Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    //votes: Mutex::new(HashMap::new()),
                })
            })
        })
        .options(options)
        .build();

    // This block attempts to use .env first and regular environment variables if it fails
    match dotenv() {
        Ok(_) => {
            println!("✅ .env found! Utilizing its environment variables!")
        },
        Err(_) => {
            println!("⚠️ .env not found!");
            println!("⚠️ Attempting to use environment variables...");
        }
    }
    let token = env::var("TOKEN").expect("❗Discord token environment variable not found! Quitting program...\n");
    
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}