use crate::arrivaldisplay::{self, Arrival, ArrivalDisplayError};
use crate::cta::traintracker::ArrivalsParameters;
use crate::{cta, util, CTAShared};

use gtfs_structures::Stop;
use serenity::builder::CreateCommand;
use serenity::model::application::{ResolvedOption, ResolvedValue};
use serenity::all::{ComponentInteraction, ComponentInteractionDataKind, Context, CreateAttachment, CreateButton, CreateCommandOption, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, InteractionResponseFlags};

pub async fn run<'a>(ctx: &Context, options: &'a[ResolvedOption<'a>]) -> CreateInteractionResponseMessage {
  if let Some(ResolvedOption {  
    value: ResolvedValue::String(val), ..
  }) = options.first()
  {
    return arrivals_command(ctx, val).await;
  }
  CreateInteractionResponseMessage::new().content("Options not provided.".to_string())
}

pub async fn select(ctx: &Context, component: &ComponentInteraction) -> CreateInteractionResponse {
  if let ComponentInteractionDataKind::StringSelect { values } = &component.data.kind {
    CreateInteractionResponse::UpdateMessage(arrivals_command(ctx, values.first().unwrap_or(&String::new()).as_str()).await)
  }
  else {
    CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().ephemeral(true).content("An error occured while attempting to select the stop."))
  }
}

pub async fn refresh(ctx: &Context, component: &ComponentInteraction) -> CreateInteractionResponse {
  let last_time = component.message.edited_timestamp
  .unwrap_or(component.message.timestamp).clone().with_timezone(&chrono::Utc);
  if chrono::Utc::now().signed_duration_since(last_time).num_seconds() <= 30 {
    return CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().ephemeral(true).content("Please wait at least 30 seconds before refreshing."))
  }
  let stop_name = component.data.custom_id
    .split_once("/stationName/")
    .unwrap_or(("","")).1;
  CreateInteractionResponse::UpdateMessage(arrivals_command(ctx, stop_name).await)
}

async fn arrivals_command(ctx: &Context, search: &str) -> CreateInteractionResponseMessage {
  let data = ctx.data.read().await;
  let data = data.get::<CTAShared>().expect("no shared data");

  let tt = &data.traintracker;
  let gtfs = &data.gtfs;

  let found_stop_ids: Vec<String> = gtfs.search_stops(search).unwrap_or_default();

  let mut stations: Vec<Stop> = Vec::new();
  for stop_id in found_stop_ids {
    let stop = gtfs.gtfs_data.get_stop(stop_id.as_str()).expect("Search result is invalid.");
    // stop IDs over 40000 are train stations.
    let id = str::parse::<i32>(&stop.id).expect("Couldn't parse Stop ID as i32.");
    if !(40000..=50000).contains(&id) {
      continue;
    }
    stations.push(stop.clone());
  }
  if stations.len() > 25 {
    return CreateInteractionResponseMessage::new()
      .content("Too many results found for that station name. Please narrow your search.".to_string())
      .flags(InteractionResponseFlags::EPHEMERAL);
  } else if stations.len() > 1 {
    let select_menu_options: Vec<CreateSelectMenuOption> = stations.into_iter().map(|stop| {
      let name = stop.name.unwrap_or_else(|| format!("Station ID {}", stop.id));
      CreateSelectMenuOption::new(name.clone(), name)
    }).collect();
    // .keys().map(|stop_name| {
    //   CreateSelectMenuOption::new(stop_name, stop_name)
    // }).collect();
    return CreateInteractionResponseMessage::new()
      .content("Multiple stations found for that query. Please select one")
      .select_menu(CreateSelectMenu::new("arrivals:select", CreateSelectMenuKind::String { options: select_menu_options })
        .min_values(1)
        .max_values(1))
  } else if stations.is_empty() {
    return CreateInteractionResponseMessage::new()
      .content("No stations found for that search.".to_string())
      .flags(InteractionResponseFlags::EPHEMERAL);
  }
  let station = stations.first().expect("No first station. This should not be possible to reach.");
  let predictions = tt.arrivals(ArrivalsParameters{
    id: cta::traintracker::MapOrStopID::MapID { mapid: stations.first().unwrap().id.parse().unwrap() },
    max: None,
    rt: None
  }).await;
  match predictions {
    Ok(prds) => {
      let arrivals: Vec<Arrival> = prds.into_iter().map(|prd| {
        Arrival{
          destination_name: prd.destination_name,
          route: prd.route.into(),
          countdown: util::countdown(
            util::minutes_until(prd.arrival_time.and_local_timezone(chrono_tz::America::Chicago).unwrap())),
          is_scheduled: prd.is_scheduled,
          train_number: prd.run_number
        }
      }).collect();
      let png_data = arrivaldisplay::render_doc(
        &arrivaldisplay::train(
          format!("Upcoming Arrivals for {}", station.name.clone().unwrap_or_else(|| format!("Station ID {}", station.id))),
          &arrivals[0..8.min(arrivals.len())]));
      match png_data {
        Ok(data) => {
          CreateInteractionResponseMessage::new()
            .content(format!("Arrival Board Generated <t:{}:R>", chrono::Local::now().timestamp()))
            .embed(CreateEmbed::new()
              .title(format!("Arrivals for {}", station.name.clone().unwrap_or_else(|| format!("Station ID {}", station.id))))
              .image("attachment://arrivals.png"))
            .add_file(CreateAttachment::bytes(data, "arrivals.png"))
            .components(Vec::new())
            .button(CreateButton::new(format!("arrivals:refresh/stationName/{}", station.name.clone().unwrap()))
            .style(serenity::all::ButtonStyle::Primary)
            .label("Refresh"))
        },
        Err(ArrivalDisplayError::EncodingError(err)) => {
          println!("Error encoding arrival board: {err}");
          CreateInteractionResponseMessage::new()
            .content("Error creating Arrival Board. Please try again later.".to_string())
            .flags(InteractionResponseFlags::EPHEMERAL)
        }
        Err(ArrivalDisplayError::FileError(err)) => {
          println!("Error accessing arrival board files: {err}");
          CreateInteractionResponseMessage::new()
            .content("Error creating Arrival Board. Please try again later.".to_string())
            .flags(InteractionResponseFlags::EPHEMERAL)
        }
      }
    },
    Err(e) => {
      CreateInteractionResponseMessage::new()
        .content(format!("Error getting arrivals: {e}"))
        .flags(InteractionResponseFlags::EPHEMERAL)
    }
  }
}

pub fn register() -> CreateCommand {
  // let station_name_auto = CreateCommandOption::new(serenity::all::CommandOptionType::String, "station_name", "Station Name")
  let station_name = CreateCommandOption::new(serenity::all::CommandOptionType::String, "station_name", "Station Name Search").required(true).set_autocomplete(true);
  CreateCommand::new("arrivals")
    .add_option(station_name)
    .add_integration_type(serenity::all::InstallationContext::Guild)
    .add_integration_type(serenity::all::InstallationContext::User)
    .description("Gets information about a given train.")
}