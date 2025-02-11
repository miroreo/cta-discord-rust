use chrono::NaiveDateTime;
use serde::*;
use serde_with::*;
use reqwest::*;
use std::result::Result;
use thiserror::Error;

#[derive(Deserialize,Debug)]
struct TopLevelResponse<I> {
  ctatt: I
}

#[serde_as]
#[derive(Deserialize,Debug)]
struct PositionTT {
  #[serde_as(as = "DisplayFromStr")]
  tmst: chrono::NaiveDateTime,
  #[serde_as(as = "DisplayFromStr")]
  errCd: i32,
  errNm: Option<String>,
  route: Vec<TTRoute>,
}

#[serde_as]
#[derive(Deserialize, Debug)]
struct TTRoute {
  #[serde(rename="@name")]
  name: LRouteCode,
  train: Vec<TTPosition>,
}

#[serde_as]
#[derive(Deserialize, Debug)]
struct TTPosition {

  #[serde_as(as = "DisplayFromStr")]
  rn: i32,
  #[serde_as(as = "DisplayFromStr")]
  destSt: i32,
  destNm: String,
  #[serde_as(as = "DisplayFromStr")]
  trDr: i8,
  #[serde_as(as = "DisplayFromStr")]
  nextStaId: i32,
  #[serde_as(as = "DisplayFromStr")]
  nextStpId: i32,
  nextStaNm: String,
  #[serde_as(as = "DisplayFromStr")]
  prdt: NaiveDateTime,
  #[serde_as(as = "DisplayFromStr")]
  arrT: NaiveDateTime,
  #[serde_as(as = "DisplayFromStr")]
  isApp: i8,
  #[serde_as(as = "DisplayFromStr")]
  isDly: i8,
  #[serde_as(as = "DisplayFromStr")]
  lat: f32,
  #[serde_as(as = "DisplayFromStr")]
  lon: f32,
  #[serde_as(as = "DisplayFromStr")]
  heading: i32
}

#[serde_as]
#[derive(Deserialize, Debug)]
struct ArrivalsTT {
  #[serde_as(as = "DisplayFromStr")]
  tmst: chrono::NaiveDateTime,
  #[serde_as(as = "DisplayFromStr")]
  errCd: i32,
  errNm: Option<String>,
  eta: Vec<TTArrival>,
}

#[serde_as]
#[derive(Deserialize, Debug)]
struct TTArrival {
  #[serde_as(as = "DisplayFromStr")]
  rn: i32,
  rt: LRouteCode,
  #[serde_as(as = "DisplayFromStr")]
  destSt: i32,
  destNm: String,
  #[serde_as(as = "DisplayFromStr")]
  trDr: i8,
  #[serde_as(as = "DisplayFromStr")]
  prdt: NaiveDateTime,
  #[serde_as(as = "DisplayFromStr")]
  arrT: NaiveDateTime,
  #[serde_as(as = "DisplayFromStr")]
  isApp: i8,
  #[serde_as(as = "DisplayFromStr")]
  isSch: i8,
  #[serde_as(as = "DisplayFromStr")]
  isDly: i8,
  #[serde_as(as = "DisplayFromStr")]
  isFlt: i8,
  #[serde_as(as = "DisplayFromStr")]
  lat: f32,
  #[serde_as(as = "DisplayFromStr")]
  lon: f32,
  #[serde_as(as = "DisplayFromStr")]
  heading: i32
}

#[derive(Deserialize, Debug)]
enum LRouteCode {
  Red,
  P,
  Y,
  Blue,
  Pink,
  G,
  Org,
  Brn
}

#[derive(Deserialize, Debug)]
enum LRouteName {
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
  tmst: NaiveDateTime,
  #[serde_as(as = "DisplayFromStr")]
  errCd: i32,
  errNm: Option<String>,
  position: Position,
  eta: Vec<TTFollowEta>
}

#[serde_as]
#[derive(Deserialize, Debug)]
struct Position {
  #[serde_as(as = "DisplayFromStr")]
  lat: f32,
  #[serde_as(as = "DisplayFromStr")]
  lon: f32,
  #[serde_as(as = "DisplayFromStr")]
  heading: i32,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct TTFollowEta {
  #[serde_as(as = "DisplayFromStr")]
  pub staId: i32,
  #[serde_as(as = "DisplayFromStr")]
  pub stpId: i32,
  pub staNm: String,
  #[serde_as(as = "DisplayFromStr")]
  pub rn: i32,
  pub rt: LRouteName,
  #[serde_as(as = "DisplayFromStr")]
  pub destSt: i32,
  pub destNm: String,
  #[serde_as(as = "DisplayFromStr")]
  pub trDr: i8,
  #[serde_as(as = "DisplayFromStr")]
  pub prdt: NaiveDateTime,
  #[serde_as(as = "DisplayFromStr")]
  pub arrT: NaiveDateTime,
  #[serde_as(as = "DisplayFromStr")]
  pub isApp: i8,
  #[serde_as(as = "DisplayFromStr")]
  pub isSch: i8,
  #[serde_as(as = "DisplayFromStr")]
  pub isDly: i8,
  #[serde_as(as = "DisplayFromStr")]
  pub isFlt: i8,
}

#[derive(Error, Debug)]
pub enum TrainTrackerError {
  #[error("Failed to fetch data from TrainTracker API")]
  RequestError(#[from] reqwest::Error),
  #[error("Failed to parse JSON data returned from TrainTracker API")]
  ParseError(#[from] serde_json::Error),
  // #[error("TrainTracker threw an error.")]
}
pub struct TrainTracker {
  token: String,
}
impl TrainTracker {
  const base_url: &str = "https://lapi.transitchicago.com/api/1.0/";

  pub fn new(token: &str) -> Self {
    Self {
      token: token.to_string()
    }
  }
  pub async fn train_next_stations(&self, train_number: i32) -> Result<Vec<TTFollowEta>, TrainTrackerError> {
    let resp_text = get(format!("{}ttfollow.aspx?runnumber={train_number}&key={}&outputType=JSON", Self::base_url, self.token))
      .await?
      .text()
      .await?;
    println!("{}", resp_text);
    // let topLevel: TopLevelResponse<FollowTrainTT> = ;
    Ok(serde_json::from_str::<TopLevelResponse<FollowTrainTT>>(&resp_text)?.ctatt.eta)
  }
}