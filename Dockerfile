# Start from an environment that has 'curl' and 'unzip' at least
FROM ubuntu:20.04

# 1. Install dependencies
RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 2. Download & install substreams-sink-pubsub
#    Replace with the current release or build from source
RUN curl -L "https://github.com/streamingfast/substreams-sink-pubsub/releases/download/v0.2.0/substreams-sink-pubsub_0.2.0_linux_x86_64.tar.gz" \
    | tar -xz -C /usr/local/bin

# 3. Copy in your Substreams config/artifacts
WORKDIR /app
COPY substreams.yaml ./substreams.yaml
COPY substreams.wasm ./substreams.wasm
# If you have more .proto or .spkg files, copy them too

# 5. Copy your GCP credentials into the image (careful: best practice to mount secrets at runtime, not bake them into an image)
# COPY gcp-service-key.json /app/gcp-service-key.json

# 6. Final entrypoint: run substreams-sink-pubsub indefinite from block #8044914
ENTRYPOINT [ \
  "substreams-sink-pubsub", \
  "sink", \
  "/app/substreams.yaml", \
  "map_pubsub", \
  "anchorDroppeds", \
  "8044914:", \
  "--project=atsea-dev" \
]
