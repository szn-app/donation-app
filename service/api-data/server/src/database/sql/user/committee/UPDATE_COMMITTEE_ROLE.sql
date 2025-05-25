-- Schema Types:
-- id_profile: BIGINT (references user.profile(id))
-- id_community: BIGINT (references user.community(id))
-- member_role: committee_role ENUM ('organizer', 'member')
-- joined_at: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
-- Parameters:
-- $1: BIGINT - profile id
-- $2: BIGINT - community id
-- $3: committee_role - new member role ('organizer' or 'member')

UPDATE "user".committee 
SET member_role = $3 
WHERE id_profile = $1 AND id_community = $2 
RETURNING id_profile, id_community, member_role, joined_at 