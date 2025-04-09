# syntax=docker/dockerfile:1

# building/compiling stage
FROM rust AS build

COPY . /app
WORKDIR /app

# Install build dependencies
RUN apt update && apt install -y cmake libsasl2-modules-gssapi-mit libsasl2-dev

RUN cargo build --release --locked

# configure run script
RUN mv /app/target/release/webhook-handler /bin/webhook-handler
RUN chmod +x /bin/webhook-handler

# ---

# image hosting application (required glibc for rust)
FROM gcr.io/distroless/cc AS production

# ISSUE: github container registry doesn't pick up org.opencontainers.image.source value to associate automatically the image with the repo.
LABEL org.opencontainers.image.source="https://github.com/szn-app/donation-app"
LABEL org.opencontainers.image.description="donation-app webhook-handler microservice"
LABEL org.opencontainers.image.licenses="GPL-3.0"

COPY --from=build /bin/webhook-handler /bin/
ENTRYPOINT ["/bin/webhook-handler"]

# ---

FROM rust AS development

# Install cmake build dependencies
RUN apt update && apt install -y cmake libsasl2-modules-gssapi-mit libsasl2-dev 

RUN cargo install cargo-watch --locked

COPY . /app
WORKDIR /app

ENV RUST_LOG=debug

COPY ./script/dev.sh /bin/dev.sh
RUN chmod +x /bin/dev.sh

ENTRYPOINT ["/bin/bash", "-c"]
CMD ["source /bin/dev.sh && hot_reload@webhook-handler"]
