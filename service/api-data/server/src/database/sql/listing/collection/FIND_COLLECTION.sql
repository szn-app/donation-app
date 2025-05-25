-- Get collection by ID
SELECT 
    id,
    id_community,
    title,
    visibility,
    type,
    position,
    created_at,
    updated_at
FROM "listing"."collection"
WHERE id = $1; 