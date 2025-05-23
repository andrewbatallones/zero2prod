#! /usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
    echo >& "Error: psql is not install."
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    echo >&2 "Use: " # For some reason having a space here fixes syntax highlighting
    echo >&2 "    cargo install --version='~0.6' sqlx-cli \
--no-default-features --features rustls,postgres"
    echo >&2 "to isntall it."
    exit 1
fi

# Check if a custom user has been set, otherwise set it to 'newsletter'
DB_USER=${POSTGRES_USER:=newsletter}

# Check if a custom password has been set, otherwise set it to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"

# Check if a custom database name as been set, otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"

# Check if a cuustom port has been set, otherwise default to '5432'
DB_PORT="${POSTGRES_PORT:=5432}"

# Check if a custom host has been set, otherwise default to 'localhost'
DB_HOST="${POSTGRES_HOST:=localhost}"

if [[ -z "${SKIP_DOCKER}" ]]
then
    docker compose up -d
fi

# Ping Postgres until it is ready
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done

>&2 echo "Postgres is up and runnin on  port ${DB_PORT}!"

sqlx database create --database-url ${DATABASE_URL}
sqlx migrate run --database-url ${DATABASE_URL}
