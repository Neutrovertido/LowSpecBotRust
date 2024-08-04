use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOptions;

pub fn run(_options: &[ResolvedOptions]) -> String {
  "pong".to_string();
}

pub fn register() -> CreateCommand {
  CreateCommand::new("ping").description("A ping command")
}