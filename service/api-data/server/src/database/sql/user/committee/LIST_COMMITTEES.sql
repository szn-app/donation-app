-- Schema Types:
-- id_profile: BIGINT (references user.profile(id))
-- id_community: BIGINT (references user.community(id))
-- member_role: committee_role ENUM ('organizer', 'member')
-- joined_at: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
-- Note: PRIMARY KEY (id_profile, id_community)

SELECT * FROM committee 