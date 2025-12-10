# ---- Build Stage ----
FROM rust:1 AS builder

WORKDIR /app

# Copier tous les fichiers Cargo (y compris sous-dossiers)
COPY Cargo.toml Cargo.lock ./
COPY migration ./migration
COPY src ./src

# Compiler pour créer un cache des dépendances
RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:stable-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/* /app/app

CMD ["/app/app"]
