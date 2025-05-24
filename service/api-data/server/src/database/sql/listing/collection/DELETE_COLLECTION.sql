-- Delete collection
DELETE FROM "listing"."collection"
WHERE id = $1
RETURNING id; 