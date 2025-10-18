-- Add migration script here
UPDATE alert_history
SET headline = 'No headline saved'
WHERE headline IS NULL;

UPDATE alert_history
SET short_description = 'No short_description saved'
WHERE short_description IS NULL;

UPDATE alert_history
SET published_to = 0
WHERE published_to IS NULL;

ALTER TABLE alert_history
ALTER COLUMN headline SET NOT NULL;

ALTER TABLE alert_history
ALTER COLUMN short_description SET NOT NULL;

ALTER TABLE alert_history
DROP COLUMN guid;

ALTER TABLE alert_history
ALTER COLUMN published_to SET NOT NULL;


