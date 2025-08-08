use std::str::FromStr;

use serenity::all::Context;
use serenity::all::Command;
use serenity::all::Guild;
use serenity::all::GuildId;
pub mod ping;
pub mod route_name;
pub mod get_train;
pub mod bus;
pub mod arrivals;
pub mod autocomplete;
pub mod broadcast;

pub struct BotCommand {
    
}

pub async fn initialize(context: Context) {
  println!("Registering global commands");
  if let Err(why) = Command::set_global_commands(&context.http, vec![
    ping::register(),
    route_name::register(),
    get_train::register(),
    bus::register(),
    arrivals::register()
  ]).await {
    println!("Could not register global commands: {why:?}");
  }
  match std::env::var("ADMIN_GUILDS") {
    Ok(val) => {
      let ids = val.split(',')
        .flat_map(|v| GuildId::from_str(v))
        .collect::<Vec<GuildId>>();
      for id in ids {
        if let Err(why) = id.set_commands(&context.http, vec![
          broadcast::register()
        ]).await {
          println!("Could not register admin commands for guild {}: {why}", id.to_string());
        }
      }
    },
    Err(std::env::VarError::NotPresent) => {
      println!("ADMIN_GUILDS environment variable not set. Skipping adding Admin Commands");
    },
    Err(std::env::VarError::NotUnicode(_)) => {
      println!("ADMIN_GUILDS environment variable has invalid data. Skipping adding Admin Commands");
    }
  }
  println!("Finished registering global commands");
}

