--
-- PostgreSQL database dump
--

-- Dumped from database version 17.4 (Debian 17.4-1.pgdg110+2)
-- Dumped by pg_dump version 17.4

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: extension; Type: SCHEMA; Schema: -; Owner: postgres-user
--

CREATE SCHEMA extension;


ALTER SCHEMA extension OWNER TO "postgres-user";

--
-- Name: interaction; Type: SCHEMA; Schema: -; Owner: postgres-user
--

CREATE SCHEMA interaction;


ALTER SCHEMA interaction OWNER TO "postgres-user";

--
-- Name: listing; Type: SCHEMA; Schema: -; Owner: postgres-user
--

CREATE SCHEMA listing;


ALTER SCHEMA listing OWNER TO "postgres-user";

--
-- Name: user; Type: SCHEMA; Schema: -; Owner: postgres-user
--

CREATE SCHEMA "user";


ALTER SCHEMA "user" OWNER TO "postgres-user";

--
-- Name: postgis; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS postgis WITH SCHEMA extension;


--
-- Name: EXTENSION postgis; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION postgis IS 'PostGIS geometry and geography spatial types and functions';


--
-- Name: vector; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS vector WITH SCHEMA extension;


--
-- Name: EXTENSION vector; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION vector IS 'vector data type and ivfflat and hnsw access methods';


--
-- Name: collection_type; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.collection_type AS ENUM (
    'featured',
    'regular'
);


ALTER TYPE public.collection_type OWNER TO postgres;

--
-- Name: collection_visibility; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.collection_visibility AS ENUM (
    'public',
    'restricted'
);


ALTER TYPE public.collection_visibility OWNER TO postgres;

--
-- Name: committee_role; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.committee_role AS ENUM (
    'organizer',
    'member'
);


ALTER TYPE public.committee_role OWNER TO postgres;

--
-- Name: community_type; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.community_type AS ENUM (
    'solo',
    'organized'
);


ALTER TYPE public.community_type OWNER TO postgres;

--
-- Name: item_condition; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.item_condition AS ENUM (
    'brand_new',
    'pre_owned_barely_used',
    'pre_owned_usable',
    'pre_owned_damaged'
);


ALTER TYPE public.item_condition OWNER TO postgres;

--
-- Name: item_intent_action; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.item_intent_action AS ENUM (
    'request',
    'offer'
);


ALTER TYPE public.item_intent_action OWNER TO postgres;

--
-- Name: item_status; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.item_status AS ENUM (
    'draft',
    'active',
    'disabled',
    'archived'
);


ALTER TYPE public.item_status OWNER TO postgres;

--
-- Name: item_type; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.item_type AS ENUM (
    'inkind',
    'inquiry',
    'monetary',
    'service'
);


ALTER TYPE public.item_type OWNER TO postgres;

--
-- Name: media_type; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.media_type AS ENUM (
    'image',
    'video'
);


ALTER TYPE public.media_type OWNER TO postgres;

--
-- Name: message_type; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.message_type AS ENUM (
    'text',
    'schedule_opportunity'
);


ALTER TYPE public.message_type OWNER TO postgres;

--
-- Name: pledge_intent_action; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.pledge_intent_action AS ENUM (
    'give',
    'receive'
);


ALTER TYPE public.pledge_intent_action OWNER TO postgres;

--
-- Name: pledge_status; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.pledge_status AS ENUM (
    'pending',
    'approved',
    'declined'
);


ALTER TYPE public.pledge_status OWNER TO postgres;

--
-- Name: profile_type; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.profile_type AS ENUM (
    'individual',
    'organization',
    'company'
);


ALTER TYPE public.profile_type OWNER TO postgres;

--
-- Name: transaction_status; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.transaction_status AS ENUM (
    'inprogress',
    'completed',
    'cancelled'
);


ALTER TYPE public.transaction_status OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: message; Type: TABLE; Schema: interaction; Owner: postgres
--

CREATE TABLE interaction.message (
    id bigint NOT NULL,
    id_sender bigint,
    id_transaction bigint,
    type public.message_type,
    content text NOT NULL,
    sent_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone,
    CONSTRAINT chk_content_length CHECK ((char_length(content) <= 10000))
);


