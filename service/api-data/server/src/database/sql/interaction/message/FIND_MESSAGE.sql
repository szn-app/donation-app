-- Get message by ID
SELECT 
    id,
    id_item,
    content,
    created_by,
    created_at
FROM "interaction"."message"
WHERE id = $1; 