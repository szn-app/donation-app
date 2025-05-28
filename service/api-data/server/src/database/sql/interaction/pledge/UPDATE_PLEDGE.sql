UPDATE "interaction"."pledge" 
SET status = $2
WHERE id = $1
RETURNING * 