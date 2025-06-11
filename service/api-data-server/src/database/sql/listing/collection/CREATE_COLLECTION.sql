-- Create a new collection
INSERT INTO "listing"."collection" (
    id_community,
    title,
    visibility,
    variant,
    position
)
VALUES (
    $1, -- id_community (BIGINT)
    $2, -- title (VARCHAR(150))
    $3, -- visibility (collection_visibility)
    $4, -- variant (collection_type)
    $5  -- position (INT4)
)
RETURNING 
    id,
    id_community,
    title,
    visibility,
    variant,
    position,
    created_at,
    updated_at; 