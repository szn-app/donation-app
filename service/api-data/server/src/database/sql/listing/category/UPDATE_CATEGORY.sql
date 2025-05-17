UPDATE category
SET name = COALESCE($2, name),
    description = COALESCE($3, description),
    parent_id = COALESCE($4, parent_id)
WHERE id = $1
RETURNING * 