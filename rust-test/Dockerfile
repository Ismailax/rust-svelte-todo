# ---------- Build stage ----------
FROM rust:slim-bookworm AS builder
WORKDIR /app

# Build real app
COPY . .
RUN cargo build --release

# ---------- Runtime stage ----------
FROM debian:bookworm-slim

# Minimal runtime deps (HTTPS certs, tzdata for timestamps if needed)
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates tzdata \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary
COPY --from=builder /app/target/release/rust-test /usr/local/bin/app

# (Optional) helpful logging
ENV RUST_LOG=debug

EXPOSE 8081
USER 10001
CMD ["/usr/local/bin/app"]