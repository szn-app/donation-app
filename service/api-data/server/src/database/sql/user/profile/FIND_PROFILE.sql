SELECT id, name, description, type, owner, created_at, updated_at, created_by
FROM "user".profile 
WHERE id = $1; 