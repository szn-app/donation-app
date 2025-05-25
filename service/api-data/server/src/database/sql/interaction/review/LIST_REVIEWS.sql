-- Schema Types:
-- id_transaction: BIGINT (references interaction.transaction(id))
-- id_subject_profile: BIGINT (references user.profile(id))
-- reviewer: BIGINT (references user.profile(id))
-- comment: TEXT (max length 10000)
-- score: INT2 DEFAULT 0 NOT NULL
-- created_at: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
-- Note: PRIMARY KEY (id_transaction, id_subject_profile)

SELECT id_transaction, id_subject_profile, reviewer, comment, score, created_at
FROM "interaction"."review"; 