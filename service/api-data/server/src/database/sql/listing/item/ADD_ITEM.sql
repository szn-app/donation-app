INSERT INTO item (title, description, id_category, id_profile, id_location, price, currency)
VALUES ($1, $2, $3, $4, $5, $6, $7)
RETURNING * 