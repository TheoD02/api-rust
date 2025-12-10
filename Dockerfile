# ---- Build Stage ----
FROM rust:1 AS builder

# Create app directory
WORKDIR /app

# Cache dependencies first
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy real source
COPY . .

# Build actual binary
RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:stable-slim

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/* /app/app

# Run the app
CMD ["/app/app"]
