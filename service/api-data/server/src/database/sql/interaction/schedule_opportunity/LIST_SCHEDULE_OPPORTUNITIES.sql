-- Schema Types:
-- id: BIGINT (references interaction.message(id))
-- window_start: TIMESTAMPTZ NULL
-- window_end: TIMESTAMPTZ NULL

SELECT id, window_start, window_end
FROM "interaction"."schedule_opportunity"; 