FROM node:16.13 as build-frontend
WORKDIR /frontend
# install and cache dependencies
COPY vue/package.json vue/package-lock.json ./
RUN npm ci
# copy and build source code
COPY vue .
RUN npm run build


FROM rust:1.57 as build-backend
WORKDIR /backend
# install and cache dependencies
RUN cargo init
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
# copy and build source code
COPY src src
RUN touch src/main.rs && cargo build --release
# make sure all tests pass
RUN cargo test --release


FROM debian:buster-slim
WORKDIR /app
COPY --from=build-frontend /static static
COPY --from=build-backend /backend/target/release/chitchat /usr/local/bin/chitchat
EXPOSE 3000
CMD [ "chitchat" ]
