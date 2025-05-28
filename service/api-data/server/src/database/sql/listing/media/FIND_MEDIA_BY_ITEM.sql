SELECT id, id_item, caption, url, variant, created_at
FROM "listing"."media"
WHERE id_item = $1; 