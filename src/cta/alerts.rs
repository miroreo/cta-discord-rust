use std::{ops::Deref, str::FromStr};

use chrono::{Date, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{DisplayFromStr, serde, serde_as};
use thiserror::Error;
use crate::util::bool_from_string;

const ALERTS_URL: &str = "https://www.transitchicago.com/api/1.0/alerts.aspx?outputType=JSON";
#[derive(Error, Debug)]
pub enum AlertsError {
  #[error("Failed to fetch data from Alerts API")]
  RequestError(#[from] reqwest::Error),
  #[error("Failed to parse JSON data returned from Alerts API")]
  ParseError(#[from] serde_json::Error),
  #[error("Alerts API provided invalid data")]
  DataError,
  #[error("Database provided invalid data")]
  DataBaseError,
  #[error("There are no active alerts.")]
  NoAlerts
}

#[serde_as]
#[derive(Deserialize, Debug)]
struct AlertsAPIResponse {
  #[serde(rename="CTAAlerts")]
  alerts: CTAAlerts,
}

#[serde_as]
#[derive(Deserialize, Debug)]
struct CTAAlerts {
  #[serde(rename="TimeStamp")]
  timestamp: String,
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="ErrorCode")]
  error_code: i32,
  #[serde(rename="ErrorMessage")]
  error_message: Option<String>,
  #[serde(rename="Alert")]
  alerts: Option<Vec<Alert>>,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Alert {
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="AlertId")]
  pub id: i32,
  #[serde(rename="Headline")]
  pub headline: String,
  #[serde(rename="ShortDescription")]
  pub short_description: String,
  #[serde(rename="FullDescription")]
  // #[serde(flatten)]
  pub full_description: CDATA<String>,
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="SeverityScore")]
  pub severity_score: i32,
  #[serde(rename="SeverityColor")]
  pub severity_color: String,
  #[serde(rename="SeverityCSS")]
  pub severity_css: String,
  #[serde(rename="Impact")]
  pub impact: String,
  // #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="EventStart")]
  pub event_start: DateOrDateTime,
  #[serde_as(as = "Option<DisplayFromStr>")]
  #[serde(rename="EventEnd")]
  pub event_end: Option<NaiveDateTime>,
  #[serde(rename="TBD")]
  #[serde(deserialize_with = "bool_from_string")]
  pub tbd: bool,
  #[serde(rename="MajorAlert")]
  #[serde(deserialize_with = "bool_from_string")]
  pub major_alert: bool,
  #[serde(rename="AlertURL")]
  // #[serde(flatten)]
  pub alert_url: CDATA<String>,
  #[serde(rename="ImpactedService")]
  // #[serde(flatten)]
  pub impacted_services: ImpactedService,
}

#[serde(untagged)]
#[derive(Deserialize, Serialize, Debug)]
pub enum DateOrDateTime {
  DateTime(NaiveDateTime),
  Date(NaiveDate)
}
impl std::fmt::Display for DateOrDateTime {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      DateOrDateTime::DateTime(naive_date_time) => {
        write!(f, "DateTime {}", naive_date_time.to_string())
      },
      DateOrDateTime::Date(naive_date) => {
        write!(f, "Date {}", naive_date.to_string())
      },
    }
  }
}
#[derive(Deserialize, Serialize, Debug)]
pub struct CDATA<I> {
  #[serde(rename="#cdata-section")]
  inner: I
}

impl <I> Deref for CDATA<I> {
  type Target = I;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

#[derive(Debug)]
pub struct ImpactedService {
  // #[serde(rename="Service")]
  pub impacted_services: Vec<Service>
}
impl<'de> Deserialize<'de> for ImpactedService {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> 
  {
    let value: Value = Deserialize::deserialize(deserializer)?;

    if let Some(array) = value["Service"].as_array() {
      let children = serde_json::from_value::<Vec<Service>>(Value::Array(array.clone()))
        .map_err(serde::de::Error::custom)?;
      // dbg!(&children);
      Ok(ImpactedService{
        impacted_services: children
      })
    } else if value.is_object() {
      let single = serde_json::from_value::<Service>(value["Service"].clone())
        .map_err(serde::de::Error::custom)?;
      Ok(ImpactedService{
        impacted_services: vec![single]
      })
    } else {
      Err(serde::de::Error::custom("Unexpected type"))
    }
  }
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct Service {
  #[serde_as(as = "DisplayFromStr")]
  #[serde(rename="ServiceType")]
  pub stype: ServiceType,
  #[serde(rename="ServiceTypeDescription")]
  pub stype_description: String,
  #[serde(rename="ServiceId")]
  pub id: String,
  #[serde(rename="ServiceName")]
  pub name: String,
  #[serde(rename="ServiceBackColor")]
  pub background_color: String,
  #[serde(rename="ServiceTextColor")]
  pub text_color: String,
  #[serde(rename="ServiceURL")]
  // #[serde(flatten)]
  pub url: CDATA<String>,
}
#[derive(Deserialize, Serialize, Debug)]
pub enum ServiceType {
  SystemWide,
  TrainRoute,
  BusRoute,
  TrainStation,
}
impl FromStr for ServiceType {
  type Err = AlertsError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "X" => Ok(Self::SystemWide),
      "R" => Ok(Self::TrainRoute),
      "B" => Ok(Self::BusRoute),
      "T" => Ok(Self::TrainStation),
      _ => Err(AlertsError::DataError)
    }
  }
}
impl std::fmt::Display for ServiceType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = match self {
      ServiceType::SystemWide => "X".to_string(),
      ServiceType::TrainRoute => "R".to_string(),
      ServiceType::BusRoute => "B".to_string(),
      ServiceType::TrainStation => "T".to_string(),
    };
    write!(f, "{}", s)
  }
}
// impl ToString for ServiceType {
//   fn to_string(&self) -> String {
//     match self {
//       ServiceType::SystemWide => "X".to_string(),
//       ServiceType::TrainRoute => "R".to_string(),
//       ServiceType::BusRoute => "B".to_string(),
//       ServiceType::TrainStation => "T".to_string(),
//     }
//   }
// }

fn yes() -> bool { true }
#[derive(Serialize, Debug, Default)]
pub struct AlertsOptions {
  #[serde(rename="activeonly")]
  #[serde(default)]
  pub active_only: Option<bool>,
  #[serde(default="yes")]
  pub accessibility: Option<bool>,
  #[serde(default="yes")]
  pub planned: Option<bool>,
  #[serde(rename="routeid")]
  pub route_ids: Vec<String>,
  #[serde(rename="bystartdate")]
  pub by_start_date: Option<NaiveDate>,
  #[serde(rename="recentdays")]
  pub recent_days: Option<i32>,
}
pub async fn get_active_alerts(options: AlertsOptions) -> Result<Vec<Alert>, AlertsError> {
  let query_string = serde_structuredqs::to_string(&options).expect("Could not parse options for get_active_alerts");
  let response_text = reqwest::get(format!("{ALERTS_URL}&{query_string}"))
    .await?
    .text()
    .await?;
  // println!("{}", response_text);
  let response: AlertsAPIResponse = serde_json::from_str::<AlertsAPIResponse>(&response_text)?;
  match response.alerts.alerts {
    Some(data) => Ok(data),
    None => match response.alerts.error_code {
        50 | 25 => Ok(Vec::<Alert>::new()),
        _ => Err(AlertsError::DataError),
    },
  }
}