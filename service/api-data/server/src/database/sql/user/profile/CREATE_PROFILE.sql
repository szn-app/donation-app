-- Schema Types:
-- id: BIGINT (GENERATED ALWAYS AS IDENTITY PRIMARY KEY)
-- name: VARCHAR(100)
-- description: TEXT (max length 10000)
-- variant: profile_type ENUM ('individual', 'organization', 'company')
-- owner: UUID NOT NULL (references user.account(id))
-- created_at: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
-- updated_at: TIMESTAMPTZ NULL
-- created_by: UUID NOT NULL (references user.account(id))
-- Parameters:
-- $1: VARCHAR(100) - name
-- $2: TEXT - description
-- $3: profile_type - variant ('individual', 'organization', or 'company')
-- $4: UUID - owner account id
-- $5: UUID - created_by account id

INSERT INTO "user"."profile" (name, description, variant, owner, created_by)
VALUES ($1, $2, $3, $4, $5)
RETURNING id, name, description, variant, owner, created_at, updated_at, created_by; 