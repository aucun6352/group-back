FROM rust:1.63.0-slim
LABEL maintainer="aucun6352@gmail.com"
LABEL name="group"

ARG UTILS="pkg-config libssl-dev netcat"

RUN apt-get update \
    && apt-get install -y $UTILS

ENV WORK_ROOT=/var/www/app

RUN mkdir -p $WORK_ROOT
WORKDIR $WORK_ROOT
VOLUME $WORK_ROOT

COPY . .
ENTRYPOINT ["./docker/app/docker-entry.sh"]