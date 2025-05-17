UPDATE "interaction"."schedule_opportunity"
SET window_start = $2, window_end = $3
WHERE id = $1
RETURNING id, window_start, window_end; 