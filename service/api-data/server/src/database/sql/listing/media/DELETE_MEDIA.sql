-- Delete media entry
DELETE FROM "listing"."media"
WHERE id = $1
RETURNING id; 