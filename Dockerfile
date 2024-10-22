# Use the official Rust image as the base image
FROM rust:1.82.0 AS build

# Install musl-tools for static linking
RUN apt-get update && apt-get install -y musl-tools

# Install the musl target
RUN rustup target add x86_64-unknown-linux-musl

# Create a new directory for the application
WORKDIR /usr/src/zyrixo

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Build the dependencies (this helps leverage Docker layer caching)
RUN cargo fetch

# Copy the source code into the container
COPY . .

# Build the application statically in release mode
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Start a new stage for a minimal image with just the binary
FROM debian:buster-slim

# Install necessary dependencies including CA certificates
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the statically compiled binary
COPY --from=build /usr/src/zyrixo/target/x86_64-unknown-linux-musl/release/zyrixo /usr/local/bin/zyrixo

# Set the default command to run the application
CMD ["zyrixo"]