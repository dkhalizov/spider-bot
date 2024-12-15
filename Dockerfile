FROM rust:1.83-slim AS builder

WORKDIR /usr/src/app

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && \
    apt-get install -y libssl3 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/spider-bot /app/spider-bot

ENTRYPOINT ["/app/spider-bot"]
