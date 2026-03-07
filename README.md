# Hestia

Hestia is a self-hosted family utility application. It started with simple receipt tracking and will expand to recipe cataloging, grocery management, and more.

## Tech Stack

- **Backend**: Rust + Axum + sqlx + PostgreSQL 16
- **Frontend**: SvelteKit (SPA mode, static adapter)
- **Infrastructure**: Docker Compose, Caddy reverse proxy, Tailscale

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) (v20+)
- [Docker](https://docs.docker.com/get-docker/) & Docker Compose
- [just](https://github.com/casey/just) (command runner)

## Quick Start

```bash
# Clone and setup
git clone <repo-url> && cd hestia
just setup

# Start the database
just db

# Run the API and frontend (in separate terminals)
just dev-api
just dev-web
```

## Commands

Run `just` to see all available recipes. Key commands:

### Development

| Command | Description |
|---------|-------------|
| `just setup` | Install dependencies and create `.env` files |
| `just db` | Start Postgres via Docker Compose |
| `just db-stop` | Stop Postgres |
| `just db-migrate` | Run database migrations |
| `just dev-api` | Start the Rust API server |
| `just dev-web` | Start the SvelteKit dev server |

### Testing

| Command | Description |
|---------|-------------|
| `just test` | Run all tests (API + frontend) |
| `just test-api` | Run Rust API tests |
| `just test-web` | Run SvelteKit frontend tests |

### Linting & Building

| Command | Description |
|---------|-------------|
| `just check` | Run all checks (lint + build) |
| `just lint` | Run all linters (clippy + svelte-check) |
| `just build` | Build everything |

### Docker (Full Stack)

| Command | Description |
|---------|-------------|
| `just up` | Build and start all containers |
| `just down` | Stop all containers |
| `just logs` | View container logs |
| `just logs -f` | Follow container logs |
| `just restart api` | Rebuild and restart a specific service |

## Project Structure

```
hestia/
├── api/           # Rust backend (Axum)
├── web/           # SvelteKit frontend
├── docker/        # Dockerfiles + docker-compose.yml
├── Caddyfile      # Reverse proxy config
└── justfile       # Task runner recipes
```

See [ROADMAP.md](ROADMAP.md) for the full architecture and development plan.
