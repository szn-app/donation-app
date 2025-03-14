# https://supabase.com/docs/guides/self-hosting
deploy_api_data_supabase() {
    # postgres with extensions 
    https://hub.docker.com/r/supabase/postgres

    helm repo add supabase https://supabase.github.io/helm-charts
    helm show values supabase/supabase
    
    # components: postgres, realtime, storage
    helm install supabase supabase/supabase -f values.yaml --namespace supabase

    # S3 storage class 

    helm upgrade --install supabase supabase/supabase -f values.yaml --namespace supabase

    verify() {
        # use to test setup 
        # https://github.com/supabase-community/supabase-kubernetes/blob/main/charts/supabase/README.md#testing-suite
        echo ''
    }
}

deploy_postgres_stackgres_instance() { 
    # create stackgres crd 'SGCluster' instance with Supabase compatible configuration:
    # https://stackgres.io/doc/latest/reference/crd/sgcluster/
    # https://stackgres.io/doc/latest/runbooks/supabase-stackgres/
    # https://github.com/ongres/apps-on-stackgres/tree/main/examples/supabase
    # https://stackgres.io/blog/running-supabase-on-top-of-stackgres/
    # SQL scripts based on https://github.com/supabase-community/supabase-kubernetes

    # image based on modified https://hub.docker.com/_/postgres by supabase https://hub.docker.com/r/supabase/postgres
    local t=$(mktemp) && cat <<'EOF' > "$t"

# pool config
apiVersion: stackgres.io/v1
kind: SGPoolingConfig
metadata:
  name: supabase-db
spec:
  pgBouncer:
    pgbouncer.ini:
      pgbouncer:
        ignore_startup_parameters: extra_float_digits,search_path
---
# sql init script for supabase
apiVersion: stackgres.io/v1
kind: SGScript
metadata:
  name: supabase-initdb
spec:
  scripts:
  - name: 00-reset-auth
    script: |
      drop role authenticator;
  - name: 01-initial-schema
    script: |
      -- Set up realtime
      create schema if not exists realtime;
      -- create publication supabase_realtime; -- defaults to empty publication
      create publication supabase_realtime;
      create schema if not exists _realtime;
      create schema if not exists realtime;

      -- Supabase super admin
      create user supabase_admin;
      alter user  supabase_admin with superuser createdb createrole replication bypassrls;

      -- Extension namespacing
      create schema if not exists extensions;
      create extension if not exists "uuid-ossp"      with schema extensions;
      create extension if not exists pgcrypto         with schema extensions;
      create extension if not exists pgjwt            with schema extensions;

      -- Set up auth roles for the developer
      create role anon                nologin noinherit;
      create role authenticated       nologin noinherit; -- "logged in" user: web_user, app_user, etc
      create role service_role        nologin noinherit bypassrls; -- allow developers to create JWT's that bypass their policies

      create user authenticator noinherit;
      grant anon              to authenticator;
      grant authenticated     to authenticator;
      grant service_role      to authenticator;
      grant supabase_admin    to authenticator;

      grant usage                     on schema public to postgres, anon, authenticated, service_role;
      alter default privileges in schema public grant all on tables to postgres, anon, authenticated, service_role;
      alter default privileges in schema public grant all on functions to postgres, anon, authenticated, service_role;
      alter default privileges in schema public grant all on sequences to postgres, anon, authenticated, service_role;

      -- Allow Extensions to be used in the API
      grant usage                     on schema extensions to postgres, anon, authenticated, service_role;

      -- Set up namespacing
      alter user supabase_admin SET search_path TO public, extensions; -- don't include the "auth" schema

      -- These are required so that the users receive grants whenever "supabase_admin" creates tables/function
      alter default privileges for user supabase_admin in schema public grant all
          on sequences to postgres, anon, authenticated, service_role;
      alter default privileges for user supabase_admin in schema public grant all
          on tables to postgres, anon, authenticated, service_role;
      alter default privileges for user supabase_admin in schema public grant all
          on functions to postgres, anon, authenticated, service_role;

      -- Set short statement/query timeouts for API roles
      alter role anon set statement_timeout = '3s';
      alter role authenticated set statement_timeout = '8s';
  - name: 02-auth-schema
    script: |
      CREATE SCHEMA IF NOT EXISTS auth AUTHORIZATION supabase_admin;

      -- auth.users definition

      CREATE TABLE auth.users (
      instance_id uuid NULL,
      id uuid NOT NULL UNIQUE,
      aud varchar(255) NULL,
      "role" varchar(255) NULL,
      email varchar(255) NULL UNIQUE,
      encrypted_password varchar(255) NULL,
      confirmed_at timestamptz NULL,
      invited_at timestamptz NULL,
      confirmation_token varchar(255) NULL,
      confirmation_sent_at timestamptz NULL,
      recovery_token varchar(255) NULL,
      recovery_sent_at timestamptz NULL,
      email_change_token varchar(255) NULL,
      email_change varchar(255) NULL,
      email_change_sent_at timestamptz NULL,
      last_sign_in_at timestamptz NULL,
      raw_app_meta_data jsonb NULL,
      raw_user_meta_data jsonb NULL,
      is_super_admin bool NULL,
      created_at timestamptz NULL,
      updated_at timestamptz NULL,
      CONSTRAINT users_pkey PRIMARY KEY (id)
      );
      CREATE INDEX users_instance_id_email_idx ON auth.users USING btree (instance_id, email);
      CREATE INDEX users_instance_id_idx ON auth.users USING btree (instance_id);
      comment on table auth.users is 'Auth: Stores user login data within a secure schema.';

      -- auth.refresh_tokens definition

      CREATE TABLE auth.refresh_tokens (
      instance_id uuid NULL,
      id bigserial NOT NULL,
      "token" varchar(255) NULL,
      user_id varchar(255) NULL,
      revoked bool NULL,
      created_at timestamptz NULL,
      updated_at timestamptz NULL,
      CONSTRAINT refresh_tokens_pkey PRIMARY KEY (id)
      );
      CREATE INDEX refresh_tokens_instance_id_idx ON auth.refresh_tokens USING btree (instance_id);
      CREATE INDEX refresh_tokens_instance_id_user_id_idx ON auth.refresh_tokens USING btree (instance_id, user_id);
      CREATE INDEX refresh_tokens_token_idx ON auth.refresh_tokens USING btree (token);
      comment on table auth.refresh_tokens is 'Auth: Store of tokens used to refresh JWT tokens once they expire.';

      -- auth.instances definition

      CREATE TABLE auth.instances (
      id uuid NOT NULL,
      uuid uuid NULL,
      raw_base_config text NULL,
      created_at timestamptz NULL,
      updated_at timestamptz NULL,
      CONSTRAINT instances_pkey PRIMARY KEY (id)
      );
      comment on table auth.instances is 'Auth: Manages users across multiple sites.';

      -- auth.audit_log_entries definition

      CREATE TABLE auth.audit_log_entries (
      instance_id uuid NULL,
      id uuid NOT NULL,
      payload json NULL,
      created_at timestamptz NULL,
      CONSTRAINT audit_log_entries_pkey PRIMARY KEY (id)
      );
      CREATE INDEX audit_logs_instance_id_idx ON auth.audit_log_entries USING btree (instance_id);
      comment on table auth.audit_log_entries is 'Auth: Audit trail for user actions.';

      -- auth.schema_migrations definition

      CREATE TABLE auth.schema_migrations (
      "version" varchar(255) NOT NULL,
      CONSTRAINT schema_migrations_pkey PRIMARY KEY ("version")
      );
      comment on table auth.schema_migrations is 'Auth: Manages updates to the auth system.';

      INSERT INTO auth.schema_migrations (version)
      VALUES  ('20171026211738'),
              ('20171026211808'),
              ('20171026211834'),
              ('20180103212743'),
              ('20180108183307'),
              ('20180119214651'),
              ('20180125194653');

      create or replace function auth.uid() 
      returns uuid 
      language sql stable
      as $$
      select 
          coalesce(
          current_setting('request.jwt.claim.sub', true),
          (current_setting('request.jwt.claims', true)::jsonb ->> 'sub')
      )::uuid
      $$;

      create or replace function auth.role() 
      returns text 
      language sql stable
      as $$
      select 
          coalesce(
          current_setting('request.jwt.claim.role', true),
          (current_setting('request.jwt.claims', true)::jsonb ->> 'role')
      )::text
      $$;

      create or replace function auth.email() 
      returns text 
      language sql stable
      as $$
      select 
          coalesce(
          current_setting('request.jwt.claim.email', true),
          (current_setting('request.jwt.claims', true)::jsonb ->> 'email')
      )::text
      $$;

      -- usage on auth functions to API roles
      GRANT USAGE ON SCHEMA auth TO anon, authenticated, service_role;

      -- Supabase super admin
      CREATE USER supabase_auth_admin NOINHERIT CREATEROLE LOGIN NOREPLICATION;
      GRANT ALL PRIVILEGES ON SCHEMA auth TO supabase_auth_admin;
      GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA auth TO supabase_auth_admin;
      GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA auth TO supabase_auth_admin;
      ALTER USER supabase_auth_admin SET search_path = "auth";
      ALTER table "auth".users OWNER TO supabase_auth_admin;
      ALTER table "auth".refresh_tokens OWNER TO supabase_auth_admin;
      ALTER table "auth".audit_log_entries OWNER TO supabase_auth_admin;
      ALTER table "auth".instances OWNER TO supabase_auth_admin;
      ALTER table "auth".schema_migrations OWNER TO supabase_auth_admin;

      ALTER FUNCTION "auth"."uid" OWNER TO supabase_auth_admin;
      ALTER FUNCTION "auth"."role" OWNER TO supabase_auth_admin;
      ALTER FUNCTION "auth"."email" OWNER TO supabase_auth_admin;
      GRANT EXECUTE ON FUNCTION "auth"."uid"() TO PUBLIC;
      GRANT EXECUTE ON FUNCTION "auth"."role"() TO PUBLIC;
      GRANT EXECUTE ON FUNCTION "auth"."email"() TO PUBLIC;
  - name: 03-storage-schema
    script: |
      CREATE SCHEMA IF NOT EXISTS storage AUTHORIZATION supabase_admin;

      grant usage on schema storage to postgres, anon, authenticated, service_role;
      alter default privileges in schema storage grant all on tables to postgres, anon, authenticated, service_role;
      alter default privileges in schema storage grant all on functions to postgres, anon, authenticated, service_role;
      alter default privileges in schema storage grant all on sequences to postgres, anon, authenticated, service_role;

      CREATE TABLE "storage"."buckets" (
          "id" text not NULL,
          "name" text NOT NULL,
          "owner" uuid,
          "created_at" timestamptz DEFAULT now(),
          "updated_at" timestamptz DEFAULT now(),
          CONSTRAINT "buckets_owner_fkey" FOREIGN KEY ("owner") REFERENCES "auth"."users"("id"),
          PRIMARY KEY ("id")
      );
      CREATE UNIQUE INDEX "bname" ON "storage"."buckets" USING BTREE ("name");

      CREATE TABLE "storage"."objects" (
          "id" uuid NOT NULL DEFAULT extensions.uuid_generate_v4(),
          "bucket_id" text,
          "name" text,
          "owner" uuid,
          "created_at" timestamptz DEFAULT now(),
          "updated_at" timestamptz DEFAULT now(),
          "last_accessed_at" timestamptz DEFAULT now(),
          "metadata" jsonb,
          CONSTRAINT "objects_bucketId_fkey" FOREIGN KEY ("bucket_id") REFERENCES "storage"."buckets"("id"),
          CONSTRAINT "objects_owner_fkey" FOREIGN KEY ("owner") REFERENCES "auth"."users"("id"),
          PRIMARY KEY ("id")
      );
      CREATE UNIQUE INDEX "bucketid_objname" ON "storage"."objects" USING BTREE ("bucket_id","name");
      CREATE INDEX name_prefix_search ON storage.objects(name text_pattern_ops);

      ALTER TABLE storage.objects ENABLE ROW LEVEL SECURITY;

      CREATE FUNCTION storage.foldername(name text)
      RETURNS text[]
      LANGUAGE plpgsql
      AS $function$
      DECLARE
      _parts text[];
      BEGIN
      select string_to_array(name, '/') into _parts;
      return _parts[1:array_length(_parts,1)-1];
      END
      $function$;

      CREATE FUNCTION storage.filename(name text)
      RETURNS text
      LANGUAGE plpgsql
      AS $function$
      DECLARE
      _parts text[];
      BEGIN
      select string_to_array(name, '/') into _parts;
      return _parts[array_length(_parts,1)];
      END
      $function$;

      CREATE FUNCTION storage.extension(name text)
      RETURNS text
      LANGUAGE plpgsql
      AS $function$
      DECLARE
      _parts text[];
      _filename text;
      BEGIN
      select string_to_array(name, '/') into _parts;
      select _parts[array_length(_parts,1)] into _filename;
      -- @todo return the last part instead of 2
      return split_part(_filename, '.', 2);
      END
      $function$;

      CREATE FUNCTION storage.search(prefix text, bucketname text, limits int DEFAULT 100, levels int DEFAULT 1, offsets int DEFAULT 0)
      RETURNS TABLE (
          name text,
          id uuid,
          updated_at TIMESTAMPTZ,
          created_at TIMESTAMPTZ,
          last_accessed_at TIMESTAMPTZ,
          metadata jsonb
      )
      LANGUAGE plpgsql
      AS $function$
      DECLARE
      _bucketId text;
      BEGIN
          -- will be replaced by migrations when server starts
          -- saving space for cloud-init
      END
      $function$;

      -- create migrations table
      -- https://github.com/ThomWright/postgres-migrations/blob/master/src/migrations/0_create-migrations-table.sql
      -- we add this table here and not let it be auto-created so that the permissions are properly applied to it
      CREATE TABLE IF NOT EXISTS storage.migrations (
      id integer PRIMARY KEY,
      name varchar(100) UNIQUE NOT NULL,
      hash varchar(40) NOT NULL, -- sha1 hex encoded hash of the file name and contents, to ensure it hasn't been altered since applying the migration
      executed_at timestamp DEFAULT current_timestamp
      );

      CREATE USER supabase_storage_admin NOINHERIT CREATEROLE LOGIN NOREPLICATION;
      GRANT ALL PRIVILEGES ON SCHEMA storage TO supabase_storage_admin;
      GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA storage TO supabase_storage_admin;
      GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA storage TO supabase_storage_admin;
      ALTER USER supabase_storage_admin SET search_path = "storage";
      ALTER table "storage".objects owner to supabase_storage_admin;
      ALTER table "storage".buckets owner to supabase_storage_admin;
      ALTER table "storage".migrations OWNER TO supabase_storage_admin;
      ALTER function "storage".foldername(text) owner to supabase_storage_admin;
      ALTER function "storage".filename(text) owner to supabase_storage_admin;
      ALTER function "storage".extension(text) owner to supabase_storage_admin;
      ALTER function "storage".search(text,text,int,int,int) owner to supabase_storage_admin;
  - name: 04-post-setup
    script: |
      ALTER ROLE postgres SET search_path TO "\$user",public,extensions;
      CREATE OR REPLACE FUNCTION extensions.notify_api_restart()
      RETURNS event_trigger
      LANGUAGE plpgsql
      AS $$
      BEGIN
          NOTIFY ddl_command_end;
      END;
      $$;
      CREATE EVENT TRIGGER api_restart ON ddl_command_end
      EXECUTE PROCEDURE extensions.notify_api_restart();
      COMMENT ON FUNCTION extensions.notify_api_restart IS 'Sends a notification to the API to restart. If your database schema has changed, this is required so that Supabase can rebuild the relationships.';

      -- Trigger for pg_cron
      CREATE OR REPLACE FUNCTION extensions.grant_pg_cron_access()
      RETURNS event_trigger
      LANGUAGE plpgsql
      AS $$
      DECLARE
      schema_is_cron bool;
      BEGIN
      schema_is_cron = (
          SELECT n.nspname = 'cron'
          FROM pg_event_trigger_ddl_commands() AS ev
          LEFT JOIN pg_catalog.pg_namespace AS n
          ON ev.objid = n.oid
      );

      IF schema_is_cron
      THEN
          grant usage on schema cron to postgres with grant option;

          alter default privileges in schema cron grant all on tables to postgres with grant option;
          alter default privileges in schema cron grant all on functions to postgres with grant option;
          alter default privileges in schema cron grant all on sequences to postgres with grant option;

          alter default privileges for user supabase_admin in schema cron grant all
              on sequences to postgres with grant option;
          alter default privileges for user supabase_admin in schema cron grant all
              on tables to postgres with grant option;
          alter default privileges for user supabase_admin in schema cron grant all
              on functions to postgres with grant option;

          grant all privileges on all tables in schema cron to postgres with grant option; 

      END IF;

      END;
      $$;
      CREATE EVENT TRIGGER issue_pg_cron_access ON ddl_command_end WHEN TAG in ('CREATE SCHEMA')
      EXECUTE PROCEDURE extensions.grant_pg_cron_access();
      COMMENT ON FUNCTION extensions.grant_pg_cron_access IS 'Grants access to pg_cron';

      -- Supabase dashboard user
      CREATE ROLE dashboard_user NOSUPERUSER CREATEDB CREATEROLE REPLICATION;
      GRANT ALL ON DATABASE postgres TO dashboard_user;
      GRANT ALL ON SCHEMA auth TO dashboard_user;
      GRANT ALL ON SCHEMA extensions TO dashboard_user;
      GRANT ALL ON SCHEMA storage TO dashboard_user;
      GRANT ALL ON ALL TABLES IN SCHEMA auth TO dashboard_user;
      GRANT ALL ON ALL TABLES IN SCHEMA extensions TO dashboard_user;
      -- GRANT ALL ON ALL TABLES IN SCHEMA storage TO dashboard_user;
      GRANT ALL ON ALL SEQUENCES IN SCHEMA auth TO dashboard_user;
      GRANT ALL ON ALL SEQUENCES IN SCHEMA storage TO dashboard_user;
      GRANT ALL ON ALL SEQUENCES IN SCHEMA extensions TO dashboard_user;
      GRANT ALL ON ALL ROUTINES IN SCHEMA auth TO dashboard_user;
      GRANT ALL ON ALL ROUTINES IN SCHEMA storage TO dashboard_user;
      GRANT ALL ON ALL ROUTINES IN SCHEMA extensions TO dashboard_user;
  - name: 05-reset-auth
    script: |
      alter role authenticator inherit;
      alter role authenticator superuser;
---
# stackgres cluster
apiVersion: stackgres.io/v1
kind: SGCluster
metadata:
  name: supabase-db
spec:
  instances: 1
  pods:
    persistentVolume:
      size: '5Gi'
  # configurations:
  #   sgPoolingConfig: supabase-db
  postgres:
    version: 14
    # https://stackgres.io/doc/latest/intro/extensions/
    extensions:
      - name: pgsodium
        version: 3.1.9
      - name: pg_graphql
        version: 1.5.11
      - name: pg_stat_statements
      - name: pgcrypto
      - name: pgjwt
      - name: uuid-ossp
  # managedSql:
  #   scripts:
  #     - sgScript: supabase-initdb

EOF


  kubectl apply -f "$t"
  
  verify() {
    kubectl describe sgcluster supabase-db
    kubectl get pods

    kubectl delete sgcluster supabase-db

    {
      # password
      SUPERUSER_PASSWORD=$(kubectl get secret supabase-db --template '{{ printf "%s" (index .data "superuser-password" | base64decode) }}')
        
      # creates JWT secret
      kubectl -n default create secret generic demo-supabase-jwt \
        --from-literal=anonKey='eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ewogICAgInJvbGUiOiAiYW5vbiIsCiAgICAiaXNzIjogInN1cGFiYXNlIiwKICAgICJpYXQiOiAxNjc1NDAwNDAwLAogICAgImV4cCI6IDE4MzMxNjY4MDAKfQ.ztuiBzjaVoFHmoljUXWmnuDN6QU2WgJICeqwyzyZO88' \
        --from-literal=serviceKey='eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ewogICAgInJvbGUiOiAic2VydmljZV9yb2xlIiwKICAgICJpc3MiOiAic3VwYWJhc2UiLAogICAgImlhdCI6IDE2NzU0MDA0MDAsCiAgICAiZXhwIjogMTgzMzE2NjgwMAp9.qNsmXzz4tG7eqJPh1Y58DbtIlJBauwpqx39UF-MwM8k' \
        --from-literal=secret='abcdefghijklmnopqrstuvwxyz123456' \
              --dry-run=client -o yaml | kubectl apply -f <(sed '/metadata:/a\  annotations:\n    meta.helm.sh/release-name: demo\n    meta.helm.sh/release-namespace: default\n  labels:\n    app.kubernetes.io/managed-by: Helm' /dev/stdin)

      # creates SMTP secret
      kubectl -n default create secret generic demo-supabase-smtp \
        --from-literal=username='your-mail@example.com' \
        --from-literal=password='example123456' \
        --dry-run=client -o yaml | kubectl apply -f <(sed '/metadata:/a\  annotations:\n    meta.helm.sh/release-name: demo\n    meta.helm.sh/release-namespace: default\n  labels:\n    app.kubernetes.io/managed-by: Helm' /dev/stdin)

      # creates DB secret
      kubectl -n default create secret generic demo-supabase-db \
        --from-literal=username='postgres' \
        --from-literal=password="$SUPERUSER_PASSWORD" \
        --dry-run=client -o yaml | kubectl apply -f <(sed '/metadata:/a\  annotations:\n    meta.helm.sh/release-name: demo\n    meta.helm.sh/release-namespace: default\n  labels:\n    app.kubernetes.io/managed-by: Helm' /dev/stdin)

      helm install demo -f values.stackgres.yaml .
    } 
  }
}


d1() { 
    local t=$(mktemp) && cat <<'EOF' > "$t"
# stackgres cluster
apiVersion: stackgres.io/v1
kind: SGCluster
metadata:
  name: db-x
spec:
  instances: 1
  pods:
    persistentVolume:
      size: '5Gi'
  configurations:
    sgPoolingConfig: supabase-db
  postgres:
    version: '16'
    # https://stackgres.io/doc/latest/intro/extensions/
    extensions:
      - name: pgsodium
        version: 3.1.9
      - name: pg_graphql
        version: '1.5.11'
      - name: pg_stat_statements
      - name: pgcrypto
      - name: pgjwt
      - name: uuid-ossp
  # managedSql:
  #   scripts:
  #     - sgScript: supabase-initdb

EOF


  kubectl apply -f "$t"
}

d1_delete() {
  kubectl delete sgcluster db-x
}