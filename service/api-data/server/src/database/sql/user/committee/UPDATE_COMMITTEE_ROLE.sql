UPDATE committee 
SET member_role = $3
WHERE id_profile = $1 AND id_community = $2
RETURNING * 