-- Update schedule
UPDATE "interaction"."schedule"
SET 
    scheduled_for = $2
WHERE id = $1
RETURNING 
    id,
    scheduled_for; 