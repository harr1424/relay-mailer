# Stage 1: Build the binary
FROM rust:latest AS builder

# Set working directory
WORKDIR /app

# Copy Cargo.toml and src files separately for efficient caching
COPY Cargo.toml .
COPY src ./src

# Build the project in release mode
RUN cargo build --release

# Runtime stage: Use a secure Debian base with OpenSSL 3 compatibility
FROM debian:bookworm-slim

# Install necessary libraries for Rust binaries
RUN apt-get update && \
    apt-get install -y libgcc-s1 libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory to /app
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/relay-mailer /app/relay-mailer

# Copy the runtime configuration file
COPY SecretConfig.toml /app/SecretConfig.toml 

# Configure the binary as the entrypoint
ENTRYPOINT ["./relay-mailer"]
