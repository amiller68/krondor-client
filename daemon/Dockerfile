# Courtesy of Roger Torres: https://dev.to/rogertorres/first-steps-with-docker-rust-30oi

# Our build base
FROM rust:1.66-slim-buster as build

# create a new empty shell project
RUN USER=root cargo new --bin daemon
WORKDIR /daemon

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/daemon*
RUN cargo build --release

# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /daemon/target/release/daemon .

# set the startup command to run your binary
CMD ["./daemon"]
