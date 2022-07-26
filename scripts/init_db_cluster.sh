#!/usr/bin/env bash
set -x
set -eo pipefail

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=indexer_db}"
DB_HOST="${POSTGRES_HOST:=db}"
DB_PORT="${POSTGRES_PORT:=5432}"

export PGPASSWORD="$DB_PASSWORD"
until psql --host "${DB_HOST}" --username "${DB_USER}" --port "${DB_PORT}" --dbname "postgres" --command '\q'; do
  echo >&2 "Postgres is still unavailable - sleeping"
  sleep 1
done

echo >&2 "Postgres is up and running on port ${DB_PORT}!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run
