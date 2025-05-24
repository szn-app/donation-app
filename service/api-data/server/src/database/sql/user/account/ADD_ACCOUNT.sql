-- Create a new user account
INSERT INTO "user"."account" (id, remarks) 
VALUES ($1, $2) 
RETURNING id, remarks, created_at;