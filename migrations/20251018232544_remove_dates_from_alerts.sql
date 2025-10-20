-- Add migration script here
ALTER TABLE current_alerts
DROP COLUMN event_start;

ALTER TABLE current_alerts
DROP COLUMN event_end;