-- Schema Types:
-- id: UUID PRIMARY KEY
-- remarks: TEXT NULL
-- created_at: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
-- Parameters:
-- $1: UUID - account id
-- $2: TEXT - remarks

-- Create a new user account
INSERT INTO "user"."account" (id, remarks) 
VALUES ($1, $2) 
RETURNING id, remarks, created_at;