use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde, serde_as};
use reqwest::get;
use std::{result::Result, str::FromStr};
use thiserror::Error;
use crate::util::bool_from_string;

#[derive(Deserialize,Debug)]
struct TopLevelResponse<I> {
  ctatt: I
}

#[serde_as]
#[derive(Deserialize,Debug)]
struct PositionTT {
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="tmst")]
  timestamp: chrono::NaiveDateTime,
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="errCd")]
  error_code: i32,
  #[serde(rename="errNm")]
  error_name: Option<String>,
  route: Vec<TTRoute>,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct TTRoute {
  #[serde(rename="@name")]
  pub name: LRouteCode,
  #[serde(rename="train")]
  pub trains: Vec<TTPosition>,
}

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
pub struct TTPosition {
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="rn")]
  pub run_number: i32,
  #[serde(rename="rt")]
  pub route: Option<LRouteCode>,
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="destSt")]
  pub destination_station: i32,
  #[serde(rename="destNm")]
  pub destination_name: String,
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="trDr")]
  pub train_direction: i8,
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="nextStaId")]
  pub next_station_id: i32,
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="nextStpId")]
  pub next_stop_id: i32,
  #[serde(rename="nextStaNm")]
  pub next_station_name: String,
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="prdt")]
  pub prediction_time: NaiveDateTime,
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="arrT")]
  pub arrival_time: NaiveDateTime,
  #[serde(deserialize_with = "bool_from_string")]
  #[serde(rename="isApp")]
  pub is_approaching: bool,
  #[serde(deserialize_with = "bool_from_string")]
  #[serde(rename="isDly")]
  pub is_delayed: bool,
  #[serde_as(as = "DisplayFromStr")]
  pub lat: f32,
  #[serde_as(as = "DisplayFromStr")]
  pub lon: f32,
  #[serde_as(as = "DisplayFromStr")]
  pub heading: i32
}

#[serde_as]
#[derive(Deserialize, Debug)]
struct ArrivalsTT {
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="tmst")]
  timestamp: chrono::NaiveDateTime,
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="errCd")]
  error_code: i32,
  #[serde(rename="errNm")]
  error_name: Option<String>,
  #[serde(rename="eta")]
  arrivals: Vec<TTArrival>,
}

#[allow(clippy::struct_excessive_bools)]
#[serde_as]
#[derive(Deserialize, Debug)]
pub struct TTArrival {
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="rn")]
  pub run_number: i32,
  #[serde(rename="rt")]
  pub route: LRouteCode,
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="destSt")]
  pub destination_station: i32,
  #[serde(rename="destNm")]
  pub destination_name: String,
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="trDr")]
  pub train_direction: i8,
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="prdt")]
  pub prediction_time: NaiveDateTime,
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="arrT")]
  pub arrival_time: NaiveDateTime,
  #[serde(deserialize_with = "bool_from_string")]
  #[serde(rename="isApp")]
  pub is_approaching: bool,
  #[serde(deserialize_with = "bool_from_string")]
  #[serde(rename="isSch")]
  pub is_scheduled: bool,
  #[serde(deserialize_with = "bool_from_string")]
  #[serde(rename="isDly")]
  pub is_delayed: bool,
  #[serde(deserialize_with = "bool_from_string")]
  #[serde(rename="isFlt")]
  pub is_faulted: bool,
  #[serde(rename="lat")]
  pub latitude: Option<String>,
  #[serde(rename="lon")]
  pub longitude: Option<String>,
  pub heading: Option<String>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum LRouteCode {
  Red,
  P,
  Y,
  Blue,
  Pink,
  G,
  Org,
  Brn
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum LRouteName {
  #[serde(rename="Red Line")]
  Red,
  #[serde(rename="Purple Line")]
  P,
  #[serde(rename="Yellow Line")]
  Y,
  #[serde(rename="Blue Line")]
  Blue,
  #[serde(rename="Pink Line")]
  Pink,
  #[serde(rename="Green Line")]
  G,
  #[serde(rename="Orange Line")]
  Org,
  #[serde(rename="Brown Line")]
  Brn
}

impl From<LRouteCode> for LRouteName {
  fn from(value: LRouteCode) -> Self {
    match value {
      LRouteCode::Red => LRouteName::Red,
      LRouteCode::P => LRouteName::P,
      LRouteCode::Y => LRouteName::Y,
      LRouteCode::Blue => LRouteName::Blue,
      LRouteCode::Pink => LRouteName::Pink,
      LRouteCode::G => LRouteName::G,
      LRouteCode::Org => LRouteName::Org,
      LRouteCode::Brn => LRouteName::Brn
    }
  }
}
impl From<LRouteName> for LRouteCode {
  fn from(value: LRouteName) -> Self {
    match value {
      LRouteName::Red => LRouteCode::Red,
      LRouteName::P => LRouteCode::P,
      LRouteName::Y => LRouteCode::Y,
      LRouteName::Blue => LRouteCode::Blue,
      LRouteName::Pink => LRouteCode::Pink,
      LRouteName::G => LRouteCode::G,
      LRouteName::Org => LRouteCode::Org,
      LRouteName::Brn => LRouteCode::Brn
    }
  }
}

#[serde_as]
#[derive(Deserialize, Debug)]
struct FollowTrainTT {
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="tmst")]
  timestamp: NaiveDateTime,
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="errCd")]
  error_code: i32,
  #[serde(rename="errNm")]
  error_name: Option<String>,
  position: Position,
  eta: Vec<TTFollowEta>
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Position {
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="lat")]
  pub latitude: f32,
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="lon")]
  pub longitude: f32,
  #[serde_as(as = "DisplayFromStr")]
  pub heading: i32,
}

