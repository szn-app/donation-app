INSERT INTO review (id_transaction, id_subject, rating, comment)
VALUES ($1, $2, $3, $4)
RETURNING * 