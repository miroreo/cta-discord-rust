-- Add migration script here
ALTER TABLE current_alerts
DROP COLUMN impacted_services;
ALTER TABLE current_alerts
ADD COLUMN impacted_services jsonb[] NOT NULL;