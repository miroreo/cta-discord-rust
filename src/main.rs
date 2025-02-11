mod commands;
mod cta;
extern crate dotenv;

use dotenv::dotenv;
use serenity::all::{Command, CreateInteractionResponse, Interaction};
use std::env;

use serenity::{async_trait, FutureExt};
use serenity::model::prelude::{Message, Ready};
use serenity::prelude::*;
use serenity::all::CreateInteractionResponseMessage;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, ctx: Context, r: Ready) {
    println!("Connected as {}", r.user.name);

    commands::initialize(ctx).await;
  }
  async fn message(&self, ctx: Context, msg: Message) {
    if msg.content == "!ping" {
      if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("Error sending message: {why:?}");
      }
    }
  }
  
  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
      // println!("Recieved command interaction: {command:#?}");
      
      let content = match command.data.name.as_str() {
        "ping" => Some(commands::ping::run(&command.data.options())),
        "route_name" => Some(commands::route_name::run(&command.data.options()).await),
        "get_train" => Some(commands::get_train::run(&command.data.options()).await),
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
  dotenv().ok();
  tokio::spawn(cta::load_gtfs());
  let ctaTT = cta::traintracker::TrainTracker::new(env::var("CTA_RAIL_API_KEY").expect("CTA_RAIL_API_KEY not found.").as_str());
  ctaTT.train_next_stations(618).await;
  // login with bot token from the environment
  let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
  // Set gateway intents
  let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;
  
  // Create new instaance of the Client
  let mut client = Client::builder(&token, intents)
    .event_handler(Handler)
    .await.expect("Error creating client.");

  // start listening for events by starting a single shard
  if let Err(why) = client.start().await {
    println!("Client error: {why:?}");
  }
}