#[allow(clippy::struct_excessive_bools)]
#[serde_as]
#[derive(Deserialize, Debug)]
pub struct TTFollowEta {
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="staId")]
  pub station_id: i32,
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="stpId")]
  pub stop_id: i32,
  #[serde(rename="staNm")]
  pub station_name: String,
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="rn")]
  pub run_number: i32,
  #[serde(rename="rt")]
  pub route: LRouteName,
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="destSt")]
  pub destination_station: i32,
  #[serde(rename="destNm")]
  pub destination_name: String,
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="trDr")]
  pub train_direction: TrainDirection,
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="prdt")]
  pub prediction_time: NaiveDateTime,
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="arrT")]
  pub arrival_time: NaiveDateTime,
  #[serde(deserialize_with = "bool_from_string")]
  #[serde(rename="isApp")]
  pub is_approaching: bool,
  #[serde(deserialize_with = "bool_from_string")]
  #[serde(rename="isSch")]
  pub is_scheduled: bool,
  #[serde(deserialize_with = "bool_from_string")]
  #[serde(rename="isDly")]
  pub is_delayed: bool,
  #[serde(deserialize_with = "bool_from_string")]
  #[serde(rename="isFlt")]
  pub is_faulted: bool,
}

#[derive(Debug, Clone)]
pub enum TrainDirection {
  Northbound = 1,
  Southbound = 5,
}
impl FromStr for TrainDirection {
  type Err = TrainTrackerError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "1" => Ok(TrainDirection::Northbound),
      "5" => Ok(TrainDirection::Southbound),
      _ => Err(TrainTrackerError::DataError)
    }
  }
}
#[derive(Error, Debug)]
pub enum TrainTrackerError {
  #[error("Failed to fetch data from TrainTracker API")]
  RequestError(#[from] reqwest::Error),
  #[error("Failed to parse JSON data returned from TrainTracker API")]
  ParseError(#[from] serde_json::Error),
  #[error("TrainTracker API provided invalid data")]
  DataError,
}

#[derive(Serialize, Debug)]
pub struct ArrivalsParameters {
  #[serde(flatten)]
  pub id: MapOrStopID,
  pub max: Option<i32>,
  pub rt: Option<String>,
}
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum MapOrStopID {
  MapID { mapid: i32 },
  StopID { stpid: i32 }
}
pub struct TrainTracker {
  token: String,
}
impl TrainTracker {
  const BASE_URL: &str = "https://lapi.transitchicago.com/api/1.0/";

  pub fn new(token: &str) -> Self {
    Self {
      token: token.to_string()
    }
  }
  pub async fn follow_train(&self, train_number: i32) -> Result<Vec<TTFollowEta>, TrainTrackerError> {
    let resp_text = get(format!("{}ttfollow.aspx?runnumber={train_number}&key={}&outputType=JSON", Self::BASE_URL, self.token))
      .await?
      .text()
      .await?;
    // let topLevel: TopLevelResponse<FollowTrainTT> = ;
    Ok(serde_json::from_str::<TopLevelResponse<FollowTrainTT>>(&resp_text)?.ctatt.eta)
  }
  pub async fn arrivals(&self, options: ArrivalsParameters) -> Result<Vec<TTArrival>, TrainTrackerError> {
    let mut params: String = match options.id {
      MapOrStopID::MapID { mapid } => format!("mapid={mapid}"),
      MapOrStopID::StopID { stpid } => format!("stpid={stpid}"),
    };
    if options.max.is_some() {
      params = format!("{}&max={}", params, options.max.unwrap());
    }
    if options.rt.is_some() {
      params = format!("{}&rt={}", params, options.rt.unwrap().as_str());
    }
    let resp_text = get(format!("{}ttarrivals.aspx?{}&key={}&outputType=JSON", Self::BASE_URL, params, self.token))
      .await?
      .text()
      .await?;
    // println!("{resp}", resp_text);
    Ok(serde_json::from_str::<TopLevelResponse<ArrivalsTT>>(&resp_text)?.ctatt.arrivals)
  }

  pub async fn positions(&self, rt: Vec<LRouteCode>) -> Result<Vec<TTRoute>, TrainTrackerError> {
    let routes: String = rt.iter()
      .filter_map(|r| serde_json::ser::to_string(r).ok())
      .fold(String::new(), |prev, r| 
        format!("{prev}{r},"));
    let resp_text = get(format!("{}ttpositions.aspx?rt={routes}&key={}&outputType=JSON", Self::BASE_URL, self.token))
      .await?
      .text()
      .await?;
    // let topLevel: TopLevelResponse<FollowTrainTT> = ;
    Ok(serde_json::from_str::<TopLevelResponse<PositionTT>>(&resp_text)?
      .ctatt
      .route)
  }
}