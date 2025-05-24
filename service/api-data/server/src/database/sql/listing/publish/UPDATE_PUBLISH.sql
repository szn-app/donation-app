-- Update publish entry
UPDATE "listing"."publish"
SET 
    note = $3,
    position = $4
WHERE id_item = $1 AND id_collection = $2
RETURNING 
    id_item,
    id_collection,
    note,
    position,
    added_by,
    posted_on; 