# syntax=docker/dockerfile:1

# building/compiling stage
FROM rust AS build

COPY . /app
WORKDIR /app

RUN cargo build --release --locked

# configure run script
RUN mv /app/target/release/auth-token-exchange /bin/auth-token-exchange
RUN chmod +x /bin/auth-token-exchange

# ---

# image hosting application (required glibc for rust)
FROM gcr.io/distroless/cc AS release

# ISSUE: github container registry doesn't pick up org.opencontainers.image.source value to associate automatically the image with the repo.
LABEL org.opencontainers.image.source="https://github.com/szn-app/donation-app"
LABEL org.opencontainers.image.description="donation-app auth-token-exchange microservice"
LABEL org.opencontainers.image.licenses="GPL-3.0"

COPY --from=build /bin/auth-token-exchange /bin/
ENTRYPOINT ["/bin/auth-token-exchange"]

# ---

FROM rust AS debug
RUN cargo install cargo-watch --locked

COPY . /app
WORKDIR /app

ENV RUST_LOG=debug

COPY ./script/dev.sh /bin/dev.sh
RUN chmod +x /bin/dev.sh

ENTRYPOINT ["/bin/bash", "-c"]
CMD ["source /bin/dev.sh && hot_reload@auth-token-exchange"]
