UPDATE "listing"."item" 
SET is_reported = true
WHERE id = $1 
RETURNING *; 