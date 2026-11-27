FROM rust:1-slim-bookworm AS builder
WORKDIR /build
RUN apt-get update && apt-get install -y --no-install-recommends build-essential pkg-config && rm -rf /var/lib/apt/lists/*
COPY Backend/Cargo.toml Backend/Cargo.toml
COPY Backend/src Backend/src
RUN cd Backend && cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates ffmpeg && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /build/Backend/target/release/agentfather-backend /usr/local/bin/agentfather-backend
COPY frontend/dist /app/frontend/dist
COPY assets /app/assets
RUN mkdir -p /app/data /app/hermes_data/audio
EXPOSE 8090
CMD ["agentfather-backend"]
