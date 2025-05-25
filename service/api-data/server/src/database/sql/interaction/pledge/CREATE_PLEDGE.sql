-- Schema Types:
-- id: BIGINT (GENERATED ALWAYS AS IDENTITY PRIMARY KEY)
-- id_profile: BIGINT (references user.profile(id))
-- id_item: BIGINT (references listing.item(id))
-- intent_action: pledge_intent_action ENUM ('give', 'receive')
-- message: TEXT (max length 10000)
-- status: pledge_status ENUM ('pending', 'approved', 'declined') NOT NULL DEFAULT 'pending'
-- pledged_at: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
-- updated_at: TIMESTAMPTZ NULL
-- updated_by: UUID (references user.account(id))
-- Parameters:
-- $1: BIGINT - profile id
-- $2: BIGINT - item id
-- $3: pledge_intent_action - intent action ('give' or 'receive')
-- $4: TEXT - message
-- $5: pledge_status - status ('pending', 'approved', or 'declined')

INSERT INTO "interaction"."pledge" (id_profile, id_item, intent_action, message, status)
VALUES ($1, $2, $3, $4, $5)
RETURNING id, id_profile, id_item, intent_action, message, status, pledged_at, updated_at, updated_by; 