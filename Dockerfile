# Rust as the base image
FROM rust:latest as build

# Create a new empty shell project
RUN USER=root cargo new --bin hh-notifier
WORKDIR /hh-notifier

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release.
RUN cargo build --release

# The final base image
FROM debian:buster-slim

# Copy from the previous build
COPY --from=build /hh-notifier/target/release/hh-notifier /usr/src/hh-notifier
# COPY --from=build /hh-notifier/target/release/hh-notifier/target/x86_64-unknown-linux-musl/release/hh-notifier .

# Run the binary
CMD ["/usr/src/hh-notifier"]
