-- Get appointment by ID
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
WHERE id = $1; 