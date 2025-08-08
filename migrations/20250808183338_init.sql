-- Add migration script here
CREATE TABLE IF NOT EXISTS guilds (
  guild_id BIGINT NOT NULL,
  guild_name TEXT,
  has_alerts BOOLEAN,
  alert_channel BIGINT,
  accessibility_alerts BOOLEAN,
  planned_alerts BOOLEAN,
  route_ids TEXT [],
  ephemeral_arrivals BOOLEAN,
  PRIMARY KEY(guild_id)
);
CREATE TABLE IF NOT EXISTS alert_history (
  alert_id INT NOT NULL,
  headline TEXT,
  short_description TEXT,
  guid TEXT,
  published_to INT,
  PRIMARY KEY(alert_id)
);
CREATE TABLE IF NOT EXISTS kv_store (
  key TEXT NOT NULL,
  value TEXT,
  PRIMARY KEY(key)
);