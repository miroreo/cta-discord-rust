use serenity::all::Context;
use serenity::all::Command;
pub mod ping;
pub mod route_name;
pub mod get_train;
pub mod bus;

pub async fn initialize(context: Context) {
  println!("Initializing Global Commands");
  if let Err(why) = Command::set_global_commands(&context.http, vec![
    ping::register(),
    route_name::register(),
    get_train::register(),
    bus::register()
  ]).await {
    println!("Error adding commands: {why:?}");
  }
  println!("Initialized Global Commands");
}