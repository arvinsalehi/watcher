# ---- Build Stage ----
# Use the official Rust image as a builder.

FROM rust:1.83-bookworm as builder
# Install build dependencies.
WORKDIR /usr/src/app

# Copy dependency files and build dependencies separately to leverage Docker cache.
COPY Cargo.toml Cargo.lock ./
COPY src ./src
# Build the release binary.
RUN cargo build --release

# ---- Final Stage ----
# Use a minimal, secure base image for the final container.
FROM debian:bookworm-slim as final

# Install runtime dependencies for reqwest/native-tls: OpenSSL + CA certs
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage.
COPY --from=builder /usr/src/app/target/release/watcher /usr/local/bin/

# Set the binary as the container's entrypoint.
CMD ["/usr/local/bin/watcher"]

