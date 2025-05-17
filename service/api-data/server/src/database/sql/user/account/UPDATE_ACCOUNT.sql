UPDATE account
SET email = COALESCE($2, email),
    password_hash = COALESCE($3, password_hash),
    is_active = COALESCE($4, is_active)
WHERE id = $1
RETURNING * 