use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use serenity::all::{
  AutocompleteChoice, CommandData, CommandDataOption, CommandDataOptionValue, CommandInteraction,
  CommandOptionType, CommandType, Context, CreateAutocompleteResponse, CreateInteractionResponse,
  Interaction, ResolvedOption, ResolvedValue,
};

use crate::CTAShared;

pub async fn handle(ctx: &Context, interaction: &CommandInteraction) -> CreateInteractionResponse {
  match &interaction.data {
    CommandData {
      name: command_name,
      kind: CommandType::ChatInput,
      options: opts,
      ..
    } if command_name == "arrivals" => {
      if let Some(CommandDataOption {
        value:
          CommandDataOptionValue::Autocomplete {
            kind: CommandOptionType::String,
            value: search_string,
          },
        ..
      }) = opts.first()
      {
        return CreateInteractionResponse::Autocomplete(
          CreateAutocompleteResponse::new().set_choices(
            search_stations(ctx, search_string)
              .await
              .iter()
              .map(|res| AutocompleteChoice::new(res, res.clone()))
              .collect(),
          ),
        );
      }
      // return CreateInteractionResponse::Autocomplete(stations(ctx, interaction).await);
    }
    CommandData {
      name: command_name,
      kind: CommandType::ChatInput,
      options: opts,
      ..
    } if command_name == "bus" => {
      if let Some(CommandDataOption {
        value: CommandDataOptionValue::SubCommand(sub_data),
        ..
      }) = opts.first()
      {
        if let Some(CommandDataOption {
          name: opt_name,
          value:
            CommandDataOptionValue::Autocomplete {
              kind: CommandOptionType::String,
              value: search_string,
            },
          ..
        }) = sub_data.first()
        {
          if opt_name.as_str() == "stop_name" {
            return CreateInteractionResponse::Autocomplete(
              CreateAutocompleteResponse::new().set_choices(
                search_bus_stops(ctx, search_string)
                  .await
                  .iter()
                  .map(|res| AutocompleteChoice::new(res, res.clone()))
                  .collect(),
              ),
            );
          }
        }
      }
      return CreateInteractionResponse::Autocomplete(CreateAutocompleteResponse::new());
    }
    _ => {
      println!("Unknown autocomplete command: {}", interaction.data.name);
    }
  };
  CreateInteractionResponse::Autocomplete(CreateAutocompleteResponse::new())
  // // dbg!(interaction.data.name.as_str());
  //   if interaction.data.name.as_str() == "arrivals" {
  //     // dbg!(interaction);
  //     CreateInteractionResponse::Autocomplete(stations(ctx, interaction).await)

  //     // println!("Arrivals Autocomplete! {}", );
  //     // CreateInteractionResponse::Autocomplete(CreateAutocompleteResponse::new()
  //     //   .add_string_choice("Sox-35th", "Sox-35th")
  //     //   .add_string_choice("Sox-36th", "Sox-36th")
  //     //   .add_string_choice("Sox-37th", "Sox-37th"))
  //     // interaction.create_response(ctx.http.clone(), CreateInteractionResponse::Autocomplete(arrivals_autocomplete(ctx, interaction).clone())).await;

  //   // } else if interaction.data.name.as_str() == "bus" {
  //   //   CreateInteractionResponse::Autocomplete(())
  //   } else {
  //     println!("Unknown Autocomplete Command Name: {}", interaction.data.name.as_str());
  //     CreateInteractionResponse::Autocomplete(CreateAutocompleteResponse::new()
  //       .add_string_choice("ASox-35th", "Sox-35th")
  //       .add_string_choice("ASox-36th", "Sox-36th")
  //       .add_string_choice("ASox-37th", "Sox-37th"))
  //     // interaction.create_response(ctx.http, CreateInteractionResponse::Autocomplete(CreateAutocompleteResponse::new())).await;
  //   }
}

async fn search_stations(ctx: &Context, search: &str) -> Vec<String> {
  let data = ctx.data.read().await;
  let data = data.get::<CTAShared>().expect("no shared data");
  let gtfs = &data.gtfs;
  let matcher = SkimMatcherV2::default();

  let mut scored_suggestions: Vec<(String, i64)> = gtfs
    .gtfs_data
    .stops
    .iter()
    .filter(|stp| matches!(stp.0.clone().parse().unwrap_or(0), 40000..=49999))
    .map(|(_, stop)| {
      let name = stop.name.clone().unwrap_or_default();
      (
        name.clone(),
        matcher
          .fuzzy_match(name.as_str(), search)
          .unwrap_or_default(),
      )
    })
    .collect::<Vec<(String, i64)>>();

  scored_suggestions.retain(|(_, score)| *score != 0);
  scored_suggestions.sort_by_key(|(_, score)| *score);
  scored_suggestions.reverse();
  scored_suggestions[0..25.min(scored_suggestions.len())]
    .iter()
    .map(|(val, _)| val.clone())
    .collect()
}

