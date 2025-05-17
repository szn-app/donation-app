INSERT INTO "interaction"."pledge" (id_profile, id_item, intent_action, message, status)
VALUES ($1, $2, $3, $4, $5)
RETURNING id, id_profile, id_item, intent_action, message, status, pledged_at, updated_at, updated_by; 