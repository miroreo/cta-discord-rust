use crate::CTAShared;
use std::fmt::Write;
use chrono::TimeZone;
use chrono_tz::America::Chicago;
use serenity::builder::CreateCommand;
use serenity::model::application::{ResolvedOption, ResolvedValue};
use serenity::all::{Context, CreateCommandOption, CreateInteractionResponseMessage};

#[allow(clippy::cast_possible_truncation)]
pub async fn run<'a>(ctx: &Context, options: &'a[ResolvedOption<'a>]) -> CreateInteractionResponseMessage {
  let data = ctx.data.read().await;
  let data = data.get::<CTAShared>().expect("no shared data");
  let tt = &data.traintracker;
  if let Some(ResolvedOption {  
    value: ResolvedValue::Integer(run), ..
  }) = options.first()
  {
    let next_stations = tt.follow_train(*run as i32).await;
    match next_stations {
      Ok(val) => {
        let embed = serenity::all::CreateEmbed::new();
        let desc: String = val.iter().fold( String::new(), |mut acc, sta| {
          writeln!(
            acc,
            "{}: <t:{}:R>", 
            sta.station_name.clone(),
            chrono::DateTime::timestamp(&Chicago.from_local_datetime(&sta.arrival_time).unwrap())).unwrap();
          // embed = embed.field(, format!("<t:{}:R>", , true);
          // .chrono::TimeZoneDateTime::<chrono::Local>::from_naive_utc_and_offset(sta.arrT.checked_sub_offset(chrono::FixedOffset::west_opt(6*3600).unwrap()).expect("Time is gone I guess"), chrono::FixedOffset::west_opt(6*3600).unwrap()).timestamp()), true);
          acc
        });
        return CreateInteractionResponseMessage::new()
          .content(
            format!("Found {} upcoming stations for Train #{}", val.len(), run)).embed(embed.description(desc));
      },
      Err(err) => {
        return CreateInteractionResponseMessage::new().content(format!("Could not find the specified train: {err:?}" ));
      }
    }
  }
  CreateInteractionResponseMessage::new().content("Options not provided.".to_string())
}

pub fn register() -> CreateCommand {
  let run_option = CreateCommandOption::new(serenity::all::CommandOptionType::Integer, "train_number", "Run number of the train").required(true);
  CreateCommand::new("get_train")
    .add_option(run_option)
    .add_integration_type(serenity::all::InstallationContext::Guild)
    .add_integration_type(serenity::all::InstallationContext::User)
    .description("Gets information about a given train.")
}