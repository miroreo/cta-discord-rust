use crate::arrivaldisplay::{self, Arrival, ArrivalDisplayError};
use crate::cta::traintracker::ArrivalsParameters;
use crate::{cta, util, CTAShared};

use gtfs_structures::Stop;
use serenity::builder::CreateCommand;
use serenity::model::application::{ResolvedOption, ResolvedValue};
use serenity::all::{ComponentInteraction, ComponentInteractionDataKind, Context, CreateAttachment, CreateButton, CreateCommandOption, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, InteractionResponseFlags};

pub async fn run<'a>(ctx: &Context, options: &'a[ResolvedOption<'a>]) -> CreateInteractionResponseMessage {
  let data = ctx.data.read().await;
  let data = data.get::<CTAShared>().expect("no shared data");
  let tt = &data.traintracker;
  if let Some(ResolvedOption {  
    value: ResolvedValue::String(val), ..
  }) = options.first()
  {
    return arrivals_command(ctx, val).await;
  }
  CreateInteractionResponseMessage::new().content("Options not provided.".to_string())
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
    .split_once("/stationName/")
    .unwrap_or(("","")).1;
  CreateInteractionResponse::UpdateMessage(arrivals_command(ctx, stop_name).await)
}

async fn arrivals_command(ctx: &Context, search: &str) -> CreateInteractionResponseMessage {
  let data = ctx.data.read().await;
  let data = data.get::<CTAShared>().expect("no shared data");

  let tt = &data.traintracker;
  let gtfs = &data.gtfs;

  let found_stop_ids: Vec<String> = match gtfs.search_stops(search) {
    Some(stops) => stops,
    None => Vec::new()
  };

  let mut stations: Vec<Stop> = Vec::new();
  found_stop_ids.into_iter().for_each(|stop_id| {
    let stop = gtfs.gtfs_data.get_stop(stop_id.as_str()).expect("Search result is invalid.");
    // stop IDs over 40000 are train stations.
    let id = str::parse::<i32>(&stop.id).expect("Couldn't parse Stop ID as i32.");
    if id < 40000 || id > 50000 {
      return;
    }
    stations.push(stop.clone());
  });
  if stations.len() > 25 {
    return CreateInteractionResponseMessage::new()
      .content(format!("Too many results found for that station name. Please narrow your search."))
      .flags(InteractionResponseFlags::EPHEMERAL);
  } else if stations.len() > 1 {
    let mut select_menu_options: Vec<CreateSelectMenuOption> = stations.into_iter().map(|stop| {
      let name = stop.name.unwrap_or_else(|| format!("Station ID {}", stop.id));
      CreateSelectMenuOption::new(name.clone(), name)
    }).collect();
    // .keys().map(|stop_name| {
    //   CreateSelectMenuOption::new(stop_name, stop_name)
    // }).collect();
    return CreateInteractionResponseMessage::new()
      .content("Multiple stations found for that query. Please select one")
      .select_menu(CreateSelectMenu::new("arrivals:select", CreateSelectMenuKind::String { options: select_menu_options.into() })
        .min_values(1)
        .max_values(1))
  } else if stations.len() == 0 {
    return CreateInteractionResponseMessage::new()
      .content(format!("No stations found for that search."))
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
      let response = CreateInteractionResponseMessage::new();
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
        arrivaldisplay::train(
          format!("Upcoming Arrivals for {}", station.name.clone().unwrap_or_else(|| format!("Station ID {}", station.id))),
          arrivals.into_iter().take(8).collect()));
      match png_data {
        Ok(data) => {
          return CreateInteractionResponseMessage::new()
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
          println!("Error encoding arrival board: {}", err);
          return CreateInteractionResponseMessage::new()
            .content(format!("Error creating Arrival Board. Please try again later."))
            .flags(InteractionResponseFlags::EPHEMERAL);
        }
        Err(ArrivalDisplayError::FileError(err)) => {
          println!("Error accessing arrival board files: {}", err);
          return CreateInteractionResponseMessage::new()
            .content(format!("Error creating Arrival Board. Please try again later."))
            .flags(InteractionResponseFlags::EPHEMERAL);
        }
      }
    },
    Err(e) => {
      return CreateInteractionResponseMessage::new()
        .content(format!("Error getting arrivals: {e}"))
        .flags(InteractionResponseFlags::EPHEMERAL);
    }
  }
}

pub fn register() -> CreateCommand {
  let mut run_option = CreateCommandOption::new(serenity::all::CommandOptionType::String, "station_name", "Station Name Search").required(true);
  CreateCommand::new("arrivals")
    .add_option(run_option)
    .add_integration_type(serenity::all::InstallationContext::Guild)
    .add_integration_type(serenity::all::InstallationContext::User)
    .description("Gets information about a given train.")
}