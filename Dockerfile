FROM rust:1-slim-buster AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:3.17
WORKDIR /app
COPY --from=builder /app/target/release/sentry_webhook .
CMD ["./sentry_webhook"]
