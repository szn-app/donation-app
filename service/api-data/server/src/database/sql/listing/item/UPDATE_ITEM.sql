-- Update item
UPDATE "listing"."item"
SET 
    variant = COALESCE($2, variant),
    intent_action = COALESCE($3, intent_action),
    status = COALESCE($4, status),
    title = COALESCE($5, title),
    description = COALESCE($6, description),
    category = COALESCE($7, category),
    condition = COALESCE($8, condition),
    location = COALESCE($9, location),
    updated_at = CURRENT_TIMESTAMP
    created_by = $10
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