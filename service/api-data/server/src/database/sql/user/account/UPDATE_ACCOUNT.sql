-- Schema Types:
-- id: UUID PRIMARY KEY
-- remarks: TEXT NULL
-- created_at: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
-- Parameters:
-- $1: UUID - account id
-- $2: TEXT - new remarks

UPDATE "user".account SET remarks = $2 WHERE id = $1 RETURNING id, remarks, created_at 