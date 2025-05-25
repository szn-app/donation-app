-- Schema Types:
-- id: BIGINT (GENERATED ALWAYS AS IDENTITY PRIMARY KEY)
-- type: item_type ENUM ('in-kind', 'inquiry', 'monetary', 'service')
-- intent_action: item_intent_action ENUM ('request', 'offer')
-- status: item_status ENUM ('draft', 'active', 'disabled', 'archived')
-- title: VARCHAR(150)
-- description: TEXT (max length 10000)
-- category: BIGINT (references listing.category(id))
-- condition: item_condition ENUM ('brand_new', 'pre_owned_barely_used', 'pre_owned_usable', 'pre_owned_damaged')
-- location: BIGINT (references listing.location(id))
-- views_count: BIGINT (DEFAULT 0)
-- is_reported: BOOLEAN (DEFAULT FALSE)
-- created_at: TIMESTAMPTZ (DEFAULT CURRENT_TIMESTAMP)
-- updated_at: TIMESTAMPTZ
-- created_by: UUID (references user.account(id))

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