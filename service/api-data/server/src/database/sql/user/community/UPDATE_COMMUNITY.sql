-- Schema Types:
-- id: BIGINT (GENERATED ALWAYS AS IDENTITY PRIMARY KEY)
-- title: VARCHAR(150)
-- description: TEXT (max length 10000)
-- variant: community_type ENUM ('solo', 'organized') NOT NULL DEFAULT 'solo'
-- owner: UUID NOT NULL (references user.account(id))
-- created_at: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
-- updated_at: TIMESTAMPTZ NULL
-- created_by: UUID NOT NULL (references user.account(id))
-- Parameters:
-- $1: BIGINT - community id
-- $2: VARCHAR(150) - new title
-- $3: TEXT - new description
-- $4: community_type - new variant
-- $5: TIMESTAMPTZ - updated_at timestamp

UPDATE "user"."community" 
SET 
    title = COALESCE($2, title),
    description = COALESCE($3, description),
    variant = COALESCE($4, variant),
    updated_at = $5 
WHERE id = $1 
RETURNING 
    id, 
    title, 
    description, 
    variant, 
    owner, 
    created_at, 
    updated_at, 
    created_by 