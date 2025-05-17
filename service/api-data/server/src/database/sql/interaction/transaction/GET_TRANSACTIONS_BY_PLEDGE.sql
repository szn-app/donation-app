SELECT id, id_pledge, status, id_schedule, id_location, created_at, updated_at
FROM "interaction"."transaction"
WHERE id_pledge = $1; 