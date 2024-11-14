-- Add migration script here
ALTER TABLE matches
ALTER COLUMN player_two DROP NOT NULL;