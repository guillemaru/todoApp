# Use the official Rust image as the builder
FROM rust:latest as builder

# Set the working directory to the root of the project
WORKDIR /app

# Copy the Rust project to the container
COPY server /app

# Install any necessary dependencies
RUN cargo install diesel_cli --no-default-features --features postgres && \
    cargo build --release

# Use a smaller and more efficient runtime image
FROM debian:buster-slim

# Set the working directory to /app
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release /app/target/release
COPY server/migrations /app

RUN apt-get update
RUN apt-get install -y libpq-dev

# Expose the port that the application will listen on
EXPOSE 5000

# Start the application
CMD ["/app/target/release/server"]