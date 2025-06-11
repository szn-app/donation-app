-- Get all collections
SELECT 
    id,
    id_community,
    title,
    visibility,
    variant,
    position,
    created_at,
    updated_at
FROM "listing"."collection"
ORDER BY position, created_at DESC; 