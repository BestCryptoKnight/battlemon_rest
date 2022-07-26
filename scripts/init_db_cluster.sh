#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
  echo >&2 "to install it."
  exit 1
fi

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=indexer_db}"
DB_PORT="${POSTGRES_PORT:=5432}"

export PGPASSWORD="$DB_PASSWORD"
until psql --host "db" --username "${DB_USER}" --port "${DB_PORT}" --dbname "postgres" --command '\q'; do
  echo >&2 "Postgres is still unavailable - sleeping"
  sleep 1
done

echo >&2 "Postgres is up and running on port ${DB_PORT}!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@db:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run
