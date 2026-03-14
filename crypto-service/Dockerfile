# Multi-stage build for Rust
FROM rust:1.75 AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release 2>&1 | grep -v "not found"

COPY . .
RUN cargo build --release

# Final stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /build/target/release/otp-crypto .

EXPOSE 9090

HEALTHCHECK --interval=10s --timeout=5s --retries=3 \
  CMD curl -f http://localhost:9090/health || exit 1

CMD ["./otp-crypto"]
