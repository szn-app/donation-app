SELECT id, title, description, variant, owner, created_at, updated_at, created_by
FROM "user"."community" 
WHERE id = $1; 