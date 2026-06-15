# ---- Stage 1: Build ----
FROM rust:1.88-alpine AS builder

RUN apk add --no-cache     musl-dev     pkgconfig     openssl-dev     openssl-libs-static     postgresql-dev     make     g++

WORKDIR /app

ENV CARGO_BUILD_TARGET=x86_64-unknown-linux-musl
ENV RUSTFLAGS="-C target-feature=+crt-static"

COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl 2>/dev/null; rm -rf src

COPY src ./src
COPY migrations ./migrations
RUN touch src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl

# ---- Stage 2: Runtime ----
FROM alpine:3.20 AS runtime

RUN apk add --no-cache ca-certificates tzdata

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/backend_rust ./backend_rust
COPY --from=builder /app/migrations ./migrations

EXPOSE 8080

CMD ["./backend_rust"]
