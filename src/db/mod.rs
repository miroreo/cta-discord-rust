use sqlx::{Connection, FromRow, PgConnection};

#[derive(sqlx::FromRow, Debug)]
pub struct Alert {
  pub alert_id: i64,
  pub headline: String,
  pub short_description: String,
  pub guid: String,
  pub published_to: i64,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Guild {
  pub guild_id: i64,
  pub guild_name: Option<String>,
  pub has_alerts: Option<bool>,
  pub alert_channel: Option<i64>,
  pub accessibility_alerts: Option<bool>,
  pub planned_alerts: Option<bool>,
  pub route_ids: Option<Vec<String>>,
  pub ephemeral_arrivals: Option<bool>
}
#[derive(sqlx::FromRow, Debug)]
pub struct KeyValue {
  key: String,
  value: Option<String>,
}

pub async fn get_guilds(db_url: String) -> Result<Vec<Guild>, sqlx::Error> {
  let mut db_connection = PgConnection::connect(db_url.as_str()).await.expect("Couldn't connect to database.");
  sqlx::query_as!(Guild, "SELECT * FROM guilds;")
    .fetch_all(&mut db_connection)
    .await
}

pub async fn get_value(db_url: String, key: &str) {
  let mut db_connection = PgConnection::connect(db_url.as_str()).await.expect("Couldn't connect to database.");
  let res = sqlx::query_as!(KeyValue, "SELECT * FROM kv_store WHERE key = $1;", key)
    .fetch_one(&mut db_connection)
    .await;

  dbg!(res.unwrap());
}