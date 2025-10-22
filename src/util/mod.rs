use serde::{
  de::{self, Unexpected},
  Deserialize, Deserializer,
};

#[allow(clippy::cast_possible_truncation)]
pub fn minutes_until(input: chrono::DateTime<chrono_tz::Tz>) -> i32 {
  let now = chrono::Utc::now();
  let diff = input.to_utc() - now;
  diff.num_minutes() as i32
}
pub fn countdown(mins_until: i32) -> String {
  if mins_until < 2 {
    String::from("Due")
  } else if mins_until < 59 {
    return format!("{mins_until} min");
  } else if mins_until == 60 {
    return String::from("1 hr");
  } else {
    let hrs = mins_until % 60;
    let mins = mins_until - (60 * hrs);
    return format!("{hrs} hr {mins} min");
  }
}

/// Deserialize bool from String with custom value mapping
pub fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
  D: Deserializer<'de>,
{
  match String::deserialize(deserializer)?.as_ref() {
    "0" | "false" | "False" | "FALSE" => Ok(false),
    "true" | "1" | "True" | "TRUE" => Ok(true),
    other => Err(de::Error::invalid_value(
      Unexpected::Str(other),
      &"true or false",
    )),
  }
}
