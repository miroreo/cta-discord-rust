use serde::{Deserialize, Serialize};
use sqlx::{Connection, Executor, FromRow, PgConnection, Postgres};
use sqlx::types::Json;

use crate::cta::alerts::{DateOrDateTime, ImpactedService, Service};

#[derive(Debug, Deserialize, Serialize)]
pub struct DBAlert {
    pub alert_id: i32,
    pub headline: String,
    pub short_description: String,
    pub full_description: String,
    pub severity_score: i32,
    pub severity_color: String,
    pub impact: String,
    pub tbd: bool,
    pub major_alert: bool,
    pub alert_url: String,
    pub impacted_services: Vec<sqlx::types::Json<Service>>,
    pub published_to: i32,
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
        "SELECT 
            impacted_services AS \"impacted_services: Vec<Json<Service>>\",
            headline, short_description, full_description, severity_score, severity_color, impact, tbd, major_alert, alert_url, alert_id, published_to
            FROM current_alerts 
            WHERE alert_id = ANY($1);",
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
