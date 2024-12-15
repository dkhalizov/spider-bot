FROM rust:1.83-slim AS builder

WORKDIR /usr/src/app

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM scratch AS runtime
WORKDIR /app

COPY --from=builder /usr/src/app/target/release/spider-bot /app/spider-bot

ENTRYPOINT ["/app/spider-bot"]
