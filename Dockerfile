FROM rust:1.53-alpine3.12

RUN set -x \
  && apk upgrade --no-cache \
  && apk add --no-cache --virtual build-dependencies \
    build-base libc-dev linux-headers openssl openssl-dev bash

WORKDIR /app

COPY . /app

RUN cargo build