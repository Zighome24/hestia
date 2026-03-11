# Development Guide

This guide covers setting up and running Hestia locally for development.

## Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| [Rust](https://rustup.rs/) | stable | Backend compilation |
| [Node.js](https://nodejs.org/) | v20+ | Frontend build tooling |
| [Docker](https://docs.docker.com/get-docker/) | Latest | Running PostgreSQL (and optionally the full stack) |
| [just](https://github.com/casey/just) | Latest | Task runner (like `make` but simpler) |
| [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) | Latest | Database migrations (`cargo install sqlx-cli`) |

## Initial Setup

```bash
# Clone the repo
git clone <repo-url> && cd hestia

# Run the setup recipe — installs npm deps, creates .env files from examples
just setup
```

This creates two `.env` files:

- **`.env`** — used by Docker Compose (DB credentials, container environment)
- **`api/.env`** — used by the API when running outside Docker (points to `localhost`)

Review both files and set a real `SESSION_SECRET`:

```bash
# Generate a session secret
openssl rand -hex 32
```

## Running Locally

There are two ways to develop: **full Docker stack** or **individual services**.

### Option A: Full Docker Stack (Recommended for Integration Testing)

Runs Postgres, the API, and Caddy (serving the frontend) all in Docker:

```bash
just up          # Build and start all containers
                 # App available at http://localhost

just logs -f     # Follow container logs
just down        # Stop everything
```

This is the closest to production. Caddy serves the built frontend and proxies `/api/*` to the API container.

### Option B: Individual Services (Recommended for Active Development)

Run Postgres in Docker but the API and frontend natively for hot-reload:

```bash
# Terminal 1 — Start Postgres
just db

# Terminal 2 — Run database migrations, then start the API
just db-migrate
just dev-api     # Starts on http://localhost:8080

# Terminal 3 — Start the frontend dev server
just dev-web     # Starts on http://localhost:5173
```

**Important:** The frontend makes API calls to relative `/api` paths. In this setup, requests from the Vite dev server (`localhost:5173`) won't automatically reach the API on `localhost:8080`. You have two options:

1. **Add a Vite proxy** (recommended) — add this to `web/vite.config.ts`:

   ```ts
   export default defineConfig({
     plugins: [sveltekit()],
     server: {
       proxy: {
         '/api': 'http://localhost:8080'
       }
     },
     // ... test config
   });
   ```

2. **Use the Docker stack** for integration testing and the individual servers only for isolated API or frontend work.

## Creating User Accounts

Hestia doesn't have a public registration page. Create accounts via the CLI seed command:

```bash
just seed <username> "<display name>" <password>

# Example:
just seed alice "Alice Smith" mysecretpassword
```

This requires Postgres to be running and migrations to have been applied.

## Database

### PostgreSQL

Postgres 16 with the pgvector extension runs in Docker. The Docker Compose config exposes it on the default port `5432`.

```bash
just db           # Start Postgres container
just db-stop      # Stop Postgres container
just db-migrate   # Apply pending migrations
```

### Migrations

Database migrations are in `api/migrations/` and managed by sqlx. To create a new migration:

```bash
cd api && cargo sqlx migrate add <migration_name>
```

Migrations run automatically when the API starts (via `sqlx::migrate!()` in `main.rs`), but you can also run them explicitly with `just db-migrate`.

### Connecting Directly

```bash
# Via Docker
docker exec -it hestia-postgres-1 psql -U hestia -d hestia

# Or with psql if installed locally
psql postgres://hestia:changeme@localhost:5432/hestia
```

### Offline Mode (sqlx)

The API uses sqlx compile-time query checking. If you need to build without a running database (e.g., in CI), ensure `sqlx-data.json` is up to date:

```bash
cd api && cargo sqlx prepare
```

The Docker build uses `SQLX_OFFLINE=true` to avoid needing a database at build time.

## Testing

```bash
just test          # Run all tests (API + frontend)
just test-api      # Rust tests only (runs with --test-threads=1)
just test-web      # Vitest frontend tests only
```

### Backend Tests

- **Unit tests**: Co-located in each module under `#[cfg(test)]`
- **Integration tests**: In `api/tests/`, each test spins up a real test server with Postgres
- Tests run single-threaded (`--test-threads=1`) because some tests share environment variables

Backend integration tests require a running Postgres instance (`just db`).

### Frontend Tests

- **Framework**: Vitest + `@testing-library/svelte` + jsdom
- **Location**: `web/src/**/*.test.ts`
- No database or API required — tests mock the fetch API

## Linting and Type Checking

```bash
just lint          # Run all linters
just lint-api      # cargo clippy (warnings are errors)
just lint-web      # svelte-check (TypeScript type checking)
just check         # lint + build (full pre-commit check)
```

## Building

```bash
just build         # Build everything
just build-api     # cargo build (debug)
just build-web     # vite build (production static output)
```

The frontend builds to `web/build/` as static files with a `200.html` fallback for SPA routing.

## Docker Images

### Building Locally

```bash
# API image (multi-stage Rust build)
docker build -f docker/Dockerfile.api -t hestia-api .

# Frontend + Caddy image
docker build -f docker/Dockerfile.web -t hestia-caddy .
```

The API Dockerfile uses [cargo-chef](https://github.com/LukeMathWalker/cargo-chef) for efficient Docker layer caching — dependency builds are cached separately from application code.

### Image Architecture

| Image | Base | Contents |
|-------|------|----------|
| `hestia-api` | `debian:bookworm-slim` | Compiled Rust binary + CA certs |
| `hestia-caddy` | `caddy:2-alpine` | Caddy + built static frontend |

## File Storage

Uploaded receipt photos are stored on the local filesystem:

- **Development**: `api/uploads/` (relative to where the API runs)
- **Docker**: `/uploads` volume mount
- **Production**: `/var/lib/hestia/uploads/` bind mount

The storage path is configured via the `STORAGE_PATH` environment variable.

## Command Reference

All development tasks are run through [`just`](https://github.com/casey/just). If you don't have `just` installed, the table below shows the underlying shell commands you can run directly.

The justfile also sets `dotenv-load`, so environment variables from `.env` are automatically available to all recipes. If running commands manually, you'll need to source the `.env` file yourself or export the variables.

### Setup & Development

| `just` command | Shell equivalent | Description |
|----------------|-----------------|-------------|
| `just setup` | See note below | Install deps, create `.env` files |
| `just db` | `docker compose -f docker/docker-compose.yml up -d postgres` | Start Postgres |
| `just db-stop` | `docker compose -f docker/docker-compose.yml down` | Stop Postgres |
| `just db-migrate` | `cd api && cargo sqlx migrate run` | Run database migrations |
| `just dev-api` | `cd api && cargo run` | Start the API server |
| `just dev-web` | `cd web && npm run dev` | Start the SvelteKit dev server |
| `just seed <u> <n> <p>` | `cd api && cargo run --bin seed -- --username <u> --display-name "<n>" --password "<p>"` | Create a user account |

**`just setup` expands to:**
```bash
test -f .env || cp .env.example .env
test -f api/.env || cp api/.env.example api/.env
cd web && npm install
```

### Testing

| `just` command | Shell equivalent | Description |
|----------------|-----------------|-------------|
| `just test` | Runs `test-api` then `test-web` | Run all tests |
| `just test-api` | `cd api && cargo test -- --test-threads=1` | Rust backend tests |
| `just test-web` | `cd web && npm run test` | Vitest frontend tests |

### Linting & Building

| `just` command | Shell equivalent | Description |
|----------------|-----------------|-------------|
| `just lint` | Runs `lint-api` then `lint-web` | Run all linters |
| `just lint-api` | `cd api && cargo clippy -- -D warnings` | Clippy (warnings = errors) |
| `just lint-web` | `cd web && npm run check` | svelte-check (TypeScript) |
| `just check` | Runs `lint` then `build` | Full pre-commit check |
| `just build` | Runs `build-api` then `build-web` | Build everything |
| `just build-api` | `cd api && cargo build` | Build Rust API (debug) |
| `just build-web` | `cd web && npm run build` | Build frontend (production) |

### Docker (Full Stack)

| `just` command | Shell equivalent | Description |
|----------------|-----------------|-------------|
| `just up` | `docker compose -f docker/docker-compose.yml up -d --build` | Build and start all containers |
| `just down` | `docker compose -f docker/docker-compose.yml down` | Stop all containers |
| `just logs` | `docker compose -f docker/docker-compose.yml logs` | View container logs |
| `just logs -f` | `docker compose -f docker/docker-compose.yml logs -f` | Follow container logs |
| `just restart <svc>` | `docker compose -f docker/docker-compose.yml up -d --build <svc>` | Rebuild and restart a service |

### Cleanup

| `just` command | Shell equivalent | Description |
|----------------|-----------------|-------------|
| `just clean` | `cd api && cargo clean && cd ../web && rm -rf build .svelte-kit node_modules` | Remove all build artifacts |

## Project Conventions

- **Task runner**: Use `just` when possible — it handles `.env` loading and keeps commands consistent
- **Testing**: Backend tests must be single-threaded due to environment variable conflicts
- **API style**: RESTful JSON, all endpoints under `/api/`
- **Auth**: All routes except `/api/health` and `/api/auth/*` require authentication
- **Frontend**: SPA mode with static adapter — no server-side rendering
