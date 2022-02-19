FROM rustlang/rust:nightly-buster as build

# create a new empty shell project
RUN USER=root cargo new --bin contracts-streamflow-poller
WORKDIR /contracts-streamflow-poller

RUN apt-get update && apt-get install libudev-dev

# copy over your manifests
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/contracts_streamflow_poller*
RUN cargo build --release

# our final base
FROM debian:buster-slim

RUN apt-get update && apt-get install libudev-dev

# copy the build artifact from the build stage
COPY --from=build /contracts-streamflow-poller/target/release/contracts-streamflow-poller .

# set the startup command to run your binary
CMD ["./contracts-streamflow-poller"]