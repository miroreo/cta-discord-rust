use chrono;
use serde::{de::{self, Unexpected}, Deserialize, Deserializer};


pub fn minutes_until(input: chrono::DateTime<chrono_tz::Tz>) -> i32 {
  let now = chrono::Utc::now();
  let diff = input.to_utc() - now;
  return diff.num_minutes() as i32;
}
pub fn countdown(mins_until: i32) -> String {
  if mins_until < 2 {
    return String::from("Due");
  } else if mins_until < 59 {
    return format!("{} min", mins_until);
  } else if mins_until == 60 {
    return String::from("1 hr");
  } else {
    let hrs = mins_until % 60;
    let mins = mins_until - (60 * hrs);
    return format!("{} hr {} min", hrs, mins);
  }
}

/// Deserialize bool from String with custom value mapping
pub fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
  D: Deserializer<'de>,
{
  match String::deserialize(deserializer)?.as_ref() {
    "1" => Ok(true),
    "0" => Ok(false),
    "true" => Ok(true),
    "false" => Ok(false),
    "True" => Ok(true),
    "False" => Ok(false),
    "TRUE" => Ok(true),
    "FALSE" => Ok(false),
    other => Err(de::Error::invalid_value(
      Unexpected::Str(other),
      &"OK or nOK",
    )),
  }
}