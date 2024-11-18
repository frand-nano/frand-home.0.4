# syntax=docker/dockerfile:1
FROM frand-builder AS chef

ENV PROJECT_PATH=/usr/src/project
ENV APP_PATH=${PROJECT_PATH}/frand-home
ENV APP_BACKEND_PATH=${APP_PATH}/target/release/frand-home
ENV APP_DIST_PATH=${APP_PATH}/target/dist

WORKDIR ${PROJECT_PATH}

FROM chef AS planner

WORKDIR ${APP_PATH}

COPY . .
COPY --from=frand-node ${PROJECT_PATH} ${PROJECT_PATH}/frand-node

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

WORKDIR ${APP_PATH}

COPY --from=frand-node ${PROJECT_PATH} ${PROJECT_PATH}/frand-node
COPY --from=planner ${APP_PATH}/recipe.json .

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release
RUN trunk build --release

RUN cp ${APP_BACKEND_PATH} /usr/local/bin/frand-home
RUN mkdir /usr/local/bin/target
RUN cp -r ${APP_DIST_PATH} /usr/local/bin/target/dist

FROM alpine:3.20

WORKDIR /usr/local/bin

COPY --from=builder /usr/local/bin/frand-home .
COPY --from=builder /usr/local/bin/target/dist ./target/dist