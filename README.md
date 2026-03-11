# Hestia

Hestia is a self-hosted family utility application for receipt tracking, with plans for recipe cataloging and grocery management. It runs on your own infrastructure behind Tailscale — no public internet exposure.

## Tech Stack

- **Backend**: Rust (Axum + sqlx + tokio) — RESTful JSON API
- **Frontend**: SvelteKit (SPA mode, static adapter) — builds to static files, no Node runtime in production
- **Database**: PostgreSQL 16 with pgvector extension
- **Auth**: WebAuthn (passkeys) with password fallback, server-side sessions
- **Infrastructure**: Podman quadlets on Debian LXC, Caddy reverse proxy, Tailscale HTTPS

## Project Structure

```
hestia/
├── api/             # Rust backend (Axum)
│   ├── src/         # Application source
│   ├── migrations/  # sqlx database migrations
│   └── tests/       # Integration tests
├── web/             # SvelteKit frontend (SPA)
├── docker/          # Dockerfiles + docker-compose for local dev
├── deploy/          # Production Podman quadlet files + backup script
├── Caddyfile        # Development reverse proxy config
├── justfile         # Task runner recipes
└── ROADMAP.md       # Architecture decisions and development plan
```

## Local Development

See the full [Development Guide](docs/development.md) for detailed setup instructions.

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) (v20+)
- [Docker](https://docs.docker.com/get-docker/) & Docker Compose (for Postgres)
- [just](https://github.com/casey/just) (command runner)

### Quick Start

```bash
# 1. Clone and run initial setup (installs npm deps, creates .env files)
git clone <repo-url> && cd hestia
just setup

# 2. Start the Postgres database
just db

# 3. Run database migrations
just db-migrate

# 4. Create a user account
just seed myuser "My Name" mypassword

# 5. Start the API and frontend (in separate terminals)
just dev-api    # API at http://localhost:8080
just dev-web    # Frontend at http://localhost:5173
```

**Note:** The frontend makes API calls to relative `/api` paths. When running the API and frontend as separate dev servers, you need a Vite proxy to forward requests — see the [Development Guide](docs/development.md#option-b-individual-services-recommended-for-active-development) for details. The full Docker stack (`just up`) handles this automatically via Caddy.

### Commands

Run `just` to see all available recipes. See the [Development Guide](docs/development.md#command-reference) for the full list with underlying shell commands (for use without `just`).

| Command | Description |
|---------|-------------|
| `just setup` | Install dependencies and create `.env` files from examples |
| `just db` | Start Postgres via Docker Compose |
| `just db-stop` | Stop Postgres |
| `just db-migrate` | Run sqlx database migrations |
| `just dev-api` | Start the Rust API server (port 8080) |
| `just dev-web` | Start the SvelteKit dev server (port 5173) |
| `just seed <user> <name> <pass>` | Create a user account |
| `just test` | Run all tests (API + frontend) |
| `just test-api` | Run Rust tests (`--test-threads=1`) |
| `just test-web` | Run Vitest frontend tests |
| `just lint` | Run clippy + svelte-check |
| `just check` | Run all checks (lint + build) |
| `just build` | Build everything (API + frontend) |
| `just up` | Build and start all Docker containers |
| `just down` | Stop all Docker containers |
| `just logs` | View container logs (supports args, e.g. `just logs -f`) |
| `just restart <service>` | Rebuild and restart a specific service |
| `just clean` | Remove all build artifacts |

### Full-Stack Docker (Local)

To run the entire stack locally in Docker (Postgres + API + Caddy serving frontend):

```bash
# Build and start all containers
just up

# Access the app at http://localhost
# View logs
just logs -f

# Stop everything
just down
```

## Configuration

The application is configured via environment variables. Two `.env` files are used:

| File | Used By | Purpose |
|------|---------|---------|
| `.env` | Docker Compose | Container-level environment (DB credentials, etc.) |
| `api/.env` | `just dev-api` | API server when running outside Docker |

### Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `DATABASE_URL` | Yes | — | PostgreSQL connection string |
| `SESSION_SECRET` | Yes | — | Secret for signing session cookies (generate with `openssl rand -hex 32`) |
| `PORT` | No | `8080` | API server listen port |
| `STORAGE_PATH` | No | `./uploads` | Directory for uploaded receipt photos |
| `POSTGRES_DB` | No | `hestia` | Database name (used by Docker Compose) |
| `POSTGRES_USER` | No | `hestia` | Database user (used by Docker Compose) |
| `POSTGRES_PASSWORD` | No | — | Database password (used by Docker Compose) |

## Deployment

Hestia is designed for self-hosted deployment on a Debian LXC (Proxmox) using Podman systemd quadlets and Tailscale for private HTTPS access.

See the full [Deployment Guide](deploy/README.md) for production setup instructions.

### Deployment Overview

```
Internet ──X──  (no public exposure)

Tailscale Network
  └── Proxmox Host
       └── Debian LXC
            ├── Caddy (ports 80/443) ── Tailscale HTTPS certs
            │   ├── /api/*  → hestia-api:8080
            │   └── /*      → static frontend files
            ├── hestia-api (Rust binary)
            └── PostgreSQL 16 + pgvector
```

- **Images**: CI builds and pushes `hestia-api` and `hestia-caddy` images to `ghcr.io/Zighome24` on every push to `main`
- **Networking**: All access is through Tailscale — the app is not exposed to the public internet
- **TLS**: Caddy automatically provisions HTTPS certificates via the Tailscale certificate authority
- **Data**: All persistent data lives under `/var/lib/hestia/` (Postgres data, uploads, Caddy certs, backups)
- **Backups**: Daily automated backups of the database and uploads with 7-day retention (see [deploy/README.md](deploy/README.md#backups))

### Updating a Production Deployment

```bash
podman pull ghcr.io/zighome24/hestia-api:latest
podman pull ghcr.io/zighome24/hestia-caddy:latest
systemctl restart hestia-caddy
```

## Usage

### Authentication

Hestia supports two authentication methods:

1. **Passkeys (WebAuthn)** — the primary method. Works with 1Password, iCloud Keychain, platform authenticators, and hardware security keys. Register a passkey from the settings page after initial login.
2. **Username/Password** — fallback for initial setup or when passkeys aren't available.

Create user accounts with the seed command:

```bash
just seed username "Display Name" password
```

### Receipts

- Upload receipt photos (JPEG/PNG) with amount, date, card, and optional notes
- Organize with categories/tags
- Filter and sort by date, card, or category
- All receipts are shared across all users (family view) — the uploader is tracked

### Credit Cards

- Add cards from the Settings page (nickname + last four digits)
- Cards are per-user and used when logging receipts
- Each card tracks which user created it

## CI/CD

GitHub Actions runs on every push and PR:

1. **api-check** — `cargo check`, `cargo clippy`, `cargo test`
2. **web-build** — `npm run check`, `vitest run`, `npm run build`
3. **build-and-push** (main branch only) — builds Docker images and pushes to GHCR with `latest` + SHA tags

## License

MIT License — see [LICENSE](LICENSE) for details.
