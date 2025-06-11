-- Create a new location
-- Parameters:
-- $1: address_line1 (VARCHAR(64))
-- $2: address_line2 (VARCHAR(64))
-- $3: city (VARCHAR(50))
-- $4: state (VARCHAR(50))
-- $5: district (VARCHAR(100))
-- $6: country (VARCHAR(50))
-- $7: geom (GEOGRAPHY(Point, 4326))
-- $8: entrance_note (TEXT)
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
    $7,
    $8
)
RETURNING 
    id,
    address_line1,
    address_line2,
    city,
    district,
    state,
    country,
    geom,
    entrance_note,
    created_at; 