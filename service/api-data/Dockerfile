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
FROM gcr.io/distroless/cc AS final

# ISSUE: github container registry doesn't pick up org.opencontainers.image.source value to associate automatically the image with the repo.
LABEL org.opencontainers.image.source="https://github.com/szn-app/donation-app"
LABEL org.opencontainers.image.description="donation-app api-data microservice"
LABEL org.opencontainers.image.licenses="GPL-3.0"

COPY --from=build /bin/api-data /bin/
ENTRYPOINT ["/bin/api-data"]
