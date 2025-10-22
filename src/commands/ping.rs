use serenity::all::CreateInteractionResponseMessage;
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> CreateInteractionResponseMessage {
  CreateInteractionResponseMessage::new().content("Pong!".to_string())
}

pub fn register() -> CreateCommand {
  CreateCommand::new("ping").description("Replies with Pong!")
}
