-- Create a new appointment
INSERT INTO "interaction"."appointment" (
    id_item,
    id_schedule,
    status,
    note,
    created_by
)
VALUES (
    $1, -- id_item (BIGINT)
    $2, -- id_schedule (BIGINT)
    $3, -- status (appointment_status)
    $4, -- note (TEXT)
    $5  -- created_by (BIGINT)
)
RETURNING 
    id,
    id_item,
    id_schedule,
    status,
    note,
    created_by,
    created_at,
    updated_at; 