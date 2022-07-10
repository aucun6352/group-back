FROM rust:alpine3.16
LABEL maintainer="aucun6352@gmail.com"
LABEL name="group"

ARG UTILS="pkgconfig musl-dev openssl-dev"

RUN apk update \
    && apk upgrade \
    && apk add --update --no-cache $UTILS

ENV WORK_ROOT=/var/www/app

RUN mkdir -p $WORK_ROOT
WORKDIR $WORK_ROOT
VOLUME $WORK_ROOT

COPY . .
RUN cargo install --path .

ENTRYPOINT ["./docker/app/docker-entry.sh"]
