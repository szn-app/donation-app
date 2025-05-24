-- Update item
UPDATE "listing"."item"
SET 
    type = $2,
    intent_action = $3,
    status = $4,
    title = $5,
    description = $6,
    category = $7,
    condition = $8,
    location = $9,
    updated_at = CURRENT_TIMESTAMP
WHERE id = $1
RETURNING 
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
    created_by; 