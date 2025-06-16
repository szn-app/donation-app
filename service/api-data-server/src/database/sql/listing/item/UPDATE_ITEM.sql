-- Update item
UPDATE "listing"."item"
SET 
    -- variant = COALESCE($2, variant),
    -- intent_action = COALESCE($3, intent_action),
    status = COALESCE($2, status),
    title = COALESCE($3, title),
    description = COALESCE($4, description),
    category = COALESCE($5, category),
    condition = COALESCE($6, condition),
    location = COALESCE($7, location),
    updated_at = CURRENT_TIMESTAMP
WHERE id = $1
RETURNING 
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
    created_by;