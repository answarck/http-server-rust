FROM rust:1.85 as builder

WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder  /app/target/release/http-server /app/server
COPY index.html /app/index.html

EXPOSE 8080

CMD ["./server"]
