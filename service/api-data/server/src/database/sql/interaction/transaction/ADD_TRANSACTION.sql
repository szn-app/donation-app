INSERT INTO "interaction"."transaction" (id_pledge, status, id_schedule, id_location)
VALUES ($1, $2, $3, $4)
RETURNING id, id_pledge, status, id_schedule, id_location, created_at, updated_at; 