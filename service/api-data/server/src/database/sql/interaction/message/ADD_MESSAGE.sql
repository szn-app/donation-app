INSERT INTO "interaction"."message" (id_sender, id_transaction, type, content)
VALUES ($1, $2, $3, $4)
RETURNING id, id_sender, id_transaction, type, content, sent_at, updated_at; 