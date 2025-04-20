
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

CREATE SCHEMA IF NOT EXISTS public;


CREATE TABLE public.test (
    i varchar
);


-- account credentials and verification for app auth
CREATE TABLE public."user.account" (
    -- synced Kratos ID
    id uuid NOT NULL PRIMARY KEY
);

COMMENT ON TABLE public."user.account" IS 'account credentials and verification for app auth';

COMMENT ON COLUMN public."user.account".id IS 'synced Kratos ID';


-- community group, organization group, etc.
CREATE TABLE "user"."group" (
    id bigint NOT NULL PRIMARY KEY,
    name varchar,
    description text,
    -- represents: individual profile, organization profile, community profile
    type text NOT NULL -- Using TEXT instead of ENUM for broader compatibility and flexibility
);

COMMENT ON TABLE "user"."group" IS 'community group, organization group, etc. ';

COMMENT ON COLUMN "user"."group".type IS 'represents: individual profile, organization profile, community profile';


CREATE TABLE "user".membership (
    id_account uuid NOT NULL REFERENCES public."user.account"(id),
    id_group bigint NOT NULL REFERENCES "user"."group"(id),
    -- member, moderator, admin
    --
    -- NOTE: only used for querying purposes; enforcement is encoded in Keto tuple-based graph relationships.
    role text, -- Using TEXT instead of ENUM
    -- use timestamp with timezone
    joined_at timestamptz NOT NULL,
    PRIMARY KEY (id_account, id_group)
);

COMMENT ON COLUMN "user".membership.role IS 'member, moderator, admin

NOTE: only used for querying purposes; enforcement is encoded in Keto tuple-based graph relationships.';
COMMENT ON COLUMN "user".membership.joined_at IS 'use timestamp with timezone';


CREATE TABLE "example$user"."group-specific-type" (
    id bigint NOT NULL PRIMARY KEY REFERENCES "user"."group"(id),
    type_specific_metadata bigint
);