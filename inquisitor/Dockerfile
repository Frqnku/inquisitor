FROM rust:1.82-slim as builder

RUN apt-get update && apt-get install -y \
    iproute2 \
    iputils-ping \
    libpcap-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo fetch

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libpcap0.8 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/inquisitor .

CMD ["./inquisitor"]
