SELECT c.* FROM "user"."community" c
JOIN committee cm ON c.id = cm.id_community
WHERE cm.id_profile = $1 