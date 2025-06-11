-- Get child categories for a parent category
SELECT 
    id,
    title,
    description,
    category_parent
FROM "listing"."category"
WHERE category_parent = $1
ORDER BY title; 