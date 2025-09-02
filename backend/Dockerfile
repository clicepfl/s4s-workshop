# Build on a dedicated image to avoid build output in final image
FROM docker.io/rustlang/rust:nightly-alpine AS rust_build

RUN apk add musl-dev

# cargo-build-deps is a tool to only install and build dependencies
RUN cargo install cargo-build-deps

RUN cargo new app
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo build-deps --release

# Add and build project
COPY src ./src
RUN cargo build --release

# Setup runtime image
FROM alpine:latest AS runtime

RUN apk add ca-certificates openssl docker

WORKDIR /app

# Setup and copy the build result

ENV DATA_DIR /data
RUN mkdir /data
ENV SOCK_DIR /sockets
RUN mkdir /sockets

ENV SOCK_VOL=s4s_sockets
ENV DOCKER_HOST=unix:///var/run/docker.sock

ENV ROCKET_ADDRESS=0.0.0.0

COPY --from=rust_build /app/target/release/backend ./backend

ENV RUST_LOG=info

CMD ["./backend"]
