DELETE FROM "user"."committee"
WHERE id_profile = $1 AND id_community = $2; 