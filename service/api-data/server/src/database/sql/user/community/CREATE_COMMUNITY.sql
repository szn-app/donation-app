-- Schema Types:
-- id: BIGINT (GENERATED ALWAYS AS IDENTITY PRIMARY KEY)
-- title: VARCHAR(150)
-- description: TEXT (max length 10000)
-- type: community_type ENUM ('solo', 'organized') NOT NULL DEFAULT 'solo'
-- owner: UUID NOT NULL (references user.account(id))
-- created_at: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
-- updated_at: TIMESTAMPTZ NULL
-- created_by: UUID NOT NULL (references user.account(id))
-- Parameters:
-- $1: VARCHAR(150) - title
-- $2: TEXT - description
-- $3: community_type - type ('solo' or 'organized')
-- $4: UUID - owner account id
-- $5: UUID - created_by account id

INSERT INTO "user"."community" (title, description, type, owner, created_by)
VALUES ($1, $2, $3, $4, $5)
RETURNING id, title, description, type, owner, created_at, updated_at, created_by; 