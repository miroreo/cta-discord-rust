use serenity::all::{AutocompleteChoice, CommandDataOption, CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateAutocompleteResponse, CreateInteractionResponse, Interaction, ResolvedOption, ResolvedValue};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

use crate::CTAShared;

pub async fn handle_autocomplete(ctx: &Context, interaction: &CommandInteraction) -> CreateInteractionResponse {
  // dbg!(interaction.data.name.as_str());
    if interaction.data.name.as_str() == "arrivals" {
      // dbg!(interaction);
      CreateInteractionResponse::Autocomplete(stations(ctx, interaction).await)

      // println!("Arrivals Autocomplete! {}", );
      // CreateInteractionResponse::Autocomplete(CreateAutocompleteResponse::new()
      //   .add_string_choice("Sox-35th", "Sox-35th")
      //   .add_string_choice("Sox-36th", "Sox-36th")
      //   .add_string_choice("Sox-37th", "Sox-37th"))
      // interaction.create_response(ctx.http.clone(), CreateInteractionResponse::Autocomplete(arrivals_autocomplete(ctx, interaction).clone())).await;
      
    // } else if interaction.data.name.as_str() == "bus" {
    //   CreateInteractionResponse::Autocomplete(())
    } else {
      println!("Unknown Autocomplete Command Name: {}", interaction.data.name.as_str());
      CreateInteractionResponse::Autocomplete(CreateAutocompleteResponse::new()
        .add_string_choice("ASox-35th", "Sox-35th")
        .add_string_choice("ASox-36th", "Sox-36th")
        .add_string_choice("ASox-37th", "Sox-37th"))
      // interaction.create_response(ctx.http, CreateInteractionResponse::Autocomplete(CreateAutocompleteResponse::new())).await;
    }
}
async fn stations(ctx: &Context, interaction: &CommandInteraction) -> CreateAutocompleteResponse {
  let data = ctx.data.read().await;
  let data = data.get::<CTAShared>().expect("no shared data");
  let gtfs = &data.gtfs;
  let matcher = SkimMatcherV2::default();
  // dbg!(interaction.data.options.first());
  if let Some(CommandDataOption {
    value: CommandDataOptionValue::Autocomplete { 
      kind: CommandOptionType::String,
      value: current_search }, ..
  }) = interaction.data.options.first() {
    // dbg!(current_search);
    let mut suggestions: Vec<(String, i64)> = gtfs.gtfs_data.stops.iter().filter(|stp| matches!(stp.0.clone().parse().unwrap_or(0), 40000..=49999)).map(|(_, stop)| {
      let name = stop.name.clone().unwrap_or_default();
      (name.clone(), matcher.fuzzy_match(name.as_str(), &current_search).unwrap_or_default())
    }).collect();
    suggestions.retain(|(_, score)| *score != 0);
    suggestions.sort_by_key(|(val, score)| *score);
    suggestions.reverse();
    // dbg!(&suggestions);
    CreateAutocompleteResponse::new().set_choices(suggestions[0..25.min(suggestions.len())].iter().map(|(val, score)| {
      AutocompleteChoice::new(val.clone(), val.clone())
    }).collect())
  }
  else {
    CreateAutocompleteResponse::new()
  }
  
}
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
//       name: String::from("stop_name"),
//       value: CommandDataOptionValue::Autocomplete {
//         kind: CommandOptionType::String,
//         value: search
//       },
//       ..
//     }) = sub.first() {
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
// pub fn arrivals_autocomplete(ctx: Context, interaction: &CommandInteraction) -> CreateAutocompleteResponse {
//   println!("In Arrivals Autocomplete");
  
// }