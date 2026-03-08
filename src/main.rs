
mod commands;
mod config;
mod phrases;

use commands::eight_ball;
use dotenvy::dotenv;
use std::env;

use rand::random;
use chrono::Local;

use poise::serenity_prelude as serenity;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    time::Duration,
};
use tokio::sync::RwLock;

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {
    phrases: Arc<RwLock<Vec<String>>>,
    auto_reply_enabled: AtomicBool,
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
    println!("🦀 LowSpecBot: Rust Edition");
    println!("🔧 Setting up everything...");
    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
        commands: vec![
            commands::ping::ping(), 
            commands::eight_ball::eight_ball(), 
            commands::help::help(),
            commands::clean::clean(),
            commands::avatar::avatar(),
            commands::amplify::amplify(),
            commands::toggle_replies::toggle_replies(),
            commands::add_phrase::add_phrase(),
            commands::delete_phrase::delete_phrase(),
            commands::list_phrases::list_phrases()
            ],//commands::help(), commands::vote(), commands::getvotes()],
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
        pre_command: |_ctx| {
            Box::pin(async move {
                //println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        // This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                let mut fmsg = format!("💻 Command {} issued by {}!", ctx.command().qualified_name, ctx.author().name);
                fmsg = dateify(&fmsg);
                println!("{}", fmsg);
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
                            let mut fmsg = format!("Message received: '{}' sent by {}", content, new_message.author.name);
                            fmsg = dateify(&fmsg);
                            println!("{}", fmsg);
                        }

                        if new_message.author.bot {
                            return Ok(());
                        }

                        // @channel ;)
                        let neraiyo: String = content.to_lowercase();
                        if neraiyo.contains("nullpo") {
                            println!("🔊 Triggered neraiyo response");
                            new_message.channel_id.say(&_ctx.http, "Gah!").await?;
                        }

                        // Nakanaide 583753611067129856
                        if new_message.author.id == 583753611067129856 && new_message.attachments.len() > 0 {
                            println!("🔊 Triggered nakanaide response");
                            new_message.channel_id.say(&_ctx.http, "https://cdn.discordapp.com/attachments/557422582584836109/980875313175232512/unknown.png").await?;
                        }

                        // 8ball
                        let response_seed: u32 = (random::<u32>() % 30) + 1;
                        if _data.auto_reply_enabled.load(Ordering::Relaxed) && response_seed == 12 {
                            let content = {
                                let phrases = _data.phrases.read().await;
                                eight_ball::get_random_phrase(&phrases).map(|s| s.to_string())
                            };
                            if let Some(content) = content {
                                new_message.channel_id.say(&_ctx.http, &content).await?;
                                println!("📢 Random response triggered!");
                            }
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
                let phrases = phrases::load_phrases()?;
                let auto_reply_enabled = config::load_config()
                    .map(|c| c.auto_reply_enabled)
                    .unwrap_or(true);
                println!("⚙️ Auto-reply setting loaded: {}", if auto_reply_enabled { "enabled" } else { "disabled" });
                Ok(Data {
                    phrases: Arc::new(RwLock::new(phrases)),
                    auto_reply_enabled: AtomicBool::new(auto_reply_enabled),
                })
            })
        })
        .options(options)
        .build();

    println!("⚙ Configuring environment variables...");
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
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT | serenity::GatewayIntents::GUILD_MESSAGES;

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    let mut client = match client {
        Ok(client) => client,
        Err(error) => {
            println!("❗ Failed to create client: {}", error);
            return;
        }
    };

    if let Err(error) = client.start().await {
        println!("❗ Client exited with error: {}", error);
    }
}

fn dateify(log: &str) -> String {
    let now = Local::now();
    let format_datetime = now.format("%d/%m/%y:%H:%M").to_string();
    format!("{} {}", format_datetime, log)
}