ALTER TABLE interaction.message OWNER TO postgres;

--
-- Name: message_id_seq; Type: SEQUENCE; Schema: interaction; Owner: postgres
--

ALTER TABLE interaction.message ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME interaction.message_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: pledge; Type: TABLE; Schema: interaction; Owner: postgres
--

CREATE TABLE interaction.pledge (
    id bigint NOT NULL,
    id_profile bigint,
    id_item bigint,
    intent_action public.pledge_intent_action NOT NULL,
    message text,
    status public.pledge_status DEFAULT 'pending'::public.pledge_status NOT NULL,
    pledged_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone,
    updated_by uuid,
    CONSTRAINT chk_message_length CHECK ((char_length(message) <= 10000))
);


ALTER TABLE interaction.pledge OWNER TO postgres;

--
-- Name: pledge_id_seq; Type: SEQUENCE; Schema: interaction; Owner: postgres
--

ALTER TABLE interaction.pledge ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME interaction.pledge_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: review; Type: TABLE; Schema: interaction; Owner: postgres
--

CREATE TABLE interaction.review (
    id_transaction bigint NOT NULL,
    id_subject_profile bigint NOT NULL,
    reviewer bigint,
    comment text,
    score smallint DEFAULT 0 NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT chk_comment_length CHECK ((char_length(comment) <= 10000))
);


ALTER TABLE interaction.review OWNER TO postgres;

--
-- Name: schedule; Type: TABLE; Schema: interaction; Owner: postgres
--

CREATE TABLE interaction.schedule (
    id bigint NOT NULL,
    scheduled_for timestamp with time zone NOT NULL
);


ALTER TABLE interaction.schedule OWNER TO postgres;

--
-- Name: schedule_id_seq; Type: SEQUENCE; Schema: interaction; Owner: postgres
--

ALTER TABLE interaction.schedule ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME interaction.schedule_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: schedule_opportunity; Type: TABLE; Schema: interaction; Owner: postgres
--

CREATE TABLE interaction.schedule_opportunity (
    id bigint,
    window_start timestamp with time zone,
    window_end timestamp with time zone
);


ALTER TABLE interaction.schedule_opportunity OWNER TO postgres;

--
-- Name: transaction; Type: TABLE; Schema: interaction; Owner: postgres
--

CREATE TABLE interaction.transaction (
    id bigint NOT NULL,
    id_pledge bigint,
    status public.transaction_status DEFAULT 'inprogress'::public.transaction_status NOT NULL,
    id_schedule bigint,
    id_location bigint,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone
);


ALTER TABLE interaction.transaction OWNER TO postgres;

--
-- Name: transaction_id_seq; Type: SEQUENCE; Schema: interaction; Owner: postgres
--

ALTER TABLE interaction.transaction ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME interaction.transaction_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: category; Type: TABLE; Schema: listing; Owner: postgres
--

CREATE TABLE listing.category (
    id bigint NOT NULL,
    title character varying(150) NOT NULL,
    description text,
    category_parent bigint,
    CONSTRAINT chk_description_length CHECK ((char_length(description) <= 10000)),
    CONSTRAINT chk_no_self_reference CHECK ((id <> category_parent))
);


ALTER TABLE listing.category OWNER TO postgres;

--
-- Name: category_id_seq; Type: SEQUENCE; Schema: listing; Owner: postgres
--

ALTER TABLE listing.category ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME listing.category_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: collection; Type: TABLE; Schema: listing; Owner: postgres
--

CREATE TABLE listing.collection (
    id bigint NOT NULL,
    id_community bigint,
    title character varying(150),
    visibility public.collection_visibility NOT NULL,
    type public.collection_type,
    "position" integer DEFAULT 0 NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone
);


ALTER TABLE listing.collection OWNER TO postgres;

--
-- Name: collection_id_seq; Type: SEQUENCE; Schema: listing; Owner: postgres
--

ALTER TABLE listing.collection ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME listing.collection_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: item; Type: TABLE; Schema: listing; Owner: postgres
--

