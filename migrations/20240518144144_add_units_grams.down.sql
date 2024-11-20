-- Add down migration script here
DELETE FROM unit WHERE name = 'grams';
