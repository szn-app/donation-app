-- Get all categories
SELECT 
    id,
    title,
    description,
    category_parent
FROM "listing"."category"
ORDER BY title; 