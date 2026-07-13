# syntax=docker/dockerfile:1

# Recommended software versions matching README.md
ARG RUST_VERSION=1.96.1
ARG DEBIAN_VERSION=bullseye-20260623-slim

ARG BUILDER_IMAGE="rust:${RUST_VERSION}-slim-bullseye"
ARG RUNNER_IMAGE="debian:${DEBIAN_VERSION}"

FROM --platform=$BUILDPLATFORM ${BUILDER_IMAGE} AS builder

ARG TARGETPLATFORM
ARG BUILDPLATFORM

LABEL org.opencontainers.image.authors=conradwt@gmail.com
LABEL org.opencontainers.image.title="{{project-name}}"
LABEL org.opencontainers.image.licenses=MIT
LABEL com.conradtaylor.rust_version=$RUST_VERSION

# Install build dependencies
RUN apt-get update -y && apt-get install -y pkg-config libssl-dev build-essential git protobuf-compiler curl \
  && apt-get clean && rm -f /var/lib/apt/lists/*_*

# Install buf
RUN BIN="/usr/local/bin" && \
    curl -sSL "https://github.com/bufbuild/buf/releases/latest/download/buf-$(uname -s)-$(uname -m)" -o "${BIN}/buf" && \
    chmod +x "${BIN}/buf"

# Install connectrpc-codegen and protoc-gen-buffa plugins
RUN cargo install connectrpc-codegen protoc-gen-buffa protoc-gen-buffa-packaging

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && \
    echo "fn main() {}" > src/server.rs && \
    echo "fn main() {}" > src/client.rs
RUN cargo build --release
RUN rm -rf src

# Copy config and proto definitions
COPY buf.yaml buf.gen.yaml ./
COPY proto ./proto

# Generate code
RUN buf generate proto

# Copy actual source code
COPY src ./src

# Touch files to force cargo to rebuild them (since we replaced dummy files)
RUN touch src/server.rs src/client.rs

# Build actual release binary
RUN cargo build --release

# Final runtime image
FROM ${RUNNER_IMAGE}

# Install SSL and other runtime necessities
RUN apt-get update -y && apt-get install -y libssl1.1 ca-certificates locales \
  && apt-get clean && rm -f /var/lib/apt/lists/*_*

# Set the locale
RUN sed -i '/en_US.UTF-8/s/^# //g' /etc/locale.gen && locale-gen

ENV LANG=en_US.UTF-8
ENV LANGUAGE=en_US:en
ENV LC_ALL=en_US.UTF-8

WORKDIR /app
RUN chown nobody /app

# Copy the compiled binaries from builder
COPY --from=builder --chown=nobody:root /app/target/release/server ./
COPY --from=builder --chown=nobody:root /app/target/release/client ./

USER nobody

# Start the web server by default
CMD ["/app/server"]
