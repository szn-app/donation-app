INSERT INTO listing.publish (
    id_item, 
    id_collection, 
    note, 
    position, 
    added_by
) VALUES (
    $1, $2, $3, $4, $5
) RETURNING *; 