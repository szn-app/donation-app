SELECT id_item, id_collection, note, position, added_by, posted_on
FROM "listing"."publish"
WHERE id_item = $1 AND id_collection = $2; 