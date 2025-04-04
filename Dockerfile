# ----------------------------------------
# 1. Build Stage
# ----------------------------------------
FROM golang:1.20 as builder

# Install Git so we can clone the repository
RUN apt-get update && apt-get install -y git && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Clone substreams-sink-pubsub from GitHub
RUN git clone https://github.com/streamingfast/substreams-sink-pubsub.git

# Build the binary
WORKDIR /build/substreams-sink-pubsub
RUN go build -o /usr/local/bin/substreams-sink-pubsub ./cmd/substreams-sink-pubsub

# ----------------------------------------
# 2. Final (Runtime) Stage
# ----------------------------------------
FROM ubuntu:20.04

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
  && rm -rf /var/lib/apt/lists/*

# Copy compiled binary from builder
COPY --from=builder /usr/local/bin/substreams-sink-pubsub /usr/local/bin/substreams-sink-pubsub

# Set up working directory
WORKDIR /app

# Copy your Substreams manifest, WASM, etc. into /app
COPY substreams.yaml /app/substreams.yaml
COPY substreams.wasm /app/substreams.wasm

COPY target/wasm32-unknown-unknown/release/substreams.wasm /app/substreams.wasm

ENTRYPOINT substreams-sink-pubsub sink /app/substreams.yaml map_pubsub anchorDroppeds 8044914: --project=atsea-dev
