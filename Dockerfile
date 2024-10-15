FROM rust:1.68 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/app/target/release/rust-http-server /usr/local/bin/rust-http-server
EXPOSE 3000
CMD ["rust-http-server"]