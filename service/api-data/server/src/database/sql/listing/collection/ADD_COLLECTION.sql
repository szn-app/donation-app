-- Create a new collection
INSERT INTO "listing"."collection" (
    id_community,
    title,
    visibility,
    type,
    position
)
VALUES (
    $1, -- id_community (BIGINT)
    $2, -- title (VARCHAR(150))
    $3, -- visibility (collection_visibility)
    $4, -- type (collection_type)
    $5  -- position (INT4)
)
RETURNING 
    id,
    id_community,
    title,
    visibility,
    type,
    position,
    created_at,
    updated_at; 