INSERT INTO "user".committee (id_profile, id_community, member_role, joined_at) 
VALUES ($1, $2, $3, $4) 
RETURNING id_profile, id_community, member_role, joined_at 