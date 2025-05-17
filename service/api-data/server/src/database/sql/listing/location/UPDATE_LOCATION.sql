UPDATE location
SET name = COALESCE($2, name),
    address = COALESCE($3, address),
    city = COALESCE($4, city),
    state = COALESCE($5, state),
    country = COALESCE($6, country),
    postal_code = COALESCE($7, postal_code)
WHERE id = $1
RETURNING * 