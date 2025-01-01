FROM debian:stable-slim
LABEL version="0.0.1"

# Install Prerequisites
RUN apt update &&\
    apt install --yes --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    libssl-dev \
    pkg-config \
    nginx \
    supervisor

# Install Node & Yarn
RUN curl -fsSL https://deb.nodesource.com/setup_22.x -o /opt/nodesource_setup.sh
RUN chmod +x /opt/nodesource_setup.sh && /opt/nodesource_setup.sh -y
RUN rm -f /opt/nodesource_setup.sh
RUN apt update && \
    apt install --yes --no-install-recommends \
    nodejs
RUN corepack enable

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /opt/rustup_init.sh
RUN chmod +x /opt/rustup_init.sh && /opt/rustup_init.sh -y
RUN . "$HOME/.cargo/env" && rustup toolchain install stable
RUN rm -f /opt/rustup_init.sh

# Copy Backend
RUN mkdir /opt/chore-tracker-thing
WORKDIR /opt/chore-tracker-thing
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src src

# Build Backend
RUN . "$HOME/.cargo/env" && cargo build --release
WORKDIR /opt
RUN mv chore-tracker-thing/target/release/chore-tracker-thing ctt
RUN rm -rf /opt/chore-tracker-thing

# Copy Frontend
RUN mkdir /opt/chore-tracker-thing-web
WORKDIR /opt/chore-tracker-thing-web
COPY web/index.html index.html 
COPY web/package.json package.json 
COPY web/tsconfig.app.json tsconfig.app.json 
COPY web/tsconfig.json tsconfig.json 
COPY web/tsconfig.node.json tsconfig.node.json 
COPY web/vite.config.ts vite.config.ts 
COPY web/yarn.lock yarn.lock 
COPY web/public public 
COPY web/src src 

# Build Frontend
RUN corepack install
RUN yarn
RUN yarn build
WORKDIR /opt
RUN mv chore-tracker-thing-web/dist web
RUN rm -rf chore-tracker-thing-web

## Cleanup
RUN . "$HOME/.cargo/env" && rustup self uninstall -y
RUN apt remove -y nodejs\
    curl\
    build-essential\
    libssl-dev\
    pkg-config 

# Copy configurations
COPY .docker/nginx.conf /etc/nginx/sites-enabled/default 
COPY .docker/stop-supervisor.sh /opt/stop-supervisor.sh
RUN chmod +x /opt/stop-supervisor.sh
COPY .docker/supervisord.conf /opt/supervisord.conf

CMD ["supervisord", "-c", "/opt/supervisord.conf"]