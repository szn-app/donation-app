-- Get appointments by creator
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
WHERE created_by = $1
ORDER BY created_at DESC; 