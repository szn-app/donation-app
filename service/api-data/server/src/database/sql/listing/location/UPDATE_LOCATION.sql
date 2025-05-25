-- Update location
-- Parameters:
-- $1: id (BIGINT)
-- $2: address_line1 (VARCHAR(64))
-- $3: address_line2 (VARCHAR(64))
-- $4: city (VARCHAR(50))
-- $5: state (VARCHAR(50))
-- $6: district (VARCHAR(100))
-- $7: country (VARCHAR(50))
-- $8: longitude (FLOAT8) - X coordinate
-- $9: latitude (FLOAT8) - Y coordinate
-- $10: entrance_note (TEXT)
UPDATE "listing"."location"
SET 
    address_line1 = $2,
    address_line2 = $3,
    city = $4,
    state = $5,
    district = $6,
    country = $7,
    geom = CASE 
        WHEN $8 IS NOT NULL AND $9 IS NOT NULL 
        THEN ST_SetSRID(ST_MakePoint($8, $9), 4326)::geography
        ELSE geom 
    END,
    entrance_note = $10
WHERE id = $1
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