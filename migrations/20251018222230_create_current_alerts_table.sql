-- Add migration script here
ALTER TABLE alert_history
RENAME TO current_alerts;

ALTER TABLE current_alerts
ADD COLUMN full_description TEXT NOT NULL;

ALTER TABLE current_alerts
ADD COLUMN severity_score INT NOT NULL;

ALTER TABLE current_alerts
ADD COLUMN severity_color TEXT NOT NULL;

ALTER TABLE current_alerts
ADD COLUMN impact TEXT NOT NULL;

ALTER TABLE current_alerts
ADD COLUMN event_start TEXT NOT NULL;

ALTER TABLE current_alerts
ADD COLUMN event_end TEXT;

ALTER TABLE current_alerts
ADD COLUMN tbd BOOLEAN NOT NULL;

ALTER TABLE current_alerts
ADD COLUMN major_alert BOOLEAN NOT NULL;

ALTER TABLE current_alerts
ADD COLUMN alert_url TEXT NOT NULL;

ALTER TABLE current_alerts
ADD COLUMN impacted_services json NOT NULL;
