use std::thread::sleep;
use std::time::Duration;

use chrono::DateTime;
use serenity::all::{ChannelId, Context, CreateMessage};

use crate::{cta::{self, alerts::{AlertsError, AlertsOptions, DateOrDateTime}}, db, CTAShared};

pub async fn watch(ctx: Context) {
  // send_alert(ctx.clone(), "Test".to_string()).await;
  loop {
    check(ctx.clone()).await;

    sleep(Duration::from_secs(10));
  }
}
async fn check(ctx: Context) {
  let data = ctx.data.read().await;
  let data = data.get::<CTAShared>().expect("no shared data");
  let alerts = cta::alerts::get_active_alerts(AlertsOptions{
    route_ids: ["r", "b", "grn", "org", "brn", "p", "pink", "y"].iter().map(|s| s.to_string()).collect(),
    active_only: Some(true),
    accessibility: Some(false),
    planned: Some(false),
    by_start_date: None,
    recent_days: None,
  }).await;
  
  match alerts {
    Ok(_list) => {
      if _list.len() == 0 {
        println!("No Alerts.");
      } else {
        println!("OK. {} Alerts.", _list.len());
        for f in _list.iter() {
          dbg!(f);
        }
      }
    },
    Err(e) => {
      println!("Error: {e}");
    }
  }

  // dbg!(alerts.len());
}

async fn send_alert(ctx: Context, msg: String) {
  let guilds = match db::get_guilds(std::env::var("DATABASE_URL").unwrap().to_string()).await {
    Ok(val) => {
      for guild in val.iter() {
        match guild.alert_channel {
          Some(chan) => {
            ChannelId::from(chan as u64).send_message(ctx.http.clone(), CreateMessage::new().content("Test")).await;
          },
          None => {},
        };
        // ctx.http.send_message(guild.alert_channel.unwrap().into(), vec![], "Test").await;
      }
    },
    Err(err) => {

    }
  };
}