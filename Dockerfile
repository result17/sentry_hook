FROM rust:1-slim-buster AS builder

RUN apt-get update \
    && apt-get install --no-install-recommends -y musl-tools=1.1.21-2 \
    && rustup target add x86_64-unknown-linux-musl

WORKDIR /app

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo fetch

COPY . .
RUN cargo build --bin sentry_webhook --release --target x86_64-unknown-linux-musl

FROM debian:buster-slim

RUN apt-get update \
    && apt-get install curl

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/sentry_webhook /usr/bin/sentry_webhook

ENTRYPOINT ["/bin/sh", "-c", "sentry_webhook"]
