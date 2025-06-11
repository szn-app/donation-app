-- Delete category
DELETE FROM "listing"."category"
WHERE id = $1
RETURNING id; 