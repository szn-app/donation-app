-- Create a new schedule
INSERT INTO "interaction"."schedule" (
    scheduled_for
)
VALUES (
    $1  -- scheduled_for (TIMESTAMPTZ)
)
RETURNING 
    id,
    scheduled_for; 