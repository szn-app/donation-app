-- Schema Types:
-- id: BIGINT (GENERATED ALWAYS AS IDENTITY PRIMARY KEY)
-- name: VARCHAR(100)
-- description: TEXT (max length 10000)
-- type: profile_type ENUM ('individual', 'organization', 'company')
-- owner: UUID NOT NULL (references user.account(id))
-- created_at: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
-- updated_at: TIMESTAMPTZ NULL
-- created_by: UUID NOT NULL (references user.account(id))
-- Parameters:
-- $1: BIGINT - profile id
-- $2: VARCHAR(100) - new name
-- $3: TEXT - new description
-- $4: profile_type - new type
-- $5: TIMESTAMPTZ - updated_at timestamp

UPDATE "user".profile 
SET name = $2, description = $3, type = $4, updated_at = $5 
WHERE id = $1 
RETURNING id, name, description, type, owner, created_at, updated_at, created_by 