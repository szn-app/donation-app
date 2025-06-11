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

SELECT id, id_profile, id_item, intent_action, message, status, pledged_at, updated_at, updated_by
FROM "interaction"."pledge"; 