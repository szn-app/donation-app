-- Update location
-- Parameters:
-- $1: id (BIGINT)
-- $2: address_line1 (VARCHAR(64))
-- $3: address_line2 (VARCHAR(64))
-- $4: city (VARCHAR(50))
-- $5: state (VARCHAR(50))
-- $6: district (VARCHAR(100))
-- $7: country (VARCHAR(50))
-- $8: geom (GEOGRAPHY(Point, 4326))
-- $9: entrance_note (TEXT)
UPDATE "listing"."location"
SET 
    address_line1 = COALESCE($2, address_line1),
    address_line2 = COALESCE($3, address_line2),
    city = COALESCE($4, city),
    state = COALESCE($5, state),
    district = COALESCE($6, district),
    country = COALESCE($7, country),
    geom = COALESCE($8, geom),
    entrance_note = COALESCE($9, entrance_note)
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