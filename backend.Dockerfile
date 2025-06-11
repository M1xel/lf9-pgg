# ---- Stage 1: Build (Static Linking) ----
FROM rust:latest as builder

WORKDIR /usr/src/app

# Install the musl target and musl-tools
RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install -y musl-tools

# Create a .cargo directory and configure for static linking with musl
RUN mkdir -p .cargo
RUN echo '[target.x86_64-unknown-linux-musl]' > .cargo/config.toml
RUN echo 'linker = "rust-lld"' >> .cargo/config.toml

# Copy configuration and lock files needed to resolve dependencies
COPY Cargo.toml ./Cargo.toml
COPY Cargo.lock ./Cargo.lock
COPY crates/backend/Cargo.toml crates/backend/Cargo.toml
COPY crates/migration/Cargo.toml crates/migration/Cargo.toml

# Fetch dependencies based on the lock file.
RUN cargo fetch

# Copy the actual source code for all workspace members
COPY crates/backend/ crates/backend/
COPY crates/migration/ crates/migration/

# Build the release binary for the musl target
RUN cargo build --release --target x86_64-unknown-linux-musl

# Debug: List the built binaries to verify what we have
RUN ls -la /usr/src/app/target/x86_64-unknown-linux-musl/release/

# ---- Stage 2: Runtime ----
FROM alpine:latest

# Install minimal runtime dependencies (ca-certificates is often needed)
RUN apk add --no-cache ca-certificates

WORKDIR /app

# Copy only the statically linked compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/backend /app/backend

# Verify the binary exists and is executable
RUN ls -la /app && chmod +x /app/backend

# Expose the port the application listens on
EXPOSE 8080

# Set the command to run the application
CMD ["/app/backend"]