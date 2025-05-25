-- Update location
UPDATE "listing"."location"
SET 
    address_line1 = $2, -- VARCHAR(64)
    address_line2 = $3, -- VARCHAR(64)
    city = $4,         -- VARCHAR(50)
    state = $5,        -- VARCHAR(50)
    district = $6,     -- VARCHAR(100)
    country = $7,      -- VARCHAR(50)
    geom = $8,         -- GEOGRAPHY(Point, 4326)
    entrance_note = $9  -- TEXT
WHERE id = $1
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