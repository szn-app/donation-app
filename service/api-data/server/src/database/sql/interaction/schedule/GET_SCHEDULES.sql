-- Get all schedules
SELECT 
    id,
    scheduled_for
FROM "interaction"."schedule"
ORDER BY scheduled_for; 