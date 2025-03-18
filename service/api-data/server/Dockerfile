# syntax=docker/dockerfile:1

# building/compiling stage
FROM rust AS build

COPY . /app
WORKDIR /app

RUN cargo build --release --locked

# configure run script
RUN mv /app/target/release/api-data /bin/api-data
RUN chmod +x /bin/api-data

# ---

# image hosting application (required glibc for rust)
FROM gcr.io/distroless/cc AS production

# ISSUE: github container registry doesn't pick up org.opencontainers.image.source value to associate automatically the image with the repo.
LABEL org.opencontainers.image.source="https://github.com/szn-app/donation-app"
LABEL org.opencontainers.image.description="donation-app api-data microservice"
LABEL org.opencontainers.image.licenses="GPL-3.0"

COPY --from=build /bin/api-data /bin/
ENTRYPOINT ["/bin/api-data"]

# ---

FROM rust:latest AS development
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# ENV PATH="/root/.cargo/bin:${PATH}"
# fixes cargo build error on volume mounted code - https://github.com/rust-lang/rustup/issues/2949
# ENV RUSTUP_PERMIT_COPY_RENAME=true
# ENV CARGO_TARGET_DIR=/tmp/target

RUN rustup toolchain install nightly && \
    rustup toolchain install stable && \
    rustup default nightly && \
    rustup component add rustfmt clippy
RUN cargo install cargo-watch --locked
RUN rustup default nightly && rustc --version && cargo --version

COPY . /app
WORKDIR /app

ENV RUST_LOG=debug

COPY ./script/dev.sh /bin/dev.sh
RUN chmod +x /bin/dev.sh

ENTRYPOINT ["/bin/bash", "-c"]
CMD ["source /bin/dev.sh && hot_reload@api-data"]
