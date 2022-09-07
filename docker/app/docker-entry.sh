#!/bin/sh
MYSQL_HOST="db"
MYSQL_PORT=3306

if [ "$ROCKET_ENV" = "development" ] || [ "$ROCKET_ENV" = "test" ]; then
  until nc -z $MYSQL_HOST $MYSQL_PORT; do
    echo "MySQL is not ready, sleeping..."
    sleep 3
  done
  echo "Connected to Mysql!"
fi

exec "$@"