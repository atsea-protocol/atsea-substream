# 1. Build Stage (Go 1.21!)
FROM golang:1.21 as builder

# The rest remains the same...
RUN apt-get update && apt-get install -y git && rm -rf /var/lib/apt/lists/*
WORKDIR /build
RUN git clone https://github.com/streamingfast/substreams-sink-pubsub.git
WORKDIR /build/substreams-sink-pubsub
RUN go build -o /usr/local/bin/substreams-sink-pubsub ./cmd/substreams-sink-pubsub

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

ENTRYPOINT substreams-sink-pubsub sink /app/substreams.yaml map_pubsub anchorDroppeds 8044914: --project=atsea-dev
