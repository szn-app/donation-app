-- Schema Types:
-- id: BIGINT (GENERATED ALWAYS AS IDENTITY PRIMARY KEY)
-- id_sender: BIGINT (references user.profile(id))
-- id_transaction: BIGINT (references interaction.transaction(id))
-- variant: message_type ENUM ('text', 'schedule_opportunity')
-- content: TEXT (max length 10000)
-- sent_at: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
-- updated_at: TIMESTAMPTZ NULL
-- Parameters:
-- $1: BIGINT - item id
-- $2: TEXT - content
-- $3: BIGINT - created by profile id

-- Create a new message
INSERT INTO "interaction"."message" (
    id_item,
    content,
    created_by
)
VALUES (
    $1, -- id_item (BIGINT)
    $2, -- content (TEXT)
    $3  -- created_by (BIGINT)
)
RETURNING 
    id,
    id_item,
    content,
    created_by,
    created_at; 