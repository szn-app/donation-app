-- Schema Types:
-- id_profile: BIGINT (references user.profile(id))
-- id_community: BIGINT (references user.community(id))
-- member_role: committee_role ENUM ('organizer', 'member')
-- joined_at: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
-- Parameters:
-- $1: BIGINT - community id to search for

SELECT id_profile, id_community, member_role, joined_at
FROM "user"."committee" 
WHERE id_community = $1; 