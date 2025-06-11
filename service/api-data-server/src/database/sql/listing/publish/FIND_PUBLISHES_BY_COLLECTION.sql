SELECT * FROM "listing"."publish" 
WHERE id_collection = $1 
ORDER BY position; 