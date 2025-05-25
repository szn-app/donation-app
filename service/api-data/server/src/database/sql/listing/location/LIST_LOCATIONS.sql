-- Get all locations
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
ORDER BY created_at DESC; 