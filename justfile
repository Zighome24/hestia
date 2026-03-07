# Hestia - Family Task Management Platform
# Run `just` to see all available recipes

set dotenv-load

# Default: list all recipes
default:
    @just --list

# --- Development ---

# Start the Postgres database via Docker Compose
db:
    docker compose -f docker/docker-compose.yml up -d postgres

# Stop the Postgres database
db-stop:
    docker compose -f docker/docker-compose.yml down

# Run database migrations
db-migrate:
    cd api && cargo sqlx migrate run

# Start the API server (requires Postgres running)
dev-api:
    cd api && cargo run

# Start the SvelteKit dev server
dev-web:
    cd web && npm run dev

# --- Testing ---

# Run all tests
test: test-api test-web

# Run Rust API tests
test-api:
    cd api && cargo test -- --test-threads=1

# Run SvelteKit frontend tests
test-web:
    cd web && npm run test

# --- Linting & Checks ---

# Run all checks (lint + type check + build)
check: lint build

# Run linters (clippy + svelte-check)
lint: lint-api lint-web

# Run cargo clippy
lint-api:
    cd api && cargo clippy -- -D warnings

# Run svelte-check
lint-web:
    cd web && npm run check

# --- Building ---

# Build everything
build: build-api build-web

# Build the Rust API
build-api:
    cd api && cargo build

# Build the SvelteKit frontend
build-web:
    cd web && npm run build

# --- Docker (full stack) ---

# Build and start all containers
up:
    docker compose -f docker/docker-compose.yml up -d --build

# Stop all containers
down:
    docker compose -f docker/docker-compose.yml down

# View logs from all containers
logs *args='':
    docker compose -f docker/docker-compose.yml logs {{ args }}

# Rebuild and restart a specific service
restart service:
    docker compose -f docker/docker-compose.yml up -d --build {{ service }}

# --- Setup ---

# Initial project setup (install deps, copy env)
setup:
    @echo "Setting up Hestia..."
    @test -f .env || (cp .env.example .env && echo "Created .env from .env.example")
    @test -f api/.env || (cp api/.env.example api/.env && echo "Created api/.env from api/.env.example")
    cd web && npm install
    @echo "Setup complete! Run 'just db' to start Postgres, then 'just dev-api' and 'just dev-web'."

# --- Cleanup ---

# Clean all build artifacts
clean:
    cd api && cargo clean
    cd web && rm -rf build .svelte-kit node_modules
