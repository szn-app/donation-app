-- Schema Types:
-- id: BIGINT (GENERATED ALWAYS AS IDENTITY PRIMARY KEY)
-- id_pledge: BIGINT (references interaction.pledge(id))
-- status: transaction_status ENUM ('in-progress', 'completed', 'cancelled') NOT NULL DEFAULT 'in-progress'
-- id_schedule: BIGINT (references interaction.schedule(id))
-- id_location: BIGINT (references listing.location(id))
-- created_at: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
-- updated_at: TIMESTAMPTZ NULL
-- Parameters:
-- $1: BIGINT - pledge id
-- $2: transaction_status - status ('in-progress', 'completed', or 'cancelled')
-- $3: BIGINT - schedule id
-- $4: BIGINT - location id

INSERT INTO "interaction"."transaction" (id_pledge, status, id_schedule, id_location)
VALUES ($1, $2, $3, $4)
RETURNING id, id_pledge, status, id_schedule, id_location, created_at, updated_at; 