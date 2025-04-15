use crate::CTAShared;
use serenity::builder::CreateCommand;
use serenity::model::application::{ResolvedOption, ResolvedValue};
use serenity::all::{Context, CreateCommandOption, CreateInteractionResponseMessage};

pub async fn run<'a>(ctx: &Context, options: &'a[ResolvedOption<'a>]) -> CreateInteractionResponseMessage {
  let data = ctx.data.read().await;
  let data = data.get::<CTAShared>().expect("no shared data");
  
  let gtfs = &data.gtfs;
  if let Some(ResolvedOption {
    value: ResolvedValue::String(id), ..
  }) = options.first()
  {
    let route_name = gtfs.get_route_name(id);
    return CreateInteractionResponseMessage::new().content(route_name.to_string());
  }
  CreateInteractionResponseMessage::new().content("Invalid Route ID".to_string())
}

pub fn register() -> CreateCommand {
  let id_option = CreateCommandOption::new(serenity::all::CommandOptionType::String, "route_id", "ID of the route").required(true);
  CreateCommand::new("route_name")
    .add_option(id_option)
    .add_integration_type(serenity::all::InstallationContext::Guild)
    .add_integration_type(serenity::all::InstallationContext::User)
    .description("Gets the name of a route by ID.")
}