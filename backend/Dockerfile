#######################
#     build stage     #
#######################
FROM rust:1.57.0 AS build
WORKDIR /app
# first install the dependencies to leverage docker's build cache.
RUN cargo init
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
# then copy the source code and build the binary.
COPY . .
RUN touch src/main.rs && cargo build --release

#######################
#     final stage     #
#######################
FROM ubuntu:20.04
WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev
COPY --from=build /app/target/release/chitchat /usr/local/bin/chitchat
EXPOSE 3000
CMD [ "chitchat" ]
