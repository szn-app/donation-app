SELECT id, scheduled_for
FROM "interaction"."schedule"
WHERE id = $1; 