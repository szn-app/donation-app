UPDATE "listing"."collection"
SET id_community = $2, title = $3, visibility = $4, type = $5, position = $6, updated_at = CURRENT_TIMESTAMP
WHERE id = $1
RETURNING id, id_community, title, visibility, type, position, created_at, updated_at; 