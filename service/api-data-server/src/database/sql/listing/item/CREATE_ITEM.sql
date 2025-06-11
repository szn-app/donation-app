-- Create a new listing item
INSERT INTO "listing"."item" (
    variant,
    intent_action,
    status,
    title,
    description,
    category,
    condition,
    location,
    created_by
)
VALUES (
    $1, -- variant (item_type)
    $2, -- intent_action (item_intent_action)
    $3, -- status (item_status)
    $4, -- title (VARCHAR(150))
    $5, -- description (TEXT)
    $6, -- category (BIGINT)
    $7, -- condition (item_condition)
    $8, -- location (BIGINT)
    $9  -- created_by (UUID)
)
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