async fn search_bus_stops(ctx: &Context, search: &str) -> Vec<String> {
  let data = ctx.data.read().await;
  let data = data.get::<CTAShared>().expect("no shared data");
  let gtfs = &data.gtfs;
  let matcher = SkimMatcherV2::default();

  let mut scored_suggestions: Vec<(String, i64)> = gtfs
    .gtfs_data
    .stops
    .iter()
    .filter(|stp| matches!(stp.0.clone().parse().unwrap_or(0), 0..=39999))
    .map(|(_, stop)| {
      let name = stop.name.clone().unwrap_or_default();
      (
        name.clone(),
        matcher
          .fuzzy_match(name.as_str(), search)
          .unwrap_or_default(),
      )
    })
    .collect::<Vec<(String, i64)>>();

  scored_suggestions.dedup_by(|a, b| a.0.eq(&b.0));
  scored_suggestions.retain(|(_, score)| *score != 0);
  scored_suggestions.sort_by_key(|(_, score)| *score);
  scored_suggestions.reverse();
  scored_suggestions[0..25.min(scored_suggestions.len())]
    .iter()
    .map(|(val, _)| val.clone())
    .collect()
}
// async fn stations(ctx: &Context, interaction: &CommandInteraction) -> CreateAutocompleteResponse {

//   // dbg!(interaction.data.options.first());
//   if let Some(CommandDataOption {
//     value: CommandDataOptionValue::Autocomplete {
//       kind: CommandOptionType::String,
//       value: current_search }, ..
//   }) = interaction.data.options.first() {
//     // dbg!(current_search);

//     suggestions.retain(|(_, score)| *score != 0);
//     suggestions.sort_by_key(|(val, score)| *score);
//     suggestions.reverse();
//     // dbg!(&suggestions);
//     CreateAutocompleteResponse::new().set_choices(suggestions[0..25.min(suggestions.len())].iter().map(|(val, score)| {
//       AutocompleteChoice::new(val.clone(), val.clone())
//     }).collect())
//   }
//   else {
//     CreateAutocompleteResponse::new()
//   }

// }

// async fn bus_stops(ctx: &Context, interaction: &CommandInteraction) -> CreateAutocompleteResponse {
//   let data = ctx.data.read().await;
//   let data = data.get::<CTAShared>().expect("no shared data");
//   let gtfs = &data.gtfs;
//   let matcher = SkimMatcherV2::default();
//   // dbg!(interaction.data.options.first());

//   if let Some(CommandDataOption {
//     value: CommandDataOptionValue::SubCommand(sub), ..
//   }) = interaction.data.options.first() {
//     if let Some(CommandDataOption {
//       name: option_name
//       value: CommandDataOptionValue::Autocomplete {
//         kind: CommandOptionType::String,
//         value: search
//       },
//       ..
//     })= sub.first() {
//       // dbg!(current_search);
//       let mut suggestions: Vec<(String, i64)> = gtfs.gtfs_data.stops.iter()
//         .filter(|stp|
//           matches!(stp.0.clone().parse().unwrap_or(0), 0..=30000))
//         .map(|(_, stop)| {
//           let name = stop.name.clone().unwrap_or_default();
//           (name.clone(), matcher.fuzzy_match(name.as_str(), &current_search).unwrap_or_default())
//         }).collect();
//     suggestions.retain(|(_, score)| *score != 0);
//     suggestions.sort_by_key(|(val, score)| *score);
//     suggestions.reverse();
//     // dbg!(&suggestions);
//     CreateAutocompleteResponse::new()
//       .set_choices(
//         suggestions[0..25.min(suggestions.len())]
//           .iter()
//           .map(|(val, score)| {
//             AutocompleteChoice::new(val.clone(), val.clone())
//           }).collect())
//     }
//     else {
//       CreateAutocompleteResponse::new()
//     }
//   }
//   else {
//     CreateAutocompleteResponse::new()
//   }
// }
// // pub fn arrivals_autocomplete(ctx: Context, interaction: &CommandInteraction) -> CreateAutocompleteResponse {
// //   println!("In Arrivals Autocomplete");

// // }
