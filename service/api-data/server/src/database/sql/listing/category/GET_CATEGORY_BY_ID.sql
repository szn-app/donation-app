SELECT id, title, description, category_parent
FROM "listing"."category"
WHERE id = $1; 