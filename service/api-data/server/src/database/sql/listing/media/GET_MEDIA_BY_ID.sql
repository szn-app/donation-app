SELECT id, id_item, caption, url, type, created_at
FROM "listing"."media"
WHERE id = $1; 