-- Schema Types:
-- id: BIGINT (GENERATED ALWAYS AS IDENTITY PRIMARY KEY)
-- scheduled_for: TIMESTAMPTZ NOT NULL

-- Get all schedules
SELECT 
    id,
    scheduled_for
FROM "interaction"."schedule"
ORDER BY scheduled_for; 