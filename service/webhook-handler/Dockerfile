# syntax=docker/dockerfile:1

# building/compiling stage
FROM rust AS build

RUN apt-get update && apt-get install -y bash
SHELL ["/bin/bash", "-c"]

# musl toolchain
# RUN apt-get update -y && apt-get install musl-tools -y
# RUN rustup target add x86_64-unknown-linux-musl

COPY . /app
WORKDIR /app

RUN chmod +x ./script/deploy.sh && source ./script/deploy.sh && install.system-dependency.kafka#docker@webhook-handler

# NOTE: distroless image lacks protobuf required dependencies; otherwise build with 'musl' relase target
# RUN cargo build --release --locked --target x86_64-unknown-linux-musl

RUN cargo build --release --locked

# configure run script
# RUN mv /app/target/x86_64-unknown-linux-musl/release/webhook-handler /bin/webhook-handler
RUN mv /app/target/release/webhook-handler /bin/webhook-handler
RUN chmod +x /bin/webhook-handler

# ---

# image hosting application (if built with glibc, binary would require glibc and requires dependencies for tonic protobuf)
# TODO: use slim version after satisfying protobuf requirements, possible by retrying to build with `musl` and install dependenices on a 'distroless' image
# FROM rust:slim-bookworm AS release
FROM rust:latest AS release

# ISSUE: github container registry doesn't pick up org.opencontainers.image.source value to associate automatically the image with the repo.
LABEL org.opencontainers.image.source="https://github.com/szn-app/donation-app"
LABEL org.opencontainers.image.description="donation-app webhook-handler microservice"
LABEL org.opencontainers.image.licenses="GPL-3.0"

COPY --from=build /bin/webhook-handler /bin/
ENTRYPOINT ["/bin/webhook-handler"]

# ---

FROM rust AS debug

RUN cargo install cargo-watch --locked

RUN apt-get update && apt-get install -y bash
SHELL ["/bin/bash", "-c"]

COPY . /app
WORKDIR /app

RUN chmod +x ./script/deploy.sh && source ./script/deploy.sh && install.system-dependency.kafka#docker@webhook-handler

ENV RUST_LOG=debug

COPY ./script/dev.sh /bin/dev.sh
RUN chmod +x /bin/dev.sh

ENTRYPOINT ["/bin/bash", "-c"]
CMD ["source /bin/dev.sh && hot_reload@webhook-handler"]
