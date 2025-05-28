UPDATE "listing"."collection"
SET 
    title = $2,
    visibility = $3,
    variant = $4,
    position = $5,
    updated_at = CURRENT_TIMESTAMP
WHERE id = $1
RETURNING 
    id,
    id_community,
    title,
    visibility,
    variant,
    position,
    created_at,
    updated_at; 