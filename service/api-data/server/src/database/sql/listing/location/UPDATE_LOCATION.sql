-- Update location
UPDATE "listing"."location"
SET 
    address_line1 = $2,
    address_line2 = $3,
    city = $4,
    state = $5,
    district = $6,
    country = $7,
    geom = $8,
    entrance_note = $9
WHERE id = $1
RETURNING 
    id,
    address_line1,
    address_line2,
    city,
    state,
    district,
    country,
    geom,
    entrance_note,
    created_at; 