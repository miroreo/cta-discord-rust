use chrono::NaiveTime;
use gtfs_structures::RouteType;
use serenity::all::{
  Context, CreateCommandOption, CreateEmbed, CreateInteractionResponseMessage, Interaction,
  ResolvedValue,
};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::commands::bus;
use crate::cta::alerts::{Alert, AlertsError, AlertsOptions};
use crate::{cta, CTAShared};

pub async fn run<'a>(
  ctx: &Context,
  options: &'a [ResolvedOption<'a>],
) -> CreateInteractionResponseMessage {
  let data = ctx.data.read().await;
  let data = data.get::<CTAShared>().expect("no shared data");

  let alerts = get_alerts(ctx).await;
  match alerts {
    Ok(alerts_list) => {
      let alerts_texts: Vec<String> = alerts_list
        .iter()
        .map(|a| {
          let start_time: i64 = match a.event_start {
            cta::alerts::DateOrDateTime::DateTime(naive_date_time) => naive_date_time
              .and_local_timezone(chrono_tz::America::Chicago)
              .earliest()
              .unwrap()
              .timestamp(),
            cta::alerts::DateOrDateTime::Date(naive_date) => naive_date
              .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
              .and_local_timezone(chrono_tz::America::Chicago)
              .earliest()
              .unwrap()
              .timestamp(),
          };
          let end_time: Option<i64> = match a.event_end {
            Some(cta::alerts::DateOrDateTime::DateTime(naive_date_time)) => Some(
              naive_date_time
                .and_local_timezone(chrono_tz::America::Chicago)
                .earliest()
                .unwrap()
                .timestamp(),
            ),
            Some(cta::alerts::DateOrDateTime::Date(naive_date)) => Some(
              naive_date
                .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
                .and_local_timezone(chrono_tz::America::Chicago)
                .earliest()
                .unwrap()
                .timestamp(),
            ),
            None => None,
          };
          // match a.event_end {
          //   Some(dateTime) => {
          //     Some(dateTime
          //         .and_local_timezone(chrono_tz::America::Chicago)
          //         .earliest()
          //         .unwrap()
          //         .timestamp_millis())
          //   },
          //   None => None
          // };
          let end_fmt = format!("<t:{}:f>", end_time.clone().unwrap_or(0));
          format!(
            "**{}**\n(<t:{}:f> - {})\n{}",
            a.headline,
            start_time,
            if a.tbd { "TBD" } else { &end_fmt },
            a.short_description
          )
        })
        .collect();
      CreateInteractionResponseMessage::new().add_embed(
        CreateEmbed::new()
          .title(format!("Active CTA Alerts"))
          .description(alerts_texts.join("\n\n")),
      )
    }
    Err(AlertsError::NoAlerts) => {
      CreateInteractionResponseMessage::new().content(format!("No Current Alerts").to_string())
    }
    Err(e) => CreateInteractionResponseMessage::new()
      .content(format!("Error getting alerts: {e}").to_string())
      .ephemeral(true),
  }
}

pub fn register() -> CreateCommand {
  CreateCommand::new("alerts")
    .description("Gets current CTA Rail Service Alerts")
    .add_integration_type(serenity::all::InstallationContext::User)
    .add_integration_type(serenity::all::InstallationContext::Guild)
}

async fn get_alerts(ctx: &Context) -> Result<Vec<cta::alerts::Alert>, AlertsError> {
  let data = ctx.data.read().await;
  let data = data.get::<CTAShared>().expect("no shared data");
  let route_ids: Vec<String> = data
    .gtfs
    .gtfs_data
    .routes
    .iter()
    .filter_map(|f| match f.1.route_type {
      RouteType::Subway => Some(f.0.clone()),
      _ => None,
    })
    .collect();
  cta::alerts::get_alerts(AlertsOptions {
    route_ids: route_ids.clone(),
    active_only: Some(true),
    planned: Some(true),
    accessibility: Some(false),
    by_start_date: None,
    recent_days: None,
  })
  .await
  .unwrap();
  cta::alerts::get_alerts(AlertsOptions {
    route_ids,
    active_only: Some(true),
    planned: Some(true),
    accessibility: Some(false),
    by_start_date: None,
    recent_days: None,
  })
  .await
}
