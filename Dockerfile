FROM rust:1-alpine3.17 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:3.17.3
WORKDIR /app
COPY --from=builder /app/target/release/sentry_webhook .
CMD ["ls", "-a"]
