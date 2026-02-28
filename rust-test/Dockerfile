# ---------- Build stage ----------
FROM rust:slim-bookworm AS builder
WORKDIR /app

# Build real app
COPY . .
RUN cargo clean && cargo build --release

# ---------- Runtime stage ----------
FROM debian:bookworm-slim

# Minimal runtime deps (HTTPS certs, tzdata for timestamps if needed)
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates tzdata \
 && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 10001 appuser

WORKDIR /app

# Copy the built binary
COPY --from=builder /app/target/release/rust-test /usr/local/bin/app

# (Optional) helpful logging
ENV RUST_LOG=debug

# Make sure keys path exists (the volume will overlay it)
RUN mkdir -p /app/keys

# If permissions are an issue, uncomment:
# USER root
# RUN chmod -R a+r /app/keys
# USER appuser

EXPOSE 8081
USER appuser
CMD ["/usr/local/bin/app"]