UPDATE "listing"."collection"
SET 
    title = $2,
    visibility = $3,
    type = $4,
    position = $5,
    updated_at = CURRENT_TIMESTAMP
WHERE id = $1
RETURNING 
    id,
    id_community,
    title,
    visibility,
    type,
    position,
    created_at,
    updated_at; 