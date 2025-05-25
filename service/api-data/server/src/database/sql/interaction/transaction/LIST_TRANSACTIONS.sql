-- Schema Types:
-- id: BIGINT (GENERATED ALWAYS AS IDENTITY PRIMARY KEY)
-- id_pledge: BIGINT (references interaction.pledge(id))
-- status: transaction_status ENUM ('inprogress', 'completed', 'cancelled') NOT NULL DEFAULT 'inprogress'
-- id_schedule: BIGINT (references interaction.schedule(id))
-- id_location: BIGINT (references listing.location(id))
-- created_at: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
-- updated_at: TIMESTAMPTZ NULL

SELECT id, id_pledge, status, id_schedule, id_location, created_at, updated_at
FROM "interaction"."transaction"; 