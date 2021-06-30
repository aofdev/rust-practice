# API Practice

## Getting Started ⚡️

- Install [PostgreSQL](https://www.postgresql.org/) if you don't have it already.
* Install the [Diesel CLI](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) with the `postgres` feature enabled.

```bash
# Start docker compose
docker-compose up -d

# Copy file env
cp .env.example .env

# Setup diesel
diesel setup --database-url='postgres://postgres:9EfSjEPqwJ4uxS3CMGpcztdXjHdW8QdW@localhost:5433/db_users'

# Migration with diesel
diesel migration run

# Start API
cargo run
```

## Tests 🧪
```bash
cargo test
```