-- Delete location
DELETE FROM "listing"."location"
WHERE id = $1
RETURNING id; 