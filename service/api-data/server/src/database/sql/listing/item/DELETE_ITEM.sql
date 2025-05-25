-- Delete item
DELETE FROM "listing"."item"
WHERE id = $1 
RETURNING id; 