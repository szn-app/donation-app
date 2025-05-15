-- Postgresql sematnics https://www.postgresql.org/docs/current/protocol-flow.html#PROTOCOL-FLOW-MULTI-STATEMENT
CREATE SCHEMA IF NOT EXISTS "extension" AUTHORIZATION "postgres-user";
ALTER DEFAULT PRIVILEGES IN SCHEMA "extension"
    GRANT ALL ON TABLES TO "postgres-user";

CREATE EXTENSION IF NOT EXISTS postgis WITH SCHEMA extension CASCADE;
CREATE EXTENSION IF NOT EXISTS vector WITH SCHEMA extension CASCADE;

------------------------------------------------------------------

-- for testing and debug
CREATE SCHEMA IF NOT EXISTS "test" AUTHORIZATION "postgres-user";
ALTER DEFAULT PRIVILEGES IN SCHEMA "test"
GRANT ALL ON TABLES TO "postgres-user";

create table IF NOT EXISTS "test"."test" (i integer, s VARCHAR(100), d TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP);

-- insert into test (i, S) select generate_series(1, 100);
INSERT INTO "test"."test" (i, s)
SELECT
    seq_num,             
    'Item ' || seq_num   -- implicitly casting the integer to text for the 's' column
FROM generate_series(1, 100) AS seq_num;

------------------------------------------------------------------
-- "[MANUALLY GENRATED FROM CHARTDB DESIGN] (NOTE: when mapping from chartDB it requires refining the types and adding permissions for Postgresql)"
-- "NOTE: procedural blocks of postgresql fail to parse by tools, thus avoiding and simplifying migration SQL to a more generic standard SQL"

-- Schema Definitions
-- User Schema
CREATE SCHEMA IF NOT EXISTS "user" AUTHORIZATION "postgres-user";
ALTER DEFAULT PRIVILEGES IN SCHEMA "user"
GRANT ALL ON TABLES TO "postgres-user";
CREATE SCHEMA IF NOT EXISTS "listing" AUTHORIZATION "postgres-user";
ALTER DEFAULT PRIVILEGES IN SCHEMA "listing"
GRANT ALL ON TABLES TO "postgres-user";
CREATE SCHEMA IF NOT EXISTS "interaction" AUTHORIZATION "postgres-user";
ALTER DEFAULT PRIVILEGES IN SCHEMA "interaction"
GRANT ALL ON TABLES TO "postgres-user";

