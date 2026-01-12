FROM rust as builder
WORKDIR /usr/src/yala

COPY . .

RUN cargo build --release

# Final minimal image
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/yala/target/release/yala /usr/local/bin/yala

ENTRYPOINT ["/usr/local/bin/yala"]
