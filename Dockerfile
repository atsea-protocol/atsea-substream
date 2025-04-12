# 2. Final Runtime Stage
FROM ubuntu:20.04
RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/local/bin/substreams-sink-pubsub /usr/local/bin/
COPY substreams.yaml /app/substreams.yaml
COPY target/wasm32-unknown-unknown/release/substreams.wasm /app/substreams.wasm
