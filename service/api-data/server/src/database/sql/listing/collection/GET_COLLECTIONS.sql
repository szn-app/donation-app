-- Get all collections
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
ORDER BY position, created_at DESC; 