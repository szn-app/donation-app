DELETE FROM publish 
WHERE id_item = $1 AND id_collection = $2
RETURNING * 