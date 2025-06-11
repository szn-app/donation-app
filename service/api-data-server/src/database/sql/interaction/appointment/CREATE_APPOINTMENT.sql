-- Schema Types:
-- id: BIGINT (GENERATED ALWAYS AS IDENTITY PRIMARY KEY)
-- id_item: BIGINT (references listing.item(id))
-- id_schedule: BIGINT (references interaction.schedule(id))
-- status: appointment_status ENUM
-- note: TEXT
-- created_by: UUID (references user.account(id))
-- created_at: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
-- updated_at: TIMESTAMPTZ NULL
-- Parameters:
-- $1: BIGINT - item id
-- $2: BIGINT - schedule id
-- $3: appointment_status - status
-- $4: TEXT - note
-- $5: UUID - created by account id

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