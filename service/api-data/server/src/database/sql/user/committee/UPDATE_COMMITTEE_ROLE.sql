UPDATE "user"."committee"
SET member_role = $3
WHERE id_profile = $1 AND id_community = $2
RETURNING id_profile, id_community, member_role, joined_at; 