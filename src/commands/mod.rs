use serenity::all::Context;
use serenity::all::Command;
pub mod ping;
pub mod route_name;
pub mod get_train;

pub async fn initialize(context: Context) {
  println!("Initializing Global Commands");
  if let Err(why) = Command::set_global_commands(&context.http, vec![
    ping::register(),
    route_name::register(),
    get_train::register(),
  ]).await {
    println!("Error adding commands: {why:?}");
  }
}