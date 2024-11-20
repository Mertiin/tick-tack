-- Add migration script here
ALTER TABLE organizations
ADD CONSTRAINT unique_name UNIQUE (name);

ALTER TABLE organizations
ALTER COLUMN owner_user_id SET NOT NULL;