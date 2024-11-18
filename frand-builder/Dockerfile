# syntax=docker/dockerfile:1
FROM rust:1.81.0-alpine

RUN set -eux
RUN apk add --no-cache musl-dev pkgconfig openssl-dev openssl-libs-static
RUN cargo install cargo-chef
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown
RUN rm -rf $CARGO_HOME/registry
