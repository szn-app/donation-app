INSERT INTO "interaction"."schedule_opportunity" (id, window_start, window_end)
VALUES ($1, $2, $3)
RETURNING id, window_start, window_end; 