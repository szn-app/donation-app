SELECT id, id_sender, id_transaction, type, content, sent_at, updated_at
FROM "interaction"."message"
WHERE id_transaction = $1
ORDER BY sent_at ASC; 