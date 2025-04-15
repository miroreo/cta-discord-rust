use chrono;


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
