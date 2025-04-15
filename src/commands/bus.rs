use std::collections::HashMap;
use std::env;
use std::sync::Arc;

use chrono_tz::America::Chicago;
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use serenity::all::{ButtonKind, ComponentInteraction, ComponentInteractionData, ComponentInteractionDataKind, Context, CreateActionRow, CreateButton, CreateCommandOption, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, Embed, InteractionResponseFlags, ResolvedValue, SelectMenu};

use crate::{cta, CTAShared, CTASharedData};
use crate::cta::bustracker::{BusTracker, BusTrackerError, PredictionsParameters, VehiclesParameters, VidOrRt};

pub async fn run<'a>(ctx: &Context, options: &'a[ResolvedOption<'a>]) -> CreateInteractionResponseMessage {
  let data = ctx.data.read().await;
  let data = data.get::<CTAShared>().expect("no shared data");

  let bt = &data.bustracker;
  let gtfs = &data.gtfs;
  if let Some(option @ ResolvedOption {
    value: ResolvedValue::SubCommand(opts), ..
  }) = options.first()
  {
    match option.name {
      "vehicles" => {
        let first_option = opts.first().expect("no first option");
        if first_option.name.eq("route") {
          if let ResolvedOption {
            value: ResolvedValue::String(val), ..
          } = first_option {
            let route_codes: Vec<String> = (*val).split(",").map(|s| s.to_string()).collect();
            let vehicles = (bt).get_vehicles( VehiclesParameters { search: VidOrRt::Rt{Rt: route_codes.clone() }}).await;
            match vehicles {
              Ok(veh) => {
                let mut msg: String = veh.into_iter().map(|vehicle| {
                  format!("Bus {} Route {} ({}) to {}. {}\n", vehicle.vid, vehicle.rt, gtfs.get_route_name(&vehicle.rt).unwrap_or_else(|| "Unknown Route".to_string()), vehicle.des, String::from(cta::bustracker::BusTracker::tablockid_to_garage(&vehicle.tablockid)))
                }).collect();
                let mut truncated = false;
                if msg.len() > 4096 {
                  msg = msg.split_at(4096).0.to_string();
                  truncated = true;
                }
                return CreateInteractionResponseMessage::new()
                  .add_embed(
                    CreateEmbed::new()
                    .description(msg)
                    .title(format!("Routes: {}",route_codes.join(", "))));
              },
              Err(err) => {
                return CreateInteractionResponseMessage::new()
                  .content(format!("Error getting vehicles: {err}"))
                  .flags(InteractionResponseFlags::EPHEMERAL);
              }
            }
          }
        }
        if opts.first().expect("no first option").name.eq("buses") {
          
        }
      },
      "arrivals" => {
        let first_option = opts.first().expect("no first option");
        if first_option.name.eq("stop_name") {
          if let ResolvedOption {
            value: ResolvedValue::String(val), ..
          } = first_option {
            return arrivals_command(ctx, val).await;
          }
        }
      }
      _ => {
        
      }
    }
  } 
  CreateInteractionResponseMessage::new()
    .content("Internal error with command".to_string())
    .flags(InteractionResponseFlags::EPHEMERAL)
}

pub async fn arrivals_select(ctx: &Context, component: &ComponentInteraction) -> CreateInteractionResponse {
  if let ComponentInteractionDataKind::StringSelect { values } = &component.data.kind {
    CreateInteractionResponse::UpdateMessage(arrivals_command(ctx, values.first().unwrap_or(&"".to_string()).as_str()).await)
  }
  else {
    CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().ephemeral(true).content("An error occured while attempting to select the stop."))
  }
}

