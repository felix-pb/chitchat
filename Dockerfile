################################
#     frontend build stage     #
################################
FROM node:16.13.1 as build-frontend
WORKDIR /frontend
# first install the dependencies to leverage docker's build cache.
COPY vue/package.json vue/package-lock.json ./
RUN npm ci
# then copy the source code and build the static assets.
COPY vue .
RUN npm run build

###############################
#     backend build stage     #
###############################
FROM rust:1.57.0 as build-backend
WORKDIR /backend
# first install the dependencies to leverage docker's build cache.
RUN cargo init
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
# then copy the source code and build the binary.
COPY src src
RUN touch src/main.rs && cargo build --release
# finally make sure all tests pass.
RUN cargo test --release

#######################
#     final stage     #
#######################
FROM ubuntu:20.04
WORKDIR /app
COPY --from=build-frontend /static static
COPY --from=build-backend /backend/target/release/chitchat /usr/local/bin/chitchat
EXPOSE 3000
CMD [ "chitchat" ]
