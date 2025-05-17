INSERT INTO "listing"."location" (address_line1, address_line2, city, state, district, country, geom, entrance_note)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
RETURNING id, address_line1, address_line2, city, state, district, country, geom, entrance_note, created_at; 