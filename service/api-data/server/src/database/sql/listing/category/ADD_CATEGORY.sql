INSERT INTO category (name, description, parent_id)
VALUES ($1, $2, $3)
RETURNING * 