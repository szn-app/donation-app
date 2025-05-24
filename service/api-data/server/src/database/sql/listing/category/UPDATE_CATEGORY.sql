-- Update category
UPDATE "listing"."category"
SET 
    title = $2,
    description = $3,
    category_parent = $4
WHERE id = $1
RETURNING 
    id,
    title,
    description,
    category_parent; 