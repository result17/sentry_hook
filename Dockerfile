FROM rust:1.69-alpine3.17
COPY . .
RUN cargo build --release
CMD ["./target/release/sentry _webhook"]
