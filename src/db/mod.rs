use sqlx::{Connection, Executor, FromRow, PgConnection, Postgres};

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

pub async fn get_guilds(
    db: impl Executor<'_, Database = Postgres>,
) -> Result<Vec<DBGuild>, sqlx::Error> {
    sqlx::query_as!(DBGuild, "SELECT * FROM guilds;")
        .fetch_all(db)
        .await
}

pub async fn get_subscribed_guilds(
    db: impl Executor<'_, Database = Postgres>,
) -> Result<Vec<DBGuild>, sqlx::Error> {
    sqlx::query_as!(DBGuild, "SELECT * FROM guilds WHERE has_alerts = true;")
        .fetch_all(db)
        .await
}

pub async fn get_alerts_with_ids(
    db: impl Executor<'_, Database = Postgres>,
    ids: Vec<i32>,
) -> Result<Vec<DBAlert>, sqlx::Error> {
    sqlx::query_as!(
        DBAlert,
        "SELECT * FROM alert_history WHERE alert_id = ANY($1);",
        &ids
    )
    .fetch_all(db)
    .await
}

pub async fn get_value(db: impl Executor<'_, Database = Postgres>, key: &str) {
    let res = sqlx::query_as!(DBKeyValue, "SELECT * FROM kv_store WHERE key = $1;", key)
        .fetch_one(db)
        .await;

    dbg!(res.unwrap());
}
