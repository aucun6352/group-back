FROM rust:1.60.0-slim
LABEL maintainer="aucun6352@gmail.com"
LABEL name="group"


ENV ROCKET_ADDRESS=0.0.0.0
ENV WORK_ROOT=/var/www/app

RUN mkdir -p $WORK_ROOT
WORKDIR $WORK_ROOT
VOLUME $WORK_ROOT

ENTRYPOINT ["./docker/app/docker-entry.sh"]