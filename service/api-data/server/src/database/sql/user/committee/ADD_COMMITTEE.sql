INSERT INTO committee (name, created_by)
VALUES ($1, $2)
RETURNING * 