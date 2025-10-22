use std::thread::sleep;
use std::time::Duration;

use chrono::DateTime;
use serenity::all::{ChannelId, Context, CreateMessage};
use sqlx::{Executor, Postgres};

use crate::{
  cta::{
    self,
    alerts::{Alert, AlertsError, AlertsOptions, DateOrDateTime},
  },
  db::{self, DBAlert},
  CTAShared,
};

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
  let alerts = cta::alerts::get_active_alerts(AlertsOptions {
    route_ids: ["r", "blue", "grn", "org", "brn", "p", "pink", "y"]
      .iter()
      .map(|s| s.to_string())
      .collect(),
    active_only: Some(true),
    accessibility: Some(false),
    planned: Some(true),
    by_start_date: None,
    recent_days: None,
  })
  .await;

  match alerts {
    Ok(list) => {
      if !list.is_empty() {
        println!("Found {} alerts!", list.len());

        let in_db = db::get_alerts_with_ids(&data.db, list.iter().map(|f| f.id).collect()).await;
        for f in &list {
          dbg!(f);
        }
        match in_db {
          Ok(val) => {
            for alert in val {
              match list.iter().find(|x| x.id == alert.alert_id) {
                Some(a) => {
                  // if should_update(a.clone(), alert)
                }
                None => {}
              };
            }
          }
          Err(e) => {
            println!("Error getting alerts in database: {e}");
          }
        }
      }
    }
    Err(e) => {
      println!("Error: {e}");
    }
  }

  // dbg!(alerts.len());
}

// fn compare
fn should_update(api_alert: Alert, db_alert: DBAlert) -> bool {
  if api_alert.id != db_alert.alert_id {
    return false;
  };
  if api_alert.headline != db_alert.headline {
    return true;
  };
  if api_alert.short_description != db_alert.short_description {
    return true;
  };
  false
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
