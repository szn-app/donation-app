
--------- Postgresql sematnics https://www.postgresql.org/docs/current/protocol-flow.html#PROTOCOL-FLOW-MULTI-STATEMENT

-- example table
create table IF NOT EXISTS test (i integer);
insert into test (i) select generate_series(1, 100);
GRANT ALL ON test TO "postgres-user";
---

CREATE SCHEMA IF NOT EXISTS user AUTHORIZATION "postgres-user";
-- app tables: 
CREATE TABLE IF NOT EXISTS user.account (
    id UUID NOT NULL UNIQUE
);
GRANT ALL ON user.account TO "postgres-user";

---
-- TODO: 
-- CREATE EXTENSION postgis;

------------------------------------------------------------------
-- [MANUALLY AUTOGENRATED FROM CHARTDB DESIGN]

--- TODO: update and manually modify to grant permission and merge with above SQL code

-- Schema Definitions

-- User Schema
CREATE SCHEMA IF NOT EXISTS "user";
CREATE SCHEMA IF NOT EXISTS "listing";
CREATE SCHEMA IF NOT EXISTS "interaction";

-- user.account Table
CREATE TABLE IF NOT EXISTS "user"."account" (
    id UUID PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- user.community Table
CREATE TABLE IF NOT EXISTS "user"."community" (
    id BIGINT PRIMARY KEY,
    organizer UUID,
    title VARCHAR(255),
    description TEXT,
    type ENUM('individual', 'organization', 'other'),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by UUID
);

-- user.committee Table
CREATE TABLE IF NOT EXISTS "user"."committee" (
    id_profile BIGINT,
    id_community BIGINT,
    role ENUM('organizer', 'moderator', 'member'),
    joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id_profile, id_community),
    CONSTRAINT fk_committee_community FOREIGN KEY (id_community) REFERENCES "user"."community"(id),
    CONSTRAINT fk_committee_profile FOREIGN KEY (id_profile) REFERENCES "user"."profile"(id)
);

-- user.profile Table
CREATE TABLE IF NOT EXISTS "user"."profile" (
    id BIGINT PRIMARY KEY,
    owner UUID,
    name VARCHAR(255),
    description TEXT,
    type ENUM('individual', 'organization', 'other'),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by UUID
);

-- listing.item Table
CREATE TABLE IF NOT EXISTS "listing"."item" (
    id BIGINT PRIMARY KEY,
    type ENUM('sale', 'request', 'inquiry'),
    intent_action ENUM('request', 'offer'),
    status ENUM('active', 'completed', 'closed'),
    title VARCHAR(255),
    description TEXT,
    id_category BIGINT,
    condition ENUM('new', 'used', 'refurbished'),
    id_location BIGINT,
    views_count BIGINT DEFAULT 0,
    is_reported BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by UUID,
    CONSTRAINT fk_item_category FOREIGN KEY (id_category) REFERENCES "listing"."category"(id),
    CONSTRAINT fk_item_location FOREIGN KEY (id_location) REFERENCES "listing"."location"(id)
);

-- listing.collection Table
CREATE TABLE IF NOT EXISTS "listing"."collection" (
    id BIGINT PRIMARY KEY,
    id_community BIGINT,
    title VARCHAR(255),
    visibility ENUM('public', 'private'),
    type ENUM('featured', 'special_interest')
);

-- listing.category Table
CREATE TABLE IF NOT EXISTS "listing"."category" (
    id BIGINT PRIMARY KEY,
    title VARCHAR(255),
    description TEXT,
    id_category_parent BIGINT
);

-- listing.media Table
CREATE TABLE IF NOT EXISTS "listing"."media" (
    id BIGINT PRIMARY KEY,
    id_item BIGINT,
    url VARCHAR(255),
    type ENUM('image', 'video'),
    CONSTRAINT fk_media_item FOREIGN KEY (id_item) REFERENCES "listing"."item"(id)
);

-- listing.location Table
CREATE TABLE IF NOT EXISTS "listing"."location" (
    id BIGINT PRIMARY KEY,
    address_line1 VARCHAR(255),
    address_line2 VARCHAR(255),
    city VARCHAR(255),
    state VARCHAR(255),
    district BIGINT,
    country VARCHAR(255),
    latitude DECIMAL,
    longitude DECIMAL,
    entrance_note TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- interaction.pledge Table
CREATE TABLE IF NOT EXISTS "interaction"."pledge" (
    id BIGINT PRIMARY KEY,
    id_profile BIGINT,
    id_item BIGINT,
    intent_action ENUM('donate', 'receive'),
    message TEXT,
    status ENUM('pending', 'approved', 'declined'),
    pledged_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_pledge_profile FOREIGN KEY (id_profile) REFERENCES "user"."profile"(id),
    CONSTRAINT fk_pledge_item FOREIGN KEY (id_item) REFERENCES "listing"."item"(id)
);

-- interaction.transaction Table
CREATE TABLE IF NOT EXISTS "interaction"."transaction" (
    id BIGINT PRIMARY KEY,
    id_pledge BIGINT,
    status ENUM('completed', 'in-progress', 'failed'),
    id_schedule BIGINT,
    id_location BIGINT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP,
    CONSTRAINT fk_transaction_pledge FOREIGN KEY (id_pledge) REFERENCES "interaction"."pledge"(id),
    CONSTRAINT fk_transaction_location FOREIGN KEY (id_location) REFERENCES "listing"."location"(id),
    CONSTRAINT fk_transaction_schedule FOREIGN KEY (id_schedule) REFERENCES "interaction"."schedule"(id)
);

-- interaction.message Table
CREATE TABLE IF NOT EXISTS "interaction"."message" (
    id BIGINT PRIMARY KEY,
    id_sender BIGINT,
    id_transaction BIGINT,
    type ENUM('schedule_opportunity', 'message'),
    content TEXT,
    sent_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_message_sender FOREIGN KEY (id_sender) REFERENCES "user"."profile"(id),
    CONSTRAINT fk_message_transaction FOREIGN KEY (id_transaction) REFERENCES "interaction"."transaction"(id)
);

-- interaction.schedule Table
CREATE TABLE IF NOT EXISTS "interaction"."schedule" (
    id BIGINT PRIMARY KEY,
    scheduled_for TIMESTAMP
);

-- interaction.schedule_opportunity Table
CREATE TABLE IF NOT EXISTS "interaction"."schedule_opportunity" (
    id BIGINT PRIMARY KEY,
    window_start TIMESTAMP,
    window_end TIMESTAMP
);

-- interaction.review Table
CREATE TABLE IF NOT EXISTS "interaction"."review" (
    id_transaction BIGINT,
    id_profile BIGINT,
    id_reviewer_profile BIGINT,
    comment TEXT,
    score INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id_transaction, id_profile),
    CONSTRAINT fk_review_transaction FOREIGN KEY (id_transaction) REFERENCES "interaction"."transaction"(id),
    CONSTRAINT fk_review_profile FOREIGN KEY (id_profile) REFERENCES "user"."profile"(id),
    CONSTRAINT fk_review_reviewer_profile FOREIGN KEY (id_reviewer_profile) REFERENCES "user"."profile"(id)
);
