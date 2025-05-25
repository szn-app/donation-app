-- Update publish entry
UPDATE listing.publish
SET note = $3, 
    position = $4,
    updated_at = $5
WHERE id_item = $1 
  AND id_collection = $2 
RETURNING *; 