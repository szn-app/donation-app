SELECT id, window_start, window_end
FROM "interaction"."schedule_opportunity"
WHERE id = $1; 