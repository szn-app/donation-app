INSERT INTO "user"."community" (title, description, type, owner, created_by)
VALUES ($1, $2, $3, $4, $5)
RETURNING id, title, description, type, owner, created_at, updated_at, created_by; 