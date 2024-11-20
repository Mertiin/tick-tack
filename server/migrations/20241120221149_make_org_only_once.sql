-- Add migration script here
ALTER TABLE user_organizations
ADD CONSTRAINT unique_user_org UNIQUE (user_id, organization_id);