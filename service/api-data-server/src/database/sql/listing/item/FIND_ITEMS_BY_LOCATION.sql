-- Get items by location
SELECT 
    id,
    variant,
    intent_action,
    status,
    title,
    description,
    category,
    condition,
    location,
    views_count,
    is_reported,
    created_at,
    updated_at,
    created_by
FROM "listing"."item"
WHERE location = $1
ORDER BY created_at DESC; 