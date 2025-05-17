INSERT INTO location (name, address, city, state, country, postal_code, id_profile)
VALUES ($1, $2, $3, $4, $5, $6, $7)
RETURNING * 