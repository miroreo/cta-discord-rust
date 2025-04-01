use chrono::prelude::*;
use serde::*;
use serde_with::*;
use reqwest::*;
use std::{collections::BTreeMap, num::ParseIntError, result::Result};
use thiserror::Error;

#[derive(Deserialize, Debug)]
struct BTResponse<I> {
  #[serde(rename = "bustime-response")]
  busTimeResponse: I
}

#[derive(Deserialize, Debug)]
struct GetTimeResponse {
  tm: String
}

#[derive(Deserialize, Debug)]
struct GetVehiclesResponse {
  vehicle: Vec<Vehicle>
}

#[derive(Deserialize, Debug)]
struct GetPredictionsResponse {
  prd: Vec<Prediction>
}

#[derive(Debug, PartialEq, Eq)]
pub enum Garage {
  Garage103rd,
  Kedzie,
  ForestGlen,
  NorthPark,
  Garage74th,
  Garage77th,
  Chicago,
  Unknown
}

impl From<Garage> for String {
  fn from(garage: Garage) -> String {
    match garage {
      Garage::Chicago => "Chicago Garage",
      Garage::Garage103rd => "103rd Garage",
      Garage::Kedzie => "Kedzie Garage",
      Garage::ForestGlen => "Forest Glen Garage",
      Garage::NorthPark => "North Park Garage",
      Garage::Garage74th => "74th Garage",
      Garage::Garage77th => "77th Garage",
      Garage::Unknown => "Unknown Garage",
      
    }.to_string()
  }
}
#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Vehicle {
  #[serde_as(as = "DisplayFromStr")]
  pub lat: f32,
  #[serde_as(as = "DisplayFromStr")]
  pub lon: f32,
  #[serde_as(as = "DisplayFromStr")]
  pub hdg: i32,
  pub tmstmp: String,
  pub vid: String,
  pub pid: i32,
  pub rt: String,
  pub des: String,
  pub dly: Option<bool>,
  pub zone: String,
  pub pdist: Option<i32>,
  /// The TA Block ID field is rather odd. 
  /// It's in the form of "Route -GXX" where G corresponds to the bus's garage ID.
  pub tablockid: String,
}

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
pub struct Prediction {
  pub tmstmp: String,
  pub typ: PredictionType,
  pub stpnm: String,
  #[serde_as(as = "DisplayFromStr")]
  pub stpid: i32,
  pub vid: String,
  pub dstp: i32,
  pub rt: String,
  pub rtdir: String,
  pub des: String,
  pub prdtm: String,
  pub dly: Option<bool>,
  pub tablockid: String,
  pub tatripid: String,
  pub prdctdn: String,
  pub zone: String
}
#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub enum PredictionType {
  A,
  D
}
#[derive(Error, Debug)]
pub enum BusTrackerError {
  #[error("Failed to fetch data from BusTracker API")]
  RequestError(#[from] reqwest::Error),
  #[error("Failed to parse JSON data returned from BusTracker API")]
  ParseError(#[from] serde_json::Error),
  #[error("Failed to parse BusTracker server time.")]
  TimeError(#[from] chrono::ParseError),
  #[error("Time arithmetic went out of range.")]
  TimeOutOfRange,
  #[error("Integer Parsing Error")]
  ParseIntError(#[from] std::num::ParseIntError)
}
#[derive(Serialize, Debug)]
pub struct VehiclesParameters {
  #[serde(flatten)]
  pub search: VidOrRt,
}
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum VidOrRt {
  Vid { Vid: Vec<String> },
  Rt { Rt: Vec<String> }
}

#[derive(Serialize, Debug)]
pub struct PredictionsParameters {
  #[serde(flatten)]
  pub search: StpidOrVid
}
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum StpidOrVid {
  StpId { stpid: Vec<String>, rt: Option<Vec<String>> },
  Vid { vid: Vec<String> }
}

#[derive(Clone)]
pub struct BusTracker {
  token: String,
}
impl BusTracker {
  const BASE_URL: &str = "http://www.ctabustracker.com/bustime/api/v2/";
  pub fn new(token: &str) -> Self {
    Self {
      token: token.to_string()
    }
  }
  pub async fn get_time(&self) -> Result<DateTime<FixedOffset>, BusTrackerError> {
    let resp_text = get(format!("{}gettime?key={}&format=json", Self::BASE_URL, self.token))
      .await?
      .text()
      .await?;
    
    Ok(Self::parse_bustime(&serde_json::from_str::<BTResponse<GetTimeResponse>>(resp_text.as_str())?.busTimeResponse.tm)?)
  }
  
  pub async fn get_vehicles(&self, options: VehiclesParameters) -> Result<Vec<Vehicle>, BusTrackerError> {
    let params: String = match options.search {
      VidOrRt::Vid { Vid } => format!("vid={}", Vid.join(",")),
      VidOrRt::Rt { Rt } => format!("rt={}", Rt.join(",")),
    };
    let resp_text = get(format!("{}getvehicles?key={}&format=json&tmres=s&{}", Self::BASE_URL, self.token, params))
      .await?
      .text()
      .await?;
    Ok(serde_json::from_str::<BTResponse<GetVehiclesResponse>>(resp_text.as_str()).inspect_err(|e| println!("{}\nError: {e}", resp_text.as_str()))?.busTimeResponse.vehicle)
  }

  pub async fn get_predictions(&self, options: PredictionsParameters) -> Result<Vec<Prediction>, BusTrackerError> {
    let params: String = match options.search {
      StpidOrVid::StpId { stpid, rt } => {
        match rt {
          Some(rts) => format!("stpid={}&rt={}", stpid.join(","), rts.join(",")),
          None => format!("stpid={}", stpid.join(","))
        }
      },
      StpidOrVid::Vid { vid } => format!("vid={}", vid.join(","))
    };
    let resp_text = get(format!("{}getpredictions?key={}&format=json&{}", Self::BASE_URL, self.token, params))
      .await?
      .text()
      .await?;
    Ok(serde_json::from_str::<BTResponse<GetPredictionsResponse>>(resp_text.as_str()).inspect_err(|e| println!("{}\nError: {e}", resp_text.as_str()))?.busTimeResponse.prd)
  }

  fn parse_bustime(timestamp: &str) -> Result<DateTime<FixedOffset>, BusTrackerError> {
    let tz = chrono::FixedOffset::west_opt(6*3600).unwrap();
    NaiveDateTime::parse_from_str(timestamp, "%Y%m%d %H:%M:%S")?
      .checked_sub_offset(tz).ok_or(BusTrackerError::TimeOutOfRange)?
      .and_local_timezone(tz).latest().ok_or(BusTrackerError::TimeOutOfRange)
  }
  
  /// This function takes in the TA Block ID from the CTA Bus Tracker get_vehicles route and returns the garage which it corresponds to.
  pub fn tablockid_to_garage(tablockid: &str) -> Garage {
    let id = tablockid.split("-").nth(1).unwrap_or("000").chars().nth(0).unwrap_or('0');
    
    match id.to_digit(10) {
      Some(1) => Garage::Garage103rd,
      Some(2) => Garage::Kedzie,
      Some(4) => Garage::ForestGlen,
      Some(5) => Garage::NorthPark,
      Some(6) => Garage::Garage74th,
      Some(7) => Garage::Garage77th,
      Some(8) => Garage::Chicago,
      Some(_) => {
        println!("Unknown Garage: {id}");
        Garage::Unknown
      },
      None => Garage::Unknown
    }
  }
  // pub async fn get_predictions(&self, options: PredictionParameters) -> Result
}
#[test]
fn test_tablockid_to_garage() {
  assert_eq!(BusTracker::tablockid_to_garage("95 -109"), Garage::Garage103rd);
  assert_eq!(BusTracker::tablockid_to_garage("52A-754"), Garage::Garage77th);
  assert_eq!(BusTracker::tablockid_to_garage("119 -154"), Garage::Garage103rd);
  assert_eq!(BusTracker::tablockid_to_garage("SS -851"), Garage::Chicago);
  assert_eq!(BusTracker::tablockid_to_garage("82 -207"), Garage::Kedzie);
  assert_eq!(BusTracker::tablockid_to_garage("63 -613"), Garage::Garage74th);
  assert_eq!(BusTracker::tablockid_to_garage("49 -559"), Garage::NorthPark);
  assert_eq!(BusTracker::tablockid_to_garage("79 -712"), Garage::Garage77th);
}