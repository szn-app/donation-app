-- Get schedule by ID
SELECT 
    id,
    scheduled_for
FROM "interaction"."schedule"
WHERE id = $1; 