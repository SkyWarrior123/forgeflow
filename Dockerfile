# ---------- Rust build ----------
FROM rust:1.82-bookworm AS rust-build
WORKDIR /src
COPY Cargo.toml rust-toolchain.toml ./
COPY crates ./crates
COPY apps/api ./apps/api
COPY migrations ./migrations
RUN cargo build --release -p forgeflow-api

# ---------- Web build ----------
FROM node:22-bookworm-slim AS web-build
WORKDIR /web
COPY apps/web/package.json apps/web/package-lock.json* ./
RUN npm ci || npm install
COPY apps/web ./
RUN npm run build

# ---------- Runtime ----------
FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --create-home forgeflow

WORKDIR /app
COPY --from=rust-build /src/target/release/forgeflow-api /usr/local/bin/forgeflow-api
COPY --from=web-build /web/dist /app/static

USER 10001
EXPOSE 8080
ENV STATIC_DIR=/app/static
ENTRYPOINT ["/usr/local/bin/forgeflow-api"]