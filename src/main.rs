mod commands;
mod cta;
extern crate dotenv;

use cta::gtfs::CtaGTFS;
use cta::stations::CtaStations;
use dotenv::dotenv;
use serenity::all::{Command, CreateInteractionResponse, Interaction};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;

use serenity::{async_trait, FutureExt};
use serenity::model::prelude::{Message, Ready};
use serenity::prelude::*;
use serenity::all::CreateInteractionResponseMessage;

pub struct CTASharedData {
  pub traintracker: cta::traintracker::TrainTracker,
  pub bustracker: cta::bustracker::BusTracker,
  pub stations: cta::stations::CtaStations,
  pub gtfs: cta::gtfs::CtaGTFS,
  // info: Info,
}
pub struct CTAShared;
impl serenity::prelude::TypeMapKey for CTAShared {
  type Value = Arc<CTASharedData>;
}
// struct Info {
//   gtfs_last_updated: chrono::DateTime<chrono::FixedOffset>,
//   start_time: chrono::DateTime<chrono::FixedOffset>,
  
// }

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, ctx: Context, r: Ready) {
    println!("Connected as {}", r.user.name);

    commands::initialize(ctx.clone()).await;
    init_shared(ctx).await;
  }
  // async fn message(&self, ctx: Context, msg: Message) {
  //   if msg.content == "!ping" {
  //     if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
  //       println!("Error sending message: {why:?}");
  //     }
  //   }
  // }
  
  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
      // println!("Recieved command interaction: {command:#?}");
      
      let content = match command.data.name.as_str() {
        "ping" => Some(commands::ping::run(&command.data.options())),
        "route_name" => Some(commands::route_name::run(&ctx, &command.data.options()).await),
        "get_train" => Some(commands::get_train::run(&ctx, &command.data.options()).await),
        "bus" => Some(commands::bus::run(&ctx, &command.data.options()).await),
        _ => Some(CreateInteractionResponseMessage::new().content("not implemented yet.".to_string())),
      };
      if let Some(content) = content {
        let data = content;
        let builder = CreateInteractionResponse::Message(data);
        if let Err(why) = command.create_response(&ctx.http, builder).await {
          println!("Cannot respond to slash command: {why}");
        }
      }
    }
  }
}

#[tokio::main]
async fn main() {
  // #![warn(clippy::pedantic)]
  
  dotenv().ok();
  let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

  let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

  let mut client = Client::builder(&token, intents)
    .event_handler(Handler)
    .await.expect("Error creating client.");

  if let Err(why) = client.start().await {
    println!("Client error: {why:?}");
  }
  
}

async fn init_shared(ctx: Context) {
  println!("Initializing Shared Data.");
  // let ctaTT = cta::traintracker::TrainTracker::new();
  let initial_cta_shared = CTASharedData {
    bustracker: cta::bustracker::BusTracker::new(env::var("CTA_BUS_API_KEY").expect("CTA_BUS_API_KEY not found!").as_str()),
    gtfs: cta::gtfs::CtaGTFS::new().await,
    traintracker: cta::traintracker::TrainTracker::new(env::var("CTA_RAIL_API_KEY").expect("CTA_RAIL_API_KEY not found.").as_str()),
    stations: cta::stations::CtaStations::new().await
  };
  let mut data = ctx.data.write().await;
  data.insert::<CTAShared>(Arc::new(initial_cta_shared));
  println!("Initialized Shared Data.");
}