
# https://github.com/cloudnative-pg/postgres-containers/blob/main/Dockerfile
# https://github.com/cloudnative-pg/postgres-containers/pkgs/container/postgresql/399084424?tag=17-bookworm
# https://github.com/cloudnative-pg/postgres-containers?tab=readme-ov-file#standard-images
# https://github.com/cloudnative-pg/postgres-containers/blob/main/Debian/ClusterImageCatalog-bookworm.yaml
# https://github.com/cloudnative-pg/postgres-containers/blob/main/Debian/17/bookworm/Dockerfile
# https://github.com/cloudnative-pg/postgres-containers/pkgs/container/postgresql/versions?filters%5Bversion_type%5D=tagged
# example config: https://github.com/voltade/cnpg-supabase/blob/main/Dockerfile
# NOTE: debian_version must match between container build stages (because trunk cli is dynamically linked)
ARG debian_version=bullseye
ARG postgresql_major=17
ARG postgresql_release=${postgresql_major}.3

# === Stage 1: Builder ===
FROM postgres:${postgresql_release}-${debian_version} AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl ca-certificates build-essential gcc libc6-dev pkg-config libssl-dev
# Install Rust and Cargo
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
# Install pg-trunk
RUN cargo install pg-trunk
RUN trunk --version && which trunk && sleep 100

# === Stage 2: Final image ===
# FROM ghcr.io/cloudnative-pg/postgresql:${postgresql_release}-standard-${debian_version} AS final
# https://github.com/cloudnative-pg/postgis-containers
#       (Based on PostGIS cnpg image because installing it standalone or thorugh trunk caused version issues of dependencies)
#       includes: Barman Cloud, PGAudit, Postgres Failover Slots, pgRouting
FROM ghcr.io/cloudnative-pg/postgis:${postgresql_major}-3.5 AS final

ARG postgresql_major
USER root

COPY --from=builder /root/.cargo/bin/trunk /usr/bin/trunk

# install system dependencies for trunk extensions (taken from terminal logs of trunk): pgvector, TODO: add dependencies for rest of extensions 
RUN apt-get update && apt-get install -y --no-install-recommends \
    libc6 \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Install PostgreSQL extensions using trunk
RUN trunk install pgvector 
RUN trunk install pg_stat_statements # TODO: install dependencies
RUN trunk install timescaledb  # TODO: install dependencies
RUN trunk install pg_repack  # TODO: install dependencies
RUN trunk install pg_cron  # TODO: install dependencies
RUN trunk install pgsql_http  # TODO: install dependencies
RUN trunk install auto_explain # TODO: install dependencies

RUN rm -rf /tmp/* /usr/bin/trunk

# Fixes permissions
RUN chown -R 26:26 /var/lib/postgresql

# revert back to postgres user
RUN usermod -u 26 postgres
USER 26

#========================================================================================


# TODO:
# PostgREST
# PostGraphile

# TODO: pick extensions to use
# RUN trunk install plpgsql_check
# RUN trunk install wal2json
# RUN trunk install rum
# RUN trunk install pg_hashids
# RUN trunk install pg_graphql
# RUN trunk install pg_stat_monitor
# RUN trunk install pg_jsonschema
# RUN trunk install wrappers
# RUN trunk install hypopg
# RUN trunk install pg_tle
# RUN trunk install index_advisor
# RUN trunk install supautils
