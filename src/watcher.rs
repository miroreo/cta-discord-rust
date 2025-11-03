use std::thread::sleep;
use std::time::Duration;

use chrono::NaiveTime;
use serenity::all::{ChannelId, Context, CreateEmbed, CreateEmbedAuthor, CreateMessage, Timestamp};
use sqlx::{Executor, Postgres};
use thiserror::Error;

use crate::{
  cta::{
    self,
    alerts::{Alert, AlertsOptions},
  },
  db::{self, DBAlert},
  CTAShared,
};

static ALERTS_ICON_URL: &str = "https://www.transitchicago.com/assets/1/16/DimRiderToolDesktop/quick-link-4.png?14576";

pub async fn watch(ctx: Context) {
  static INTERVAL_SECS: u64 = 10;
  println!("Alert watcher task spawned. Polling every {INTERVAL_SECS} seconds.");
  loop {
    check(ctx.clone()).await;
    sleep(Duration::from_secs(INTERVAL_SECS));
  }
}
async fn check(ctx: Context) {
  let data = ctx.data.read().await;
  let data = data.get::<CTAShared>().expect("no shared data");
  let alerts = cta::alerts::get_alerts(AlertsOptions {
    route_ids: ["red", "blue", "g", "org", "brn", "p", "pink", "y"]
      .iter()
      .map(std::string::ToString::to_string)
      .collect(),
    active_only: Some(true),
    accessibility: Some(false),
    planned: Some(true),
    by_start_date: None,
    recent_days: None,
  })
  .await;

  match alerts {
    Ok(api_alerts) => {
      if !api_alerts.is_empty() {
        // let db_alerts = db::get_alerts_with_ids(&data.db, &list.iter().map(|f| f.id).collect::<Vec<_>>()).await;
        // // println!("Found {} alerts!", list.len());
        // for f in &list {
        //   dbg!(f);
        // }

        match db::get_alerts_with_ids(&data.db, &api_alerts.iter().map(|f| f.id).collect::<Vec<_>>()).await {
          Ok(db_alerts) => {
            let untracked_alerts = new_alerts(&db_alerts, &api_alerts);
            let updated_alerts = alerts_should_update(&db_alerts, &api_alerts);            
            if !untracked_alerts.is_empty() {
              println!("{} untracked alerts found!", untracked_alerts.len());
            }
            // if !updated_alerts.is_empty() {
            //   println!("{} updated alerts found!", updated_alerts.len());
            // }
            for a in &untracked_alerts {
              let _ = trigger(&ctx, a.clone()).await;
            }

          }
          Err(e) => {
            println!("Error getting alerts in database: {e}");
          }
        }
        // This function is likely causing a lot of duplicate messages
        // match db::drop_alerts_not_with_ids(&data.db, &api_alerts.iter().map(|f| f.id).collect::<Vec<_>>()).await {
        //   Ok(rows) => {
        //     if rows.len() != 0 {
        //       println!("Dropped {} outdated alerts from the database", rows.len());
        //     }
        //   }
        //   Err(e) => {
        //     println!("Error dropping alerts: {e}");
        //   }
        // }
      }
    }
    Err(e) => {
      println!("Error: {e}");
    }
  }

  // dbg!(alerts.len());
}
fn new_alerts(db_alerts: &[DBAlert], current_alerts: &[Alert]) -> Vec<Alert> {
  current_alerts.iter().filter(|a| !db_alerts.iter().any(|dba| a.id == dba.alert_id)).cloned().collect()
}

fn alerts_should_update(db_alerts: &[DBAlert], current_alerts: &[Alert]) -> Vec<Alert> {
  // if it's got the same ID, different headline or short_description, update
  current_alerts.iter().filter(|a| {
    db_alerts.iter().any(|dba| {
      a.id == dba.alert_id &&
      (!a.headline.eq(&dba.headline) || !a.short_description.eq(&dba.short_description))
    })
  }).cloned().collect()
}
// async fn find_new(ctx: &Context, alerts: Vec<Alert>) -> Result<Vec<Alert>, sqlx::Error> {
//   let data = ctx.data.read().await;
//   let data = data.get::<CTAShared>().expect("no shared data");
//   let db_alerts =
//     db::get_alerts_with_ids(
//       &data.db,
//       &alerts
//         .iter()
//         .map(|a| a.id)
//         .collect::<Vec<_>>()).await?;
//   Ok(
//     alerts
//       .iter()
//       .filter(|a| !db_alerts.iter().any(|dba| a.id == dba.alert_id))
//       .cloned()
//       .collect(),
//   )
// }
// fn compare

#[derive(Error, Debug)]
pub enum PublishError {
  #[error("Failed to save alert to database.")]
  DBError(#[from] sqlx::Error),
  #[error("Failed to publish alert to Discord.")]
  DiscordError(#[from] serenity::Error),
  #[error("No Channel Set")]
  NoChannelError
}

async fn trigger(ctx: &Context, alert: Alert) -> Result<(), PublishError> {
  let data = ctx.data.read().await;
  let data = data.get::<CTAShared>().expect("no shared data");
  let mut publish_count = 0;
  let start_time: i64 = match alert.event_start {
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

  // send alerts via discord
  for guild in &db::get_subscribed_guilds(&data.db).await? {
    if let Some(chan_id) = guild.alert_channel {
      if ChannelId::from(chan_id as u64)
        .send_message(&ctx.http, 
          CreateMessage::new().add_embed(CreateEmbed::new()
            .author(CreateEmbedAuthor::new("CTA Alerts").icon_url(ALERTS_ICON_URL))
            .title(&alert.headline)
            .description(&alert.short_description)
            .timestamp(Timestamp::from_unix_timestamp(start_time).unwrap()))).await.is_ok() {
        publish_count += 1;
      };
    }
  };
  // save alert to database
  let _ = db::add_alert(&data.db, alert, &publish_count).await;

  Ok(())
}
async fn send_alert(ctx: Context, db: impl Executor<'_, Database = Postgres>, msg: String) {
  let guilds = match db::get_subscribed_guilds(db).await {
    Ok(val) => {
      for guild in &val {
        match guild.alert_channel {
          Some(chan) => {
            let msg = ChannelId::from(chan as u64)
              .send_message(ctx.http.clone(), CreateMessage::new().content("Test"))
              .await;
          }
          None => {}
        };
        // ctx.http.send_message(guild.alert_channel.unwrap().into(), vec![], "Test").await;
      }
    }
    Err(err) => {}
  };
}
