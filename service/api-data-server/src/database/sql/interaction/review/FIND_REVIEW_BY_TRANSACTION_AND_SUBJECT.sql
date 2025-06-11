SELECT id_transaction, id_subject_profile, reviewer, comment, score, created_at
FROM "interaction"."review"
WHERE id_transaction = $1 AND id_subject_profile = $2; 