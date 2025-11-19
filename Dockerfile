FROM rust:1.86-slim

SHELL ["bash", "-c"]

RUN apt-get update && apt-get install -y \
    pkg-config \
    protobuf-compiler \
    clang \
    make

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked linera-service@0.15.6 linera-storage-service@0.15.6

RUN apt-get install -y curl gnupg python3 python3-pip
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - \
    && apt-get install -y nodejs

WORKDIR /build

HEALTHCHECK CMD ["curl", "-s", "http://localhost:5173"]

ENTRYPOINT ["bash", "run.bash"]