-- user.account Table
CREATE TABLE IF NOT EXISTS "user"."account" (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- user.profile (no dependencies)
CREATE TYPE profile_type AS ENUM ('individual', 'organization', 'company');
CREATE TABLE IF NOT EXISTS "user"."profile" (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name VARCHAR(100),
    description TEXT,
    type profile_type NULL,
    owner UUID NOT NULL REFERENCES "user"."account"(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ NULL,
    created_by UUID NOT NULL REFERENCES "user"."account"(id),
    CONSTRAINT chk_description_length CHECK (char_length(description) <= 10000)
);

-- user.community (no dependencies)
CREATE TYPE community_type AS ENUM ('solo', 'organized');
CREATE TABLE IF NOT EXISTS "user"."community" (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title VARCHAR(150),
    description TEXT,
    type community_type NOT NULL DEFAULT 'solo',
    owner UUID NOT NULL REFERENCES "user"."account"(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ NULL,
    created_by UUID NOT NULL REFERENCES "user"."account"(id),
    CONSTRAINT chk_description_length CHECK (char_length(description) <= 10000)
);

-- user.committee (depends on community, profile)
CREATE TYPE committee_role AS ENUM ('organizer', 'member');
CREATE TABLE IF NOT EXISTS "user"."committee" (
    id_profile BIGINT REFERENCES "user"."profile"(id),
    id_community BIGINT REFERENCES "user"."community"(id),
    member_role committee_role NOT NULL,
    joined_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY KEY (id_profile, id_community)
);

-- listing.category (no dependencies)
CREATE TABLE IF NOT EXISTS "listing"."category" (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title VARCHAR(150) NOT NULL,
    description TEXT NULL,
    category_parent BIGINT NULL REFERENCES "listing"."category"(id) ON DELETE
        SET NULL DEFERRABLE INITIALLY DEFERRED,
    CONSTRAINT chk_no_self_reference CHECK (id <> category_parent), 
    CONSTRAINT chk_description_length CHECK (char_length(description) <= 10000)
);

-- listing.location (no dependencies)
CREATE TABLE IF NOT EXISTS "listing"."location" (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    address_line1 VARCHAR(64) NOT NULL,
    address_line2 VARCHAR(64),
    city VARCHAR(50) NOT NULL,
    state VARCHAR(50),
    district BIGINT,
    country VARCHAR(50) NOT NULL,
    geom extension.GEOGRAPHY(Point, 4326),
    entrance_note TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT chk_entrance_note_length CHECK (char_length(entrance_note) <= 10000)
);

-- listing.item (depends on category, location)
CREATE TYPE item_type AS ENUM ('in-kind', 'inquiry', 'monetary', 'service');
CREATE TYPE item_intent_action AS ENUM ('request', 'offer');
CREATE TYPE item_status AS ENUM ('draft', 'active', 'disabled', 'archived');
CREATE TYPE item_condition AS ENUM ('brand_new', 'pre_owned_barely_used', 'pre_owned_usable', 'pre_owned_damaged');
CREATE TABLE IF NOT EXISTS "listing"."item" (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    type item_type NOT NULL,
    intent_action item_intent_action NOT NULL,
    status item_status NOT NULL DEFAULT 'draft',
    title VARCHAR(150) NULL,
    description TEXT NULL,
    category BIGINT REFERENCES "listing"."category"(id) ON DELETE SET NULL DEFERRABLE INITIALLY DEFERRED,
    condition item_condition NOT NULL,
    location BIGINT REFERENCES "listing"."location"(id) ON DELETE SET NULL DEFERRABLE INITIALLY DEFERRED,
    views_count BIGINT DEFAULT 0 NOT NULL,
    is_reported BOOLEAN DEFAULT FALSE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ NULL,
    created_by UUID REFERENCES "user"."account"(id) ON DELETE SET NULL DEFERRABLE INITIALLY DEFERRED,
    CONSTRAINT chk_description_length CHECK (char_length(description) <= 10000)
);

-- listing.collection (depends on community)
CREATE TYPE collection_visibility AS ENUM ('public', 'restricted');
CREATE TYPE collection_type AS ENUM ('featured', 'regular');
CREATE TABLE IF NOT EXISTS "listing"."collection" (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    id_community BIGINT REFERENCES "user"."community"(id) ON DELETE SET NULL,
    title VARCHAR(150) NULL,
    visibility collection_visibility NOT NULL,
    type collection_type NULL,
    position INT4 NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ NULL
);

-- listing.media (depends on item)
CREATE TYPE media_type AS ENUM ('image', 'video');
CREATE TABLE IF NOT EXISTS "listing"."media" (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    id_item BIGINT REFERENCES "listing"."item"(id) ON DELETE CASCADE,
    caption VARCHAR(150) NULL,
    url VARCHAR(2048) NOT NULL,
    type media_type NOT NULL, 
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- listing.publish
CREATE TABLE "listing"."publish" (
    id_item bigint NOT NULL REFERENCES "listing"."item"("id") ON DELETE CASCADE,
    id_collection bigint NOT NULL REFERENCES "listing"."collection"("id") ON DELETE CASCADE,
    note text NULL,
    position INT4 NOT NULL DEFAULT 0,
    added_by UUID REFERENCES "user"."account"("id") ON DELETE SET NULL,
    posted_on TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id_item", "id_collection")
);

-- interaction.schedule (no dependencies)
CREATE TABLE IF NOT EXISTS "interaction"."schedule" (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    scheduled_for TIMESTAMPTZ NOT NULL
);

-- interaction.pledge (depends on profile, item)
CREATE TYPE pledge_intent_action AS ENUM ('give', 'receive');
CREATE TYPE pledge_status AS ENUM ('pending', 'approved', 'declined');
CREATE TABLE IF NOT EXISTS "interaction"."pledge" (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    id_profile BIGINT REFERENCES "user"."profile"(id) ON DELETE CASCADE,
    id_item BIGINT REFERENCES "listing"."item"(id) ON DELETE CASCADE,
    intent_action pledge_intent_action NOT NULL,
    message TEXT NULL,
    status pledge_status NOT NULL DEFAULT 'pending',
    pledged_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ NULL,
    updated_by UUID REFERENCES "user"."account"(id) ON DELETE SET NULL,
    CONSTRAINT chk_message_length CHECK (char_length(message) <= 10000)
);

-- interaction.transaction (depends on pledge, location, schedule)
CREATE TYPE transaction_status AS ENUM ('in-progress', 'completed', 'cancelled');
CREATE TABLE IF NOT EXISTS "interaction"."transaction" (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    id_pledge BIGINT REFERENCES "interaction"."pledge"(id) ON DELETE CASCADE,
    status transaction_status NOT NULL DEFAULT 'in-progress',
    id_schedule BIGINT REFERENCES "interaction"."schedule"(id) ON DELETE SET NULL,
    id_location BIGINT REFERENCES "listing"."location"(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ NULL
);

-- interaction.message (depends on profile, transaction)
CREATE TYPE message_type AS ENUM ('text', 'schedule_opportunity');
CREATE TABLE IF NOT EXISTS "interaction"."message" (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    id_sender BIGINT REFERENCES "user"."profile"(id) ON DELETE SET NULL,
    id_transaction BIGINT REFERENCES "interaction"."transaction"(id) ON DELETE CASCADE,
    type message_type NULL,
    content TEXT NOT NULL,
    sent_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ NULL, 
    CONSTRAINT chk_content_length CHECK (char_length(content) <= 10000)
);

-- interaction.review (depends on transaction, profile)
CREATE TABLE IF NOT EXISTS "interaction"."review" (
    id_transaction BIGINT REFERENCES "interaction"."transaction"(id) ON DELETE SET NULL,
    id_subject_profile BIGINT REFERENCES "user"."profile"(id) ON DELETE SET NULL,
    reviewer BIGINT REFERENCES "user"."profile"(id),
    comment TEXT,
    score INT2 DEFAULT 0 NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY KEY (id_transaction, id_subject_profile),
    CONSTRAINT chk_comment_length CHECK (char_length(comment) <= 10000)
);

-- interaction.schedule_opportunity (interaction.message)
CREATE TABLE IF NOT EXISTS "interaction"."schedule_opportunity" (
    id BIGINT REFERENCES "interaction"."message"(id),
    window_start TIMESTAMPTZ NULL,
    window_end TIMESTAMPTZ NULL
);