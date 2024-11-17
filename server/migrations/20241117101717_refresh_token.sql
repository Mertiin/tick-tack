-- Add migration script here
CREATE TABLE refresh_tokens (
    refresh_token_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    token TEXT NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE refresh_tokens
ADD CONSTRAINT fk_user
FOREIGN KEY (user_id) REFERENCES users(user_id);