pub async fn arrivals_refresh(ctx: &Context, component: &ComponentInteraction) -> CreateInteractionResponse {
  let last_time = component.message.edited_timestamp
  .unwrap_or(component.message.timestamp).clone().with_timezone(&chrono::Utc);
  if chrono::Utc::now().signed_duration_since(last_time).num_seconds() <= 30 {
    return CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().ephemeral(true).content("Please wait at least 30 seconds before refreshing."))
  }
  let stop_name = component.data.custom_id
    .split_once("/stopName/")
    .unwrap_or(("","")).1;
  CreateInteractionResponse::UpdateMessage(arrivals_command(ctx, stop_name).await)
}
async fn arrivals_command(ctx: &Context, search: &str) -> CreateInteractionResponseMessage {
  let data = ctx.data.read().await;
  let data = data.get::<CTAShared>().expect("no shared data");

  let bt = &data.bustracker;
  let gtfs = &data.gtfs;

  let found_stop_ids: Vec<String> = match gtfs.search_stops(search) {
    Some(stops) => stops,
    None => Vec::new()
  };
  let mut stops: HashMap<String, Vec<String>> = HashMap::new();
  found_stop_ids.into_iter().for_each(|stp| {
    let stop = gtfs.gtfs_data.get_stop(stp.as_str()).expect("Search result is invalid.");
    if str::parse::<i32>(&stop.id).expect("Couldn't parse Stop ID as i32.") > 30000 {
      return;
    }
    let name = stop.name.as_deref().unwrap_or_default();
    let orig_val = stops.get(&name.to_string());
    if orig_val.is_some() {
      let mut new_val = orig_val.expect("Somehow orig_val is none.").to_owned();
      new_val.push(stp);
      stops.insert(name.to_string(), new_val.clone());
    }
    else {
      let mut new_val: Vec<String> = Vec::new();
      new_val.push(stp);
      stops.insert(name.to_string(), new_val);
    }
  });
  if stops.keys().into_iter().len() > 25 {
    return CreateInteractionResponseMessage::new()
      .content(format!("Too many results found for that stop name. Please narrow your search."))
      .flags(InteractionResponseFlags::EPHEMERAL);
  } else if stops.keys().into_iter().len() > 2 {
    let mut select_menu_options: Vec<CreateSelectMenuOption> = stops.keys().map(|stop_name| {
      CreateSelectMenuOption::new(stop_name, stop_name)
    }).collect();
    return CreateInteractionResponseMessage::new()
      .content("Multiple stops found for that query. Please select one")
      .select_menu(CreateSelectMenu::new("bus_arrivals:select", CreateSelectMenuKind::String { options: select_menu_options.into() })
        .min_values(1)
        .max_values(1))
  }
  for stop in stops.keys() {
    println!("{}", stop);
  }
  for id in stops.values() {
    println!("{}", id.join(","));
  }
  let stop_ids: Vec<String> = stops.values().into_iter().flat_map(|v| {v.to_owned().into_iter()}).collect();
  println!("{}", stop_ids.join(","));
  let predictions = bt.get_predictions(PredictionsParameters {
    search: cta::bustracker::StpidOrVid::StpId { stpid: stop_ids, rt: None }
  }).await;
  match predictions {
    Ok(prds) => {
      let response = CreateInteractionResponseMessage::new();
      return stops.keys().into_iter().fold(response, |response, stop_name| {
        let mut desc = "".to_string();
        prds.clone().into_iter().filter(|f| {
          f.stpnm.eq(stop_name)
        }).for_each(|prd| {
          let predicted_time = match BusTracker::parse_bustime(prd.prdtm.as_str()) {
            Ok(time) => {
              time.timestamp()
            },
            Err(e) => {
              println!("Error getting time from API: {e}");
              chrono::Utc::now().timestamp()
            }
          };
          desc.push_str(format!(
            "Route {} {} to {} <t:{}:R> (Bus #{})\n", 
            prd.rt,
            prd.rtdir,
            prd.des,
            predicted_time,
            prd.vid).as_str());
        });                    
        response.add_embed(
          CreateEmbed::new()
            .title(format!("Upcoming Arrivals for {}", stop_name))
            .description(desc))
          .content(format!("Arrival Board Generated <t:{}:R>", chrono::Local::now().timestamp()))
          .components(Vec::new())
          .button(CreateButton::new(format!("bus_arrivals:refresh/stopName/{}", stop_name))
          .style(serenity::all::ButtonStyle::Primary)
          .label("Refresh"))
      })
    },
    Err(e) => {
      return CreateInteractionResponseMessage::new()
        .content(format!("Error getting arrivals: {e}"))
        .flags(InteractionResponseFlags::EPHEMERAL);
    }
  }
}
  //  CreateInteractionResponseMessage::new().content("Pong!".to_string())


pub fn register() -> CreateCommand {
  CreateCommand::new("bus")
    .description("Get bus information.")
    .add_option(
      CreateCommandOption::new(
        serenity::all::CommandOptionType::SubCommand,
        "vehicles",
      "Get vehicle information")
      .add_sub_option(CreateCommandOption::new(
        serenity::all::CommandOptionType::String,
        "route",
        "Bus route numbers, separated by commas. ex: '20,49,X49,62'"
      )
      ))
    .add_option(
      CreateCommandOption::new(
        serenity::all::CommandOptionType::SubCommand,
        "arrivals",
        "Get upcoming bus arrivals for a stop"
      ).add_sub_option(CreateCommandOption::new(
        serenity::all::CommandOptionType::String,
        "stop_name",
        "Stop Name search"
      ).required(true))
    )
    .add_integration_type(serenity::all::InstallationContext::User)
    .add_integration_type(serenity::all::InstallationContext::Guild)
}