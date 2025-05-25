-- Schema Types:
-- id_profile: BIGINT (references user.profile(id))
-- id_community: BIGINT (references user.community(id))
-- member_role: committee_role ENUM ('organizer', 'member')
-- joined_at: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
-- Note: PRIMARY KEY (id_profile, id_community)
-- Parameters:
-- $1: BIGINT - profile id
-- $2: BIGINT - community id
-- $3: committee_role - member role ('organizer' or 'member')

INSERT INTO user.committee (id_profile, id_community, member_role)
VALUES ($1, $2, $3)
RETURNING id_profile, id_community, member_role, joined_at; 