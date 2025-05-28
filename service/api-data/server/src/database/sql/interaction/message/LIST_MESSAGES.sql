-- Schema Types:
-- id: BIGINT (GENERATED ALWAYS AS IDENTITY PRIMARY KEY)
-- id_sender: BIGINT (references user.profile(id))
-- id_transaction: BIGINT (references interaction.transaction(id))
-- variant: message_type ENUM ('text', 'schedule_opportunity')
-- content: TEXT (max length 10000)
-- sent_at: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
-- updated_at: TIMESTAMPTZ NULL

SELECT id, id_sender, id_transaction, variant, content, sent_at, updated_at
FROM "interaction"."message"; 