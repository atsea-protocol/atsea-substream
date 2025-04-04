FROM ubuntu:20.04

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install substreams-sink-pubsub
RUN curl -L "https://github.com/streamingfast/substreams-sink-pubsub/releases/download/v0.2.0/substreams-sink-pubsub_0.2.0_linux_x86_64.tar.gz" \
    | tar -xz -C /usr/local/bin

WORKDIR /app

# Copy your substreams.yaml
COPY substreams.yaml ./substreams.yaml

# **Copy** the compiled .wasm artifact from your local build
# This must match the cargo build output path
COPY target/wasm32-unknown-unknown/release/substreams.wasm /app/substreams.wasm

ENTRYPOINT substreams-sink-pubsub sink /app/substreams.yaml map_pubsub anchorDroppeds 8044914: --project=atsea-dev
