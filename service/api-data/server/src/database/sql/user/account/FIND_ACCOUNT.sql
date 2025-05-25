-- Schema Types:
-- id: UUID PRIMARY KEY
-- remarks: TEXT NULL
-- created_at: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
-- Parameters:
-- $1: UUID - account id to find

SELECT id, remarks, created_at FROM "user"."account" WHERE id = $1;