-- Schema Types:
-- id: BIGINT (GENERATED ALWAYS AS IDENTITY PRIMARY KEY)
-- title: VARCHAR(150)
-- description: TEXT (max length 10000)
-- type: community_type ENUM ('solo', 'organized') NOT NULL DEFAULT 'solo'
-- owner: UUID NOT NULL (references user.account(id))
-- created_at: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
-- updated_at: TIMESTAMPTZ NULL
-- created_by: UUID NOT NULL (references user.account(id))

SELECT id, title, description, type, owner, created_at, updated_at, created_by 
FROM "user"."community"; 