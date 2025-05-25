INSERT INTO "user".account (id, remarks, created_at) 
VALUES ($1, $2, $3) 
RETURNING id, remarks, created_at 