CREATE TABLE listing.item (
    id bigint NOT NULL,
    type public.item_type NOT NULL,
    intent_action public.item_intent_action NOT NULL,
    status public.item_status DEFAULT 'draft'::public.item_status NOT NULL,
    title character varying(150),
    description text,
    category bigint,
    condition public.item_condition NOT NULL,
    location bigint,
    views_count bigint DEFAULT 0 NOT NULL,
    is_reported boolean DEFAULT false NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone,
    created_by uuid,
    CONSTRAINT chk_description_length CHECK ((char_length(description) <= 10000))
);


ALTER TABLE listing.item OWNER TO postgres;

--
-- Name: item_id_seq; Type: SEQUENCE; Schema: listing; Owner: postgres
--

ALTER TABLE listing.item ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME listing.item_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: location; Type: TABLE; Schema: listing; Owner: postgres
--

CREATE TABLE listing.location (
    id bigint NOT NULL,
    address_line1 character varying(64) NOT NULL,
    address_line2 character varying(64),
    city character varying(50) NOT NULL,
    state character varying(50),
    district bigint,
    country character varying(50) NOT NULL,
    geom extension.geography(Point,4326),
    entrance_note text,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT chk_entrance_note_length CHECK ((char_length(entrance_note) <= 10000))
);


ALTER TABLE listing.location OWNER TO postgres;

--
-- Name: location_id_seq; Type: SEQUENCE; Schema: listing; Owner: postgres
--

ALTER TABLE listing.location ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME listing.location_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: media; Type: TABLE; Schema: listing; Owner: postgres
--

