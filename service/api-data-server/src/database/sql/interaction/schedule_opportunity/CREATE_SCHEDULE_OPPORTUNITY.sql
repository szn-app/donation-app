-- Schema Types:
-- id: BIGINT (references interaction.message(id))
-- window_start: TIMESTAMPTZ
-- window_end: TIMESTAMPTZ
-- Parameters:
-- $1: BIGINT - message id
-- $2: TIMESTAMPTZ - window start time
-- $3: TIMESTAMPTZ - window end time

INSERT INTO "interaction"."schedule_opportunity" (id, window_start, window_end)
VALUES ($1, $2, $3)
RETURNING id, window_start, window_end; 