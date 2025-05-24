-- Update appointment
UPDATE "interaction"."appointment"
SET 
    id_schedule = $2,
    status = $3,
    note = $4,
    updated_at = CURRENT_TIMESTAMP
WHERE id = $1
RETURNING 
    id,
    id_item,
    id_schedule,
    status,
    note,
    created_by,
    created_at,
    updated_at; 