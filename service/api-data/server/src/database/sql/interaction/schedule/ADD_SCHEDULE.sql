INSERT INTO "interaction"."schedule" (scheduled_for)
VALUES ($1)
RETURNING id, scheduled_for; 