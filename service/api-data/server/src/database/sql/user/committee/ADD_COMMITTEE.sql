INSERT INTO "user"."committee" (id_profile, id_community, member_role)
VALUES ($1, $2, $3)
RETURNING id_profile, id_community, member_role, joined_at; 