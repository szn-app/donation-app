-- Get appointments by item
SELECT 
    id,
    id_item,
    id_schedule,
    status,
    note,
    created_by,
    created_at,
    updated_at
FROM "interaction"."appointment"
WHERE id_item = $1
ORDER BY created_at DESC; 