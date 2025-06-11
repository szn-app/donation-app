-- Get messages by creator
SELECT 
    id,
    id_item,
    content,
    created_by,
    created_at
FROM "interaction"."message"
WHERE created_by = $1
ORDER BY created_at DESC; 