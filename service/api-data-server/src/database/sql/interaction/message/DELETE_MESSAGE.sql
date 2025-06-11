-- Delete message
DELETE FROM "interaction"."message"
WHERE id = $1
RETURNING id; 