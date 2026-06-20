# ---- Stage 1: Build ----
FROM rust:1.88-alpine AS builder

# protobuf untuk tonic-build, plus toolchain untuk kompilasi musl static (ring/rustls)
RUN apk add --no-cache musl-dev pkgconfig protobuf protobuf-dev make g++

WORKDIR /app

ENV CARGO_BUILD_TARGET=x86_64-unknown-linux-musl
ENV RUSTFLAGS="-C target-feature=+crt-static"

COPY Cargo.toml Cargo.lock* ./
COPY build.rs ./
COPY proto ./proto
COPY src ./src

RUN cargo build --release --target x86_64-unknown-linux-musl

# ---- Stage 2: Runtime ----
FROM alpine:3.20 AS runtime

RUN apk add --no-cache ca-certificates tzdata

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/telegram_bot_grpc ./telegram_bot_grpc

EXPOSE 50051

CMD ["./telegram_bot_grpc"]
