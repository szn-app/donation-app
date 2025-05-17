UPDATE "interaction"."transaction"
SET status = $2, id_schedule = $3, id_location = $4, updated_at = CURRENT_TIMESTAMP
WHERE id = $1
RETURNING id, id_pledge, status, id_schedule, id_location, created_at, updated_at; 