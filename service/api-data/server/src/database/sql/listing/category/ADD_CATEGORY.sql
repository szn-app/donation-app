INSERT INTO "listing"."category" (title, description, category_parent)
VALUES ($1, $2, $3)
RETURNING id, title, description, category_parent; 