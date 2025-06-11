-- Find locations within specified radius
-- Parameters:
-- $1: longitude (FLOAT8) - Center point X coordinate
-- $2: latitude (FLOAT8) - Center point Y coordinate
-- $3: radius_meters (FLOAT8) - Search radius in meters
-- $4: limit (INTEGER) - Maximum number of results to return
SELECT 
    l.id,
    l.address_line1,
    l.address_line2,
    l.city,
    l.district,
    l.state,
    l.country,
    ST_AsText(l.geom) as geom_text,
    ST_X(l.geom::geometry) as longitude,
    ST_Y(l.geom::geometry) as latitude,
    ST_Distance(
        l.geom::geography, 
        ST_SetSRID(ST_MakePoint($1, $2), 4326)::geography
    ) as distance_meters,
    l.entrance_note,
    l.created_at
FROM "listing"."location" l
WHERE ST_DWithin(
    l.geom::geography,
    ST_SetSRID(ST_MakePoint($1, $2), 4326)::geography,
    $3
)
ORDER BY l.geom::geography <-> ST_SetSRID(ST_MakePoint($1, $2), 4326)::geography
LIMIT $4; 