-- Get location by ID
SELECT 
    id,
    address_line1,
    address_line2,
    city,
    state,
    district,
    country,
    geom,
    entrance_note,
    created_at
FROM "listing"."location"
WHERE id = $1; 