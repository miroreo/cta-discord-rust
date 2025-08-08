use sqlx::{Connection, FromRow, PgConnection};

#[derive(sqlx::FromRow, Debug)]
pub struct DBAlert {
  pub alert_id: i32,
  pub headline: Option<String>,
  pub short_description: Option<String>,
  pub guid: Option<String>,
  pub published_to: Option<i32>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct DBGuild {
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
pub struct DBKeyValue {
  key: String,
  value: Option<String>,
}

pub async fn get_guilds(db_url: String) -> Result<Vec<DBGuild>, sqlx::Error> {
  let mut db_connection = PgConnection::connect(db_url.as_str()).await.expect("Couldn't connect to database.");
  sqlx::query_as!(DBGuild, "SELECT * FROM guilds;")
    .fetch_all(&mut db_connection)
    .await
}

pub async fn get_subscribed_guilds(db_url: String) -> Result<Vec<DBGuild>, sqlx::Error> {
  let mut db_connection = PgConnection::connect(db_url.as_str()).await.expect("Couldn't connect to database.");
  sqlx::query_as!(DBGuild, "SELECT * FROM guilds WHERE has_alerts = true;")
    .fetch_all(&mut db_connection)
    .await
}

pub async fn get_alerts_with_ids(db_url: String, ids: Vec<i32>) -> Result<Vec<DBAlert>, sqlx::Error> {
  let mut db_connection = PgConnection::connect(db_url.as_str()).await.expect("Couldn't connect to database.");
  sqlx::query_as!(DBAlert, "SELECT * FROM alert_history WHERE alert_id = ANY($1);", &ids)
    .fetch_all(&mut db_connection)
    .await
}

pub async fn get_value(db_url: String, key: &str) {
  let mut db_connection = PgConnection::connect(db_url.as_str()).await.expect("Couldn't connect to database.");
  let res = sqlx::query_as!(DBKeyValue, "SELECT * FROM kv_store WHERE key = $1;", key)
    .fetch_one(&mut db_connection)
    .await;

  dbg!(res.unwrap());
}