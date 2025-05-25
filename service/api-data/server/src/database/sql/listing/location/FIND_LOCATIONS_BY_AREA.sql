-- Get locations within a radius of a point
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
    created_at,
    extension.ST_Distance(geom::extension.geometry, $1::extension.geometry) as distance
FROM "listing"."location"
WHERE extension.ST_DWithin(geom::extension.geometry, $1::extension.geometry, $2)
ORDER BY distance; 