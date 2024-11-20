# syntax=docker/dockerfile:1
FROM rust:1.81.0-alpine As chef

RUN set -eux
RUN apk add --no-cache musl-dev pkgconfig openssl-dev openssl-libs-static
RUN cargo install cargo-chef
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown
RUN rm -rf $CARGO_HOME/registry

ENV PROJECT_PATH=/usr/src/project
ENV NODE_PATH=${PROJECT_PATH}/frand-node
ENV APP_PATH=${PROJECT_PATH}/frand-home
ENV APP_BACKEND_PATH=${APP_PATH}/target/release/frand-home
ENV APP_DIST_PATH=${APP_PATH}/target/dist

COPY frand-node ${NODE_PATH}
COPY frand-home ${APP_PATH}

FROM chef AS planner

WORKDIR ${APP_PATH}

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

WORKDIR ${APP_PATH}

COPY --from=planner ${APP_PATH}/recipe.json .

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release
RUN trunk build --release