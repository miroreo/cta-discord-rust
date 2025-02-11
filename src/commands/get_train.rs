use crate::cta;
use std::env;
use serenity::builder::CreateCommand;
use serenity::model::application::{ResolvedOption, ResolvedValue};
use serenity::all::{CreateCommandOption, CreateInteractionResponseMessage};

pub async fn run<'a>(options: &'a[ResolvedOption<'a>]) -> CreateInteractionResponseMessage {
  if let Some(ResolvedOption {
    value: ResolvedValue::Integer(run), ..
  }) = options.first()
  {
    let tt = cta::traintracker::TrainTracker::new(env::var("CTA_RAIL_API_KEY").expect("CTA_RAIL_API_KEY not found.").as_str());
    let next_stations = tt.train_next_stations(run.clone() as i32).await;
    match next_stations {
      Ok(val) => {
        let mut embed = serenity::all::CreateEmbed::new();
        for sta in &val {
          embed = embed.field(sta.staNm.clone(), format!("<t:{}:R>", chrono::DateTime::<chrono::Local>::from_naive_utc_and_offset(sta.arrT.checked_sub_offset(chrono::FixedOffset::west_opt(6*3600).unwrap()).expect("Time is gone I guess"), chrono::FixedOffset::west_opt(6*3600).unwrap()).timestamp()), true);
        }
        return CreateInteractionResponseMessage::new().content(format!("Found {} upcoming stations for Train #{}", val.len(), run)).embed(embed);
      },
      Err(err) => {
        return CreateInteractionResponseMessage::new().content(format!("Could not find the specified train: {err:?}" ));
      }
    }
  }
  CreateInteractionResponseMessage::new().content("Options not provided.".to_string())
}

pub fn register() -> CreateCommand {
  let mut run_option = CreateCommandOption::new(serenity::all::CommandOptionType::Integer, "train_number", "Run number of the train").required(true);
  CreateCommand::new("get_train")
    .add_option(run_option)
    .add_integration_type(serenity::all::InstallationContext::Guild)
    .add_integration_type(serenity::all::InstallationContext::User)
    .description("Gets information about a given train.")
}