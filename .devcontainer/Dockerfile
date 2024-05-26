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

RUN rustup target add wasm32-unknown-unknown


# Install binstall
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash


# note that this might take a while to install because it compiles everything from scratch
# Trunk also provides prebuilt binaries for a number of major package managers
# See https://trunkrs.dev/#install for further details
RUN cargo binstall -y trunk
