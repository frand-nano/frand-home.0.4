# syntax=docker/dockerfile:1
FROM frand-builder AS builder

WORKDIR ${PROJECT_PATH}
COPY --from=frand-node ${PROJECT_PATH} ${PROJECT_PATH}/frand-node

WORKDIR ${APP_PATH}
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