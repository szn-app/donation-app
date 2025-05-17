INSERT INTO "listing"."collection" (id_community, title, visibility, type, position)
VALUES ($1, $2, $3, $4, $5)
RETURNING id, id_community, title, visibility, type, position, created_at, updated_at; 