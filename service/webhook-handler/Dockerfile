# syntax=docker/dockerfile:1

# building/compiling stage
FROM rust AS build

RUN apt-get update && apt-get install -y bash
SHELL ["/bin/bash", "-c"]

COPY . /app
WORKDIR /app

RUN chmod +x ./script/deploy.sh && source ./script/deploy.sh && install.system-dependency@webhook-handler

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

RUN cargo install cargo-watch --locked

RUN apt-get update && apt-get install -y bash
SHELL ["/bin/bash", "-c"]

COPY . /app
WORKDIR /app

RUN chmod +x ./script/deploy.sh && source ./script/deploy.sh && install.system-dependency@webhook-handler

ENV RUST_LOG=debug

COPY ./script/dev.sh /bin/dev.sh
RUN chmod +x /bin/dev.sh

ENTRYPOINT ["/bin/bash", "-c"]
CMD ["source /bin/dev.sh && hot_reload@webhook-handler"]
