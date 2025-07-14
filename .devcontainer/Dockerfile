FROM rust:bookworm

ARG CONTAINER_USER=crab

# libc is needed for rust analyzer
RUN apt-get update \
    && apt-get install -y git libc-bin \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/* /tmp/library-scripts


# Set users
RUN adduser --disabled-password --gecos "" ${CONTAINER_USER}
USER ${CONTAINER_USER}
WORKDIR /home/${CONTAINER_USER}

# Install formatter for vscode plugin to work
RUN rustup component add rustfmt

# Specify wasm build target
RUN rustup target add wasm32-unknown-unknown

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall -y wasm-pack
