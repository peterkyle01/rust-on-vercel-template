-- Create users table
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS users (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        email VARCHAR(255) UNIQUE NOT NULL,
        username VARCHAR(100) UNIQUE NOT NULL,
        password_hash TEXT NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW (),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW ()
    );

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_users_email ON users (email);

CREATE INDEX IF NOT EXISTS idx_users_username ON users (username);

CREATE INDEX IF NOT EXISTS idx_users_created_at ON users (created_at);