CREATE TABLE listing.media (
    id bigint NOT NULL,
    id_item bigint,
    caption character varying(150),
    url character varying(2048) NOT NULL,
    type public.media_type NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE listing.media OWNER TO postgres;

--
-- Name: media_id_seq; Type: SEQUENCE; Schema: listing; Owner: postgres
--

ALTER TABLE listing.media ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME listing.media_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: publish; Type: TABLE; Schema: listing; Owner: postgres
--

CREATE TABLE listing.publish (
    id_item bigint NOT NULL,
    id_collection bigint NOT NULL,
    note text,
    "position" integer DEFAULT 0 NOT NULL,
    added_by uuid,
    posted_on timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE listing.publish OWNER TO postgres;

--
-- Name: test; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.test (
    i integer
);


ALTER TABLE public.test OWNER TO postgres;

--
-- Name: account; Type: TABLE; Schema: user; Owner: postgres
--

CREATE TABLE "user".account (
    id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE "user".account OWNER TO postgres;

--
-- Name: committee; Type: TABLE; Schema: user; Owner: postgres
--

CREATE TABLE "user".committee (
    id_profile bigint NOT NULL,
    id_community bigint NOT NULL,
    member_role public.committee_role NOT NULL,
    joined_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE "user".committee OWNER TO postgres;

--
-- Name: community; Type: TABLE; Schema: user; Owner: postgres
--

CREATE TABLE "user".community (
    id bigint NOT NULL,
    title character varying(150),
    description text,
    type public.community_type DEFAULT 'solo'::public.community_type NOT NULL,
    owner uuid NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone,
    created_by uuid NOT NULL,
    CONSTRAINT chk_description_length CHECK ((char_length(description) <= 10000))
);


ALTER TABLE "user".community OWNER TO postgres;

--
-- Name: community_id_seq; Type: SEQUENCE; Schema: user; Owner: postgres
--

ALTER TABLE "user".community ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME "user".community_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: profile; Type: TABLE; Schema: user; Owner: postgres
--

CREATE TABLE "user".profile (
    id bigint NOT NULL,
    name character varying(100),
    description text,
    type public.profile_type,
    owner uuid NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone,
    created_by uuid NOT NULL,
    CONSTRAINT chk_description_length CHECK ((char_length(description) <= 10000))
);


ALTER TABLE "user".profile OWNER TO postgres;

--
-- Name: profile_id_seq; Type: SEQUENCE; Schema: user; Owner: postgres
--

ALTER TABLE "user".profile ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME "user".profile_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: message message_pkey; Type: CONSTRAINT; Schema: interaction; Owner: postgres
--

ALTER TABLE ONLY interaction.message
    ADD CONSTRAINT message_pkey PRIMARY KEY (id);


--
-- Name: pledge pledge_pkey; Type: CONSTRAINT; Schema: interaction; Owner: postgres
--

ALTER TABLE ONLY interaction.pledge
    ADD CONSTRAINT pledge_pkey PRIMARY KEY (id);


--
-- Name: review review_pkey; Type: CONSTRAINT; Schema: interaction; Owner: postgres
--

ALTER TABLE ONLY interaction.review
    ADD CONSTRAINT review_pkey PRIMARY KEY (id_transaction, id_subject_profile);


--
-- Name: schedule schedule_pkey; Type: CONSTRAINT; Schema: interaction; Owner: postgres
--

ALTER TABLE ONLY interaction.schedule
    ADD CONSTRAINT schedule_pkey PRIMARY KEY (id);


--
-- Name: transaction transaction_pkey; Type: CONSTRAINT; Schema: interaction; Owner: postgres
--

ALTER TABLE ONLY interaction.transaction
    ADD CONSTRAINT transaction_pkey PRIMARY KEY (id);


--
-- Name: category category_pkey; Type: CONSTRAINT; Schema: listing; Owner: postgres
--

ALTER TABLE ONLY listing.category
    ADD CONSTRAINT category_pkey PRIMARY KEY (id);


--
-- Name: collection collection_pkey; Type: CONSTRAINT; Schema: listing; Owner: postgres
--

ALTER TABLE ONLY listing.collection
    ADD CONSTRAINT collection_pkey PRIMARY KEY (id);


--
-- Name: item item_pkey; Type: CONSTRAINT; Schema: listing; Owner: postgres
--

ALTER TABLE ONLY listing.item
    ADD CONSTRAINT item_pkey PRIMARY KEY (id);


--
-- Name: location location_pkey; Type: CONSTRAINT; Schema: listing; Owner: postgres
--

ALTER TABLE ONLY listing.location
    ADD CONSTRAINT location_pkey PRIMARY KEY (id);


--
-- Name: media media_pkey; Type: CONSTRAINT; Schema: listing; Owner: postgres
--

ALTER TABLE ONLY listing.media
    ADD CONSTRAINT media_pkey PRIMARY KEY (id);


--
-- Name: publish publish_pkey; Type: CONSTRAINT; Schema: listing; Owner: postgres
--

ALTER TABLE ONLY listing.publish
    ADD CONSTRAINT publish_pkey PRIMARY KEY (id_item, id_collection);


--
-- Name: account account_pkey; Type: CONSTRAINT; Schema: user; Owner: postgres
--

ALTER TABLE ONLY "user".account
    ADD CONSTRAINT account_pkey PRIMARY KEY (id);


--
-- Name: committee committee_pkey; Type: CONSTRAINT; Schema: user; Owner: postgres
--

ALTER TABLE ONLY "user".committee
    ADD CONSTRAINT committee_pkey PRIMARY KEY (id_profile, id_community);


--
-- Name: community community_pkey; Type: CONSTRAINT; Schema: user; Owner: postgres
--

ALTER TABLE ONLY "user".community
    ADD CONSTRAINT community_pkey PRIMARY KEY (id);


--
-- Name: profile profile_pkey; Type: CONSTRAINT; Schema: user; Owner: postgres
--

ALTER TABLE ONLY "user".profile
    ADD CONSTRAINT profile_pkey PRIMARY KEY (id);


--
-- Name: message message_id_sender_fkey; Type: FK CONSTRAINT; Schema: interaction; Owner: postgres
--

ALTER TABLE ONLY interaction.message
    ADD CONSTRAINT message_id_sender_fkey FOREIGN KEY (id_sender) REFERENCES "user".profile(id) ON DELETE SET NULL;


--
-- Name: message message_id_transaction_fkey; Type: FK CONSTRAINT; Schema: interaction; Owner: postgres
--

ALTER TABLE ONLY interaction.message
    ADD CONSTRAINT message_id_transaction_fkey FOREIGN KEY (id_transaction) REFERENCES interaction.transaction(id) ON DELETE CASCADE;


--
-- Name: pledge pledge_id_item_fkey; Type: FK CONSTRAINT; Schema: interaction; Owner: postgres
--

ALTER TABLE ONLY interaction.pledge
    ADD CONSTRAINT pledge_id_item_fkey FOREIGN KEY (id_item) REFERENCES listing.item(id) ON DELETE CASCADE;


--
-- Name: pledge pledge_id_profile_fkey; Type: FK CONSTRAINT; Schema: interaction; Owner: postgres
--

ALTER TABLE ONLY interaction.pledge
    ADD CONSTRAINT pledge_id_profile_fkey FOREIGN KEY (id_profile) REFERENCES "user".profile(id) ON DELETE CASCADE;


--
-- Name: pledge pledge_updated_by_fkey; Type: FK CONSTRAINT; Schema: interaction; Owner: postgres
--

ALTER TABLE ONLY interaction.pledge
    ADD CONSTRAINT pledge_updated_by_fkey FOREIGN KEY (updated_by) REFERENCES "user".account(id) ON DELETE SET NULL;


--
-- Name: review review_id_subject_profile_fkey; Type: FK CONSTRAINT; Schema: interaction; Owner: postgres
--

ALTER TABLE ONLY interaction.review
    ADD CONSTRAINT review_id_subject_profile_fkey FOREIGN KEY (id_subject_profile) REFERENCES "user".profile(id) ON DELETE SET NULL;


--
-- Name: review review_id_transaction_fkey; Type: FK CONSTRAINT; Schema: interaction; Owner: postgres
--

ALTER TABLE ONLY interaction.review
    ADD CONSTRAINT review_id_transaction_fkey FOREIGN KEY (id_transaction) REFERENCES interaction.transaction(id) ON DELETE SET NULL;


--
-- Name: review review_reviewer_fkey; Type: FK CONSTRAINT; Schema: interaction; Owner: postgres
--

ALTER TABLE ONLY interaction.review
    ADD CONSTRAINT review_reviewer_fkey FOREIGN KEY (reviewer) REFERENCES "user".profile(id);


--
-- Name: schedule_opportunity schedule_opportunity_id_fkey; Type: FK CONSTRAINT; Schema: interaction; Owner: postgres
--

ALTER TABLE ONLY interaction.schedule_opportunity
    ADD CONSTRAINT schedule_opportunity_id_fkey FOREIGN KEY (id) REFERENCES interaction.message(id);


--
-- Name: transaction transaction_id_location_fkey; Type: FK CONSTRAINT; Schema: interaction; Owner: postgres
--

ALTER TABLE ONLY interaction.transaction
    ADD CONSTRAINT transaction_id_location_fkey FOREIGN KEY (id_location) REFERENCES listing.location(id) ON DELETE SET NULL;


--
-- Name: transaction transaction_id_pledge_fkey; Type: FK CONSTRAINT; Schema: interaction; Owner: postgres
--

ALTER TABLE ONLY interaction.transaction
    ADD CONSTRAINT transaction_id_pledge_fkey FOREIGN KEY (id_pledge) REFERENCES interaction.pledge(id) ON DELETE CASCADE;


--
-- Name: transaction transaction_id_schedule_fkey; Type: FK CONSTRAINT; Schema: interaction; Owner: postgres
--

ALTER TABLE ONLY interaction.transaction
    ADD CONSTRAINT transaction_id_schedule_fkey FOREIGN KEY (id_schedule) REFERENCES interaction.schedule(id) ON DELETE SET NULL;


--
-- Name: category category_category_parent_fkey; Type: FK CONSTRAINT; Schema: listing; Owner: postgres
--

ALTER TABLE ONLY listing.category
    ADD CONSTRAINT category_category_parent_fkey FOREIGN KEY (category_parent) REFERENCES listing.category(id) ON DELETE SET NULL DEFERRABLE INITIALLY DEFERRED;


--
-- Name: collection collection_id_community_fkey; Type: FK CONSTRAINT; Schema: listing; Owner: postgres
--

ALTER TABLE ONLY listing.collection
    ADD CONSTRAINT collection_id_community_fkey FOREIGN KEY (id_community) REFERENCES "user".community(id) ON DELETE SET NULL;


--
-- Name: item item_category_fkey; Type: FK CONSTRAINT; Schema: listing; Owner: postgres
--

ALTER TABLE ONLY listing.item
    ADD CONSTRAINT item_category_fkey FOREIGN KEY (category) REFERENCES listing.category(id) ON DELETE SET NULL DEFERRABLE INITIALLY DEFERRED;


--
-- Name: item item_created_by_fkey; Type: FK CONSTRAINT; Schema: listing; Owner: postgres
--

ALTER TABLE ONLY listing.item
    ADD CONSTRAINT item_created_by_fkey FOREIGN KEY (created_by) REFERENCES "user".account(id) ON DELETE SET NULL DEFERRABLE INITIALLY DEFERRED;


--
-- Name: item item_location_fkey; Type: FK CONSTRAINT; Schema: listing; Owner: postgres
--

ALTER TABLE ONLY listing.item
    ADD CONSTRAINT item_location_fkey FOREIGN KEY (location) REFERENCES listing.location(id) ON DELETE SET NULL DEFERRABLE INITIALLY DEFERRED;


--
-- Name: media media_id_item_fkey; Type: FK CONSTRAINT; Schema: listing; Owner: postgres
--

ALTER TABLE ONLY listing.media
    ADD CONSTRAINT media_id_item_fkey FOREIGN KEY (id_item) REFERENCES listing.item(id) ON DELETE CASCADE;


--
-- Name: publish publish_added_by_fkey; Type: FK CONSTRAINT; Schema: listing; Owner: postgres
--

ALTER TABLE ONLY listing.publish
    ADD CONSTRAINT publish_added_by_fkey FOREIGN KEY (added_by) REFERENCES "user".account(id) ON DELETE SET NULL;


--
-- Name: publish publish_id_collection_fkey; Type: FK CONSTRAINT; Schema: listing; Owner: postgres
--

ALTER TABLE ONLY listing.publish
    ADD CONSTRAINT publish_id_collection_fkey FOREIGN KEY (id_collection) REFERENCES listing.collection(id) ON DELETE CASCADE;


--
-- Name: publish publish_id_item_fkey; Type: FK CONSTRAINT; Schema: listing; Owner: postgres
--

ALTER TABLE ONLY listing.publish
    ADD CONSTRAINT publish_id_item_fkey FOREIGN KEY (id_item) REFERENCES listing.item(id) ON DELETE CASCADE;


--
-- Name: committee committee_id_community_fkey; Type: FK CONSTRAINT; Schema: user; Owner: postgres
--

ALTER TABLE ONLY "user".committee
    ADD CONSTRAINT committee_id_community_fkey FOREIGN KEY (id_community) REFERENCES "user".community(id);


--
-- Name: committee committee_id_profile_fkey; Type: FK CONSTRAINT; Schema: user; Owner: postgres
--

ALTER TABLE ONLY "user".committee
    ADD CONSTRAINT committee_id_profile_fkey FOREIGN KEY (id_profile) REFERENCES "user".profile(id);


--
-- Name: community community_created_by_fkey; Type: FK CONSTRAINT; Schema: user; Owner: postgres
--

ALTER TABLE ONLY "user".community
    ADD CONSTRAINT community_created_by_fkey FOREIGN KEY (created_by) REFERENCES "user".account(id);


--
-- Name: community community_owner_fkey; Type: FK CONSTRAINT; Schema: user; Owner: postgres
--

ALTER TABLE ONLY "user".community
    ADD CONSTRAINT community_owner_fkey FOREIGN KEY (owner) REFERENCES "user".account(id);


--
-- Name: profile profile_created_by_fkey; Type: FK CONSTRAINT; Schema: user; Owner: postgres
--

ALTER TABLE ONLY "user".profile
    ADD CONSTRAINT profile_created_by_fkey FOREIGN KEY (created_by) REFERENCES "user".account(id);


--
-- Name: profile profile_owner_fkey; Type: FK CONSTRAINT; Schema: user; Owner: postgres
--

ALTER TABLE ONLY "user".profile
    ADD CONSTRAINT profile_owner_fkey FOREIGN KEY (owner) REFERENCES "user".account(id);


--
-- Name: TABLE message; Type: ACL; Schema: interaction; Owner: postgres
--

GRANT ALL ON TABLE interaction.message TO "postgres-user";


--
-- Name: TABLE pledge; Type: ACL; Schema: interaction; Owner: postgres
--

GRANT ALL ON TABLE interaction.pledge TO "postgres-user";


--
-- Name: TABLE review; Type: ACL; Schema: interaction; Owner: postgres
--

GRANT ALL ON TABLE interaction.review TO "postgres-user";


--
-- Name: TABLE schedule; Type: ACL; Schema: interaction; Owner: postgres
--

GRANT ALL ON TABLE interaction.schedule TO "postgres-user";


--
-- Name: TABLE schedule_opportunity; Type: ACL; Schema: interaction; Owner: postgres
--

GRANT ALL ON TABLE interaction.schedule_opportunity TO "postgres-user";


--
-- Name: TABLE transaction; Type: ACL; Schema: interaction; Owner: postgres
--

GRANT ALL ON TABLE interaction.transaction TO "postgres-user";


--
-- Name: TABLE category; Type: ACL; Schema: listing; Owner: postgres
--

GRANT ALL ON TABLE listing.category TO "postgres-user";


--
-- Name: TABLE collection; Type: ACL; Schema: listing; Owner: postgres
--

GRANT ALL ON TABLE listing.collection TO "postgres-user";


--
-- Name: TABLE item; Type: ACL; Schema: listing; Owner: postgres
--

GRANT ALL ON TABLE listing.item TO "postgres-user";


--
-- Name: TABLE location; Type: ACL; Schema: listing; Owner: postgres
--

GRANT ALL ON TABLE listing.location TO "postgres-user";


--
-- Name: TABLE media; Type: ACL; Schema: listing; Owner: postgres
--

GRANT ALL ON TABLE listing.media TO "postgres-user";


--
-- Name: TABLE publish; Type: ACL; Schema: listing; Owner: postgres
--

GRANT ALL ON TABLE listing.publish TO "postgres-user";


--
-- Name: TABLE test; Type: ACL; Schema: public; Owner: postgres
--

GRANT ALL ON TABLE public.test TO "postgres-user";


--
-- Name: TABLE account; Type: ACL; Schema: user; Owner: postgres
--

GRANT ALL ON TABLE "user".account TO "postgres-user";


--
-- Name: TABLE committee; Type: ACL; Schema: user; Owner: postgres
--

GRANT ALL ON TABLE "user".committee TO "postgres-user";


--
-- Name: TABLE community; Type: ACL; Schema: user; Owner: postgres
--

GRANT ALL ON TABLE "user".community TO "postgres-user";


--
-- Name: TABLE profile; Type: ACL; Schema: user; Owner: postgres
--

GRANT ALL ON TABLE "user".profile TO "postgres-user";


--
-- Name: DEFAULT PRIVILEGES FOR TABLES; Type: DEFAULT ACL; Schema: extension; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA extension GRANT ALL ON TABLES TO "postgres-user";


--
-- Name: DEFAULT PRIVILEGES FOR TABLES; Type: DEFAULT ACL; Schema: interaction; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA interaction GRANT ALL ON TABLES TO "postgres-user";


--
-- Name: DEFAULT PRIVILEGES FOR TABLES; Type: DEFAULT ACL; Schema: listing; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA listing GRANT ALL ON TABLES TO "postgres-user";


--
-- Name: DEFAULT PRIVILEGES FOR TABLES; Type: DEFAULT ACL; Schema: user; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA "user" GRANT ALL ON TABLES TO "postgres-user";


--
-- PostgreSQL database dump complete
--

