-- Schema Types:
-- id: BIGINT (GENERATED ALWAYS AS IDENTITY PRIMARY KEY)
-- name: VARCHAR(100)
-- description: TEXT (max length 10000)
-- type: profile_type ENUM ('individual', 'organization', 'company')
-- owner: UUID NOT NULL (references user.account(id))
-- created_at: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
-- updated_at: TIMESTAMPTZ NULL
-- created_by: UUID NOT NULL (references user.account(id))

SELECT id, name, description, type, owner, created_at, updated_at, created_by 
FROM "user"."profile"; 