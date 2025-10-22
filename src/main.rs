#![warn(clippy::pedantic)]
mod arrivaldisplay;
mod commands;
mod cta;
mod db;
mod util;
mod watcher;
extern crate dotenv;

use dotenv::dotenv;
use env_logger::Logger;
use serenity::all::{
  CommandOptionType, CreateAutocompleteResponse, CreateInteractionResponse,
  CreateInteractionResponseMessage, Interaction,
};
use sqlx::{migrate, Connection, PgConnection, Pool, Postgres};
use std::env;
use std::sync::Arc;

use serenity::async_trait;
use serenity::model::prelude::Ready;
use serenity::prelude::*;

pub struct CTASharedData {
  pub traintracker: cta::traintracker::TrainTracker,
  pub bustracker: cta::bustracker::BusTracker,
  pub stations: cta::stations::CtaStations,
  pub gtfs: cta::gtfs::CtaGTFS,
  pub db: Pool<Postgres>,
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
    println!("Connected to Discord as {}", r.user.name);

    init_shared(&ctx).await;
    commands::initialize(ctx.clone()).await;

    tokio::spawn(watcher::watch(ctx.clone()));
  }
  // async fn message(&self, ctx: Context, msg: Message) {
  //   if msg.content == "!ping" {
  //     if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
  //       println!("Error sending message: {why:?}");
  //     }
  //   }
  // }

  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction.clone() {
      // println!("Recieved command interaction: {command:#?}");

      let content = match command.data.name.as_str() {
        "ping" => Some(commands::ping::run(&command.data.options())),
        "route_name" => Some(commands::route_name::run(&ctx, &command.data.options()).await),
        "get_train" => Some(commands::get_train::run(&ctx, &command.data.options()).await),
        "bus" => Some(commands::bus::run(&ctx, &command.data.options()).await),
        "arrivals" => Some(commands::arrivals::run(&ctx, &command.data.options()).await),
        "broadcast" => {
          Some(commands::broadcast::run(&ctx, &command.data.options(), &interaction).await)
        }
        _ => {
          Some(CreateInteractionResponseMessage::new().content("not implemented yet.".to_string()))
        }
      };
      // let content = match command.data.
      if let Some(content) = content {
        let data = content;
        let builder = CreateInteractionResponse::Message(data);
        if let Err(why) = command.create_response(&ctx.http, builder).await {
          println!("Cannot respond to slash command: {why}");
        }
      }
    }
    // Check if it's one of the special cases where
    if let Interaction::Component(component) = interaction.clone() {
      let content = if component
        .data
        .custom_id
        .as_str()
        .starts_with("bus_arrivals:select")
      {
        Some(commands::bus::arrivals_select(&ctx, &component).await)
      } else if component
        .data
        .custom_id
        .as_str()
        .starts_with("bus_arrivals:refresh")
      {
        Some(commands::bus::arrivals_refresh(&ctx, &component).await)
      } else if component
        .data
        .custom_id
        .as_str()
        .starts_with("arrivals:select")
      {
        Some(commands::arrivals::select(&ctx, &component).await)
      } else if component
        .data
        .custom_id
        .as_str()
        .starts_with("arrivals:refresh")
      {
        Some(commands::arrivals::refresh(&ctx, &component).await)
      } else {
        Some(CreateInteractionResponse::Message(
          CreateInteractionResponseMessage::new().content("not implemented yet."),
        ))
      };
      if let Some(content) = content {
        let data = content;
        if let Err(why) = component.create_response(&ctx.http, data).await {
          println!("Cannot respond to slash command: {why}");
        }
      }
    }
    if let Interaction::Autocomplete(autocomplete) = interaction {
      let content = Some(commands::autocomplete::handle(&ctx, &autocomplete).await);
      if let Some(content) = content {
        let data = content;
        if let Err(why) = autocomplete.create_response(&ctx.http, data).await {
          println!("Cannot respond to autocomplete: {why}");
        }
      }
    } else {
      Some(CreateInteractionResponse::Autocomplete(
        CreateAutocompleteResponse::new(),
      ));
    }
  }
}

#[tokio::main]
async fn main() {
  dotenv().ok();
  let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

  let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

  let mut client = Client::builder(&token, intents)
    .event_handler(Handler)
    .await
    .expect("Error creating client.");

  if let Err(why) = client.start().await {
    println!("Client error: {why:?}");
  }
}

async fn init_shared(ctx: &Context) {
  println!("Initializing shared data.");

  let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");
  let db: Pool<Postgres> = Pool::connect(&db_url)
    .await
    .expect("Couldn't reach database");
  migrate!("./migrations")
    .run(&db)
    .await
    .expect("Couldn't run migrations");

  // let ctaTT = cta::traintracker::TrainTracker::new();
  let initial_cta_shared = CTASharedData {
    bustracker: cta::bustracker::BusTracker::new(
      env::var("CTA_BUS_API_KEY")
        .expect("CTA_BUS_API_KEY not found!")
        .as_str(),
    ),
    gtfs: cta::gtfs::CtaGTFS::new().await,
    traintracker: cta::traintracker::TrainTracker::new(
      env::var("CTA_RAIL_API_KEY")
        .expect("CTA_RAIL_API_KEY not found.")
        .as_str(),
    ),
    stations: cta::stations::CtaStations::new().await,
    db, // db_connection: PgConnection::connect(
        //   ).await
        //   .expect("Couldn't connect to database.")
  };
  let mut data = ctx.data.write().await;
  data.insert::<CTAShared>(Arc::new(initial_cta_shared));
  println!("Initialized Shared Data.");
}
