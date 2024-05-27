# Use the official Rust image as a base
FROM rust:latest as builder

# Set the working directory
WORKDIR hw1/src

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Use a minimal base image for the final stage
FROM debian:buster-slim

# Copy the built binary from the builder stage
COPY --from=builder hw1/src/target/release/rust_webapp /usr/local/bin/rust_webapp

# Expose the port on which the app will run
EXPOSE 8000

# Set the startup command to run the binary
CMD ["rust_webapp"]
