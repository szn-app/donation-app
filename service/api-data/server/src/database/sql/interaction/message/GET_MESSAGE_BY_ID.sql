SELECT id, id_sender, id_transaction, type, content, sent_at, updated_at
FROM "interaction"."message"
WHERE id = $1; 