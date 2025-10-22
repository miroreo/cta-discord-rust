use std::env::VarError;
use std::str::FromStr;

use serenity::all::{
  ChannelId, Context, CreateCommandOption, CreateInteractionResponseMessage, GuildId, Interaction,
  InteractionContext, InteractionResponseFlags, ResolvedValue, UserId,
};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::{db, CTAShared};

pub async fn run<'a>(
  ctx: &Context,
  options: &'a [ResolvedOption<'a>],
  interaction: &Interaction,
) -> CreateInteractionResponseMessage {
  if let Some(ResolvedOption {
    value: ResolvedValue::String(val),
    ..
  }) = options.first()
  {
    match ctx.http.get_current_application_info().await {
      Ok(info) => {
        if let Some(owner) = info.owner {
          if interaction.as_command().unwrap().user.eq(&owner) {
            let (success, fail) = broadcast(ctx, val).await;
            CreateInteractionResponseMessage::new()
              .content(format!(
                "Successfully broadcasted message to {success} guilds ({fail} failures)"
              ))
              .ephemeral(true)
          } else {
            CreateInteractionResponseMessage::new()
              .content("You are not the bot owner!".to_string())
              .ephemeral(true)
          }
        } else {
          println!("There is no bot owner!");
          CreateInteractionResponseMessage::new()
            .content("Error attempting to verify bot ownership".to_string())
            .ephemeral(true)
        }
      }
      Err(why) => {
        println!("Couldn't get current application info: {why}");
        CreateInteractionResponseMessage::new()
          .content("Error attempting to verify bot ownership".to_string())
          .ephemeral(true)
      }
    }
  } else {
    CreateInteractionResponseMessage::new()
      .content("No message specified!".to_string())
      .ephemeral(true)
  }
}

async fn broadcast(ctx: &Context, message: &str) -> (i32, i32) {
  let mut success = 0;
  let mut fail = 0;
  let data = ctx.data.read().await;
  let data = data.get::<CTAShared>().expect("no shared data");

  match db::get_subscribed_guilds(&data.db).await {
    Ok(guilds) => {
      for guild in guilds {
        match ChannelId::new(guild.alert_channel.unwrap_or_default() as u64)
          .say(&ctx.http, message)
          .await
        {
          Ok(_) => {
            success += 1;
          }
          Err(why) => {
            fail += 1;
            println!("Failed to broadcast to guild {}: {why}", guild.guild_id);
          }
        }
      }
    }
    Err(why) => {
      println!("Error reading subscribed guilds from database: {why}");
    }
  };
  (success, fail)
}

pub fn register() -> CreateCommand {
  CreateCommand::new("broadcast")
    .description("(Admin Command) Broadcast a message to all servers that subscribe to alerts.")
    .add_option(
      CreateCommandOption::new(
        serenity::all::CommandOptionType::String,
        "message",
        "Message to broadcast",
      )
      .required(true),
    )
}
