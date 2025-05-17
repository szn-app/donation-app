UPDATE media
SET url = COALESCE($2, url),
    media_type = COALESCE($3, media_type),
    position = COALESCE($4, position)
WHERE id = $1
RETURNING * 