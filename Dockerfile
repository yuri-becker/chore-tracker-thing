# Build Frontend
FROM node:lts-slim AS frontend-builder
WORKDIR /opt/ctt-web
RUN corepack enable
COPY web/index.html \
    web/package.json \
    web/tsconfig.app.json \
    web/tsconfig.json \
    web/tsconfig.node.json \
    web/vite.config.ts \
    web/yarn.lock \
    ./ 
COPY web/public public
COPY web/src src
RUN corepack install && \
    yarn && \
    yarn build

# Build Backend
FROM rust:1 AS backend-builder
WORKDIR /opt/chore-tracker-thing
COPY Cargo.toml Cargo.lock ./
COPY src src
RUN cargo build --release

# Run Stack
FROM debian:stable-slim AS app
LABEL version="0.0.1"
WORKDIR /opt
RUN set -eux; \
    apt update; \
    apt install -y --no-install-recommends \
    ca-certificates \
    nginx \
    supervisor; \
    apt clean autoclean;

# Copy build outputs
COPY --from=frontend-builder /opt/ctt-web/dist web
COPY --from=backend-builder /opt/chore-tracker-thing/target/release/chore-tracker-thing ctt

# Copy configurations
COPY .docker/nginx.conf /etc/nginx/sites-enabled/default 
COPY .docker/stop-supervisor.sh stop-supervisor.sh
RUN chmod +x /opt/stop-supervisor.sh
COPY .docker/supervisord.conf supervisord.conf

CMD ["supervisord", "-c", "/opt/supervisord.conf"]