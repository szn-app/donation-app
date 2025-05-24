-- Create a new location
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
    $1, -- address_line1 (VARCHAR(64))
    $2, -- address_line2 (VARCHAR(64))
    $3, -- city (VARCHAR(50))
    $4, -- state (VARCHAR(50))
    $5, -- district (BIGINT)
    $6, -- country (VARCHAR(50))
    $7, -- geom (GEOGRAPHY(Point, 4326))
    $8  -- entrance_note (TEXT)
)
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