-- Delete schedule
DELETE FROM "interaction"."schedule"
WHERE id = $1
RETURNING id; 