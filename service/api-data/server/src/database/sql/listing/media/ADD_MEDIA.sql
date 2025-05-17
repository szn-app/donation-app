INSERT INTO "listing"."media" (id_item, caption, url, type)
VALUES ($1, $2, $3, $4)
RETURNING id, id_item, caption, url, type, created_at; 