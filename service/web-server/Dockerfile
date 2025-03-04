# syntax=docker/dockerfile:1

FROM node:lts AS development
RUN npm install -g pnpm

COPY . /app
WORKDIR /app

RUN pnpm install

CMD ["pnpm", "run", "dev-service-port"]

# --- 

# building/compiling stage
FROM node:lts AS build
RUN npm install -g pnpm

# ARG ENV=development
# ENV NODE_ENV=$ENV

WORKDIR /app
COPY . /app

RUN pnpm install && pnpm run build

# --- 

# image hosting application
FROM nginx:alpine AS production
# ISSUE: github container registry doesn't pick up org.opencontainers.image.source value to associate automatically the image with the repo.
LABEL org.opencontainers.image.source="https://github.com/szn-app/donation-app"
LABEL org.opencontainers.image.description="donation-app microservice"
LABEL org.opencontainers.image.licenses="GPL-3.0"

# configure run script
COPY ./.nginx/nginx.conf /etc/nginx/conf.d/default.conf
COPY script/nginx.sh.template /bin/nginx.sh
RUN chmod +x /bin/nginx.sh

WORKDIR /usr/share/nginx/html
# Remove default nginx static assets
RUN rm -rf ./*
# Copy static assets from builder stage
COPY --from=build /app/dist .

# Containers run nginx with global directives and daemon off
ENTRYPOINT ["/bin/nginx.sh"]
