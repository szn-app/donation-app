-- Get all items
SELECT 
    id,
    type,
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
ORDER BY created_at DESC; 