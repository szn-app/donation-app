UPDATE "user"."profile"
SET name = $2,
    description = $3,
    type = $4,
    updated_at = CURRENT_TIMESTAMP
WHERE id = $1
RETURNING id, name, description, type, owner, created_at, updated_at, created_by; 