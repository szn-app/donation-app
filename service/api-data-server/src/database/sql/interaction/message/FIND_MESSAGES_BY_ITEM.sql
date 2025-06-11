-- Get messages by item
SELECT 
    id,
    id_item,
    content,
    created_by,
    created_at
FROM "interaction"."message"
WHERE id_item = $1
ORDER BY created_at DESC; 