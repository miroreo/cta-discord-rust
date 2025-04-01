use serde::*;
use serde_with::*;

use std::sync::OnceLock;
use reqwest::*;
static stations_url: &str = "https://data.cityofchicago.org/resource/8pix-ypme.json";

#[serde_as]
#[derive(Deserialize, Debug)]
struct Stop {
  #[serde_as(as = "DisplayFromStr")]
  stop_id: i32,

  // #[serde_as(as = "DisplayFromStr")]
  direction_id: Direction,
  stop_name: String,
  station_name: String,
  station_descriptive_name: String,

  #[serde_as(as = "DisplayFromStr")]
  map_id: i32,
  ada: bool,
  red: bool,
  blue: bool,
  g: bool,
  brn: bool,
  p: bool,
  pexp: bool,
  y: bool,
  pnk: bool,
  o: bool,
  location: Location
}

#[serde_as]
#[derive(Deserialize, Debug)]
struct Location {
  #[serde_as(as = "DisplayFromStr")]
  latitude: f32,
  #[serde_as(as = "DisplayFromStr")]
  longitude: f32,
}

#[derive(Deserialize, Debug)]
enum Direction {
  N,
  S,
  E,
  W,
}

pub struct CtaStations {
  stops: Vec<Stop>
}
impl CtaStations {
  pub async fn new() -> Self {
    let resp_text = get(stations_url)
    .await
      .expect("Could not load stations data")
      .text()
      .await.expect("Could not parse text of stations data");
    Self {
      stops: serde_json::from_str(&resp_text).expect("Could not parse station data.")
    }
  }
  pub async fn get_stop_name(&self, id: i32) -> Option<String> {
    self.stops.iter().find(|p| {
      p.map_id == id || p.stop_id == id
    }).map(|s| s.station_descriptive_name.clone())
  }
}

