-- Create a new location
-- Parameters:
-- $1: address_line1 (VARCHAR(64))
-- $2: address_line2 (VARCHAR(64))
-- $3: city (VARCHAR(50))
-- $4: state (VARCHAR(50))
-- $5: district (VARCHAR(100))
-- $6: country (VARCHAR(50))
-- $7: longitude (FLOAT8) - X coordinate
-- $8: latitude (FLOAT8) - Y coordinate
-- $9: entrance_note (TEXT)
INSERT INTO "listing"."location" (
    address_line1,
    address_line2,
    city,
    state,
    district,
    country,
    geom,
    entrance_note
)
VALUES (
    $1,
    $2,
    $3,
    $4,
    $5,
    $6,
    CASE 
        WHEN $7 IS NOT NULL AND $8 IS NOT NULL 
        THEN ST_SetSRID(ST_MakePoint($7, $8), 4326)::geography
        ELSE NULL 
    END,
    $9
)
RETURNING 
    id,
    address_line1,
    address_line2,
    city,
    district,
    state,
    country,
    ST_AsText(geom) as geom_text,
    ST_X(geom::geometry) as longitude,
    ST_Y(geom::geometry) as latitude,
    entrance_note,
    created_at; 