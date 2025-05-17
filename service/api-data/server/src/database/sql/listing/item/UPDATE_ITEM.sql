UPDATE item
SET title = COALESCE($2, title),
    description = COALESCE($3, description),
    id_category = COALESCE($4, id_category),
    id_location = COALESCE($5, id_location),
    price = COALESCE($6, price),
    currency = COALESCE($7, currency)
WHERE id = $1
RETURNING * 