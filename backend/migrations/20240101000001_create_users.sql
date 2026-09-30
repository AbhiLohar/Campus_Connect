-- Users table (permanent accounts)
CREATE TABLE users (
    id UUID PRIMARY KEY,
    public_id UUID NOT NULL UNIQUE,
    display_name VARCHAR(50) NOT NULL,
    bio TEXT DEFAULT '',
    avatar_url TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_public_id ON users(public_id);
CREATE INDEX idx_users_status ON users(status);
