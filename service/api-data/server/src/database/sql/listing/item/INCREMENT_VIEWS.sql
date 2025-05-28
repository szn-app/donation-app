UPDATE "listing"."item" 
SET views_count = views_count + 1
WHERE id = $1 
RETURNING *; 