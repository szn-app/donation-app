-- Get collections by community
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
WHERE id_community = $1
ORDER BY position, created_at DESC; 