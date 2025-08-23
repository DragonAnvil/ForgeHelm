-- This creates basic users table with ID and name

CREATE EXTENSION IF NOT EXISTS  "uuid-ossp";

CREATE TABLE users (
    -- Using UUID as Primary Key (PK) which is globally unique (not table specific unique)
    -- This is good for scaling and does not revela information about specific table record counts
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    -- Name Field with contraints
    -- Less than 255 characters
    -- Not Null
    name VARCHAR(255) NOT NULL CHECK (length(trim(name)) > 0),

    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);


-- Trigger for auto updates on updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();