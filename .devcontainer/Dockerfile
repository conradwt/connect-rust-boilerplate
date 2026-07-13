# syntax=docker/dockerfile:1

ARG VARIANT="1.96.1"
FROM rust:${VARIANT}-bullseye

ARG USERNAME=nobody
ARG USER_UID=65534
ARG USER_GID=65534

ARG INSTALL_ZSH="true"
ARG UPGRADE_PACKAGES="true"
ARG COMMON_SCRIPT_SOURCE="https://raw.githubusercontent.com/microsoft/vscode-dev-containers/main/script-library/common-debian.sh"
ARG COMMON_SCRIPT_SHA="dev-mode"

# Install needed packages and setup non-root user.
RUN apt-get update \
  && export DEBIAN_FRONTEND=noninteractive \
  && apt-get -y install --no-install-recommends curl ca-certificates 2>&1 \
  && curl -sSL ${COMMON_SCRIPT_SOURCE} -o /tmp/common-setup.sh \
  && ([ "${COMMON_SCRIPT_SHA}" = "dev-mode" ] || (echo "${COMMON_SCRIPT_SHA} */tmp/common-setup.sh" | sha256sum -c -)) \
  && /bin/bash /tmp/common-setup.sh "${INSTALL_ZSH}" "${USERNAME}" "${USER_UID}" "${USER_GID}" "${UPGRADE_PACKAGES}" \
  #
  # Install build dependencies and protoc
  && apt-get install -y build-essential pkg-config libssl-dev lldb protobuf-compiler \
  #
  # Clean up
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/* /tmp/common-setup.sh

# Install buf CLI
RUN BIN="/usr/local/bin" && \
    curl -sSL "https://github.com/bufbuild/buf/releases/latest/download/buf-$(uname -s)-$(uname -m)" -o "${BIN}/buf" && \
    chmod +x "${BIN}/buf"

# Install cargo-generate, connectrpc-codegen and buffa plugins globally
RUN cargo install cargo-generate connectrpc-codegen protoc-gen-buffa protoc-gen-buffa-packaging
