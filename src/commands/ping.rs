use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use serenity::all::CreateInteractionResponseMessage;

pub fn run(_options: &[ResolvedOption]) -> CreateInteractionResponseMessage {
  CreateInteractionResponseMessage::new().content("Pong!".to_string())
}

pub fn register() -> CreateCommand {
  CreateCommand::new("ping").description("Replies with Pong!")
}