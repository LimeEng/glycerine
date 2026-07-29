FROM rust:1.97.1 AS builder
WORKDIR /usr/src/glycerine

# Overview
# Copy Cargo.toml and Cargo.lock
# Create an empty main.rs file to allow cargo to compile and build dependencies
# Remove the empty main.rs and copy over the actual source code
# This way the dependencies should remain cached

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && printf 'fn main() {}\n' > src/main.rs \
    && cargo build --release --locked \
    && rm -rf src

COPY . .
RUN cargo build --release --locked

FROM debian:trixie-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/glycerine/target/release/glycerine /usr/local/bin/glycerine
CMD ["glycerine"]
