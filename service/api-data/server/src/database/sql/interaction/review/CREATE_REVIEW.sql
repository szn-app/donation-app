-- Schema Types:
-- id_transaction: BIGINT (references interaction.transaction(id))
-- id_subject_profile: BIGINT (references user.profile(id))
-- reviewer: BIGINT (references user.profile(id))
-- comment: TEXT (max length 10000)
-- score: INT2 DEFAULT 0 NOT NULL
-- created_at: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
-- Note: PRIMARY KEY (id_transaction, id_subject_profile)
-- Parameters:
-- $1: BIGINT - transaction id
-- $2: BIGINT - subject profile id
-- $3: INT2 - rating score
-- $4: TEXT - comment

INSERT INTO "interaction"."review" (id_transaction, id_subject, rating, comment)
VALUES ($1, $2, $3, $4)
RETURNING id_transaction, id_subject_profile, reviewer, comment, score, created_at; 