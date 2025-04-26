# example config: https://github.com/voltade/cnpg-supabase/blob/main/Dockerfile

ARG debian_version=bookworm
ARG postgresql_major=17
ARG postgresql_release=${postgresql_major}.4

# https://github.com/cloudnative-pg/postgres-containers/pkgs/container/postgresql/399084424?tag=17-bookworm
# https://github.com/cloudnative-pg/postgres-containers?tab=readme-ov-file#standard-images
# https://github.com/cloudnative-pg/postgres-containers/blob/main/Debian/ClusterImageCatalog-bookworm.yaml
# https://github.com/cloudnative-pg/postgres-containers/blob/main/Debian/17/bookworm/Dockerfile
# https://github.com/cloudnative-pg/postgres-containers/pkgs/container/postgresql/versions?filters%5Bversion_type%5D=tagged
FROM ghcr.io/cloudnative-pg/postgresql:${postgresql_release}-standard-${debian_version} AS final

ARG postgresql_major
USER root

# TODO: install extensions

# PostGIS
# PostgREST
# PostGraphile
