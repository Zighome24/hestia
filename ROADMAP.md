# Hestia - Family Task Management Platform

## Vision

Hestia is a self-hosted family utility application. The long-term vision includes grocery tracking, recipe management, and receipt management. The initial focus is **receipt capture and organization**.

---

## Architectural Decisions

### Backend: Rust + Axum

- **Language**: Rust — for learning value and strong type safety
- **Framework**: Axum (Tower/Tokio ecosystem)
- **Why Axum**: Composable Tower middleware, clean ergonomics, strong async story, well-documented, active development
- **API style**: RESTful JSON API, cleanly separated from frontend for future iOS app reuse

### Frontend: SvelteKit (SPA mode)

- **Framework**: SvelteKit in static/SPA adapter mode
- **Why SvelteKit**: Lightweight, excellent mobile UX, small compiled output, simple component model, good documentation
- **Deployment**: Builds to static files served by Caddy (no Node runtime needed in production)

### Database: PostgreSQL 16

- **Why Postgres**: pgvector for future semantic search, full-text search, JSONB for flexible metadata, compile-time query checking via sqlx
- **Migrations**: Managed via sqlx-cli
- **ORM**: sqlx (not an ORM — query-level control with compile-time verification)

### File Storage: Local Filesystem

- Receipt photos stored on local disk, paths tracked in Postgres
- Storage directory mounted as a Docker volume for persistence and backup
- Future option: swap to S3-compatible (MinIO) behind a storage abstraction without API changes

### Authentication: WebAuthn (Passkeys) + Password Fallback

- **Primary**: Passkeys via `webauthn-rs` — both users have 1Password, which supports passkeys
- **Fallback**: Username/password for new device setup or passkey unavailability
- **Sessions**: Server-side sessions with secure cookies
- **Users**: Multi-user with per-user data isolation where appropriate (e.g., "who uploaded this receipt")

### Deployment: Podman Quadlets on Proxmox LXC

- **Host**: Debian LXC on Proxmox
- **Container runtime**: Podman (root), managed via systemd quadlet files
- **Image registry**: `ghcr.io/Zighome24` — CI pushes images, LXC pulls them
- **Networking**: Tailscale for device access (no public internet exposure)
- **Reverse proxy**: Caddy — auto-TLS (Tailscale HTTPS certs), static file serving, API proxying
- **Containers** (each a `.container` quadlet):
  - `hestia-api` — Rust binary in distroless image
  - `postgres` — PostgreSQL 16 + pgvector extension
  - `caddy` — Reverse proxy + static frontend serving
- **Data**: Bind mounts under `/var/lib/hestia/` (postgres, uploads, caddy) for straightforward backups
- **Local dev**: Docker Compose (`docker/docker-compose.yml`) retained for local development and testing — works with both Docker and `podman compose`

### Security Considerations

- All traffic encrypted via Tailscale + Caddy TLS
- No public internet exposure
- WebAuthn for phishing-resistant authentication
- Password hashing via argon2 (for fallback auth)
- CSRF protection on all state-changing endpoints
- Input validation and sanitization on all API inputs
- File upload validation (type checking, size limits)
- SQL injection prevention via sqlx parameterized queries
- Rate limiting on auth endpoints
- Secure session cookies (HttpOnly, Secure, SameSite)

---

## Code Organization

```
hestia/
├── api/                      # Rust backend (Cargo project)
│   ├── src/
│   │   ├── main.rs           # Server bootstrap, router setup
│   │   ├── config.rs         # Environment/config loading
│   │   ├── routes/           # Axum route handlers
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs       # Login, register, passkey endpoints
│   │   │   ├── receipts.rs   # Receipt CRUD + photo upload
│   │   │   └── cards.rs      # Credit card management
│   │   ├── models/           # Database models + queries
│   │   │   ├── mod.rs
│   │   │   ├── user.rs
│   │   │   ├── receipt.rs
│   │   │   └── card.rs
│   │   ├── auth/             # WebAuthn + password + session logic
│   │   │   ├── mod.rs
│   │   │   ├── webauthn.rs
│   │   │   ├── password.rs
│   │   │   └── session.rs
│   │   ├── storage/          # File storage abstraction
│   │   │   ├── mod.rs
│   │   │   └── local.rs
│   │   └── error.rs          # Unified error handling
│   ├── migrations/           # sqlx migrations
│   └── Cargo.toml
├── web/                      # SvelteKit frontend
│   ├── src/
│   │   ├── routes/           # SvelteKit pages
│   │   │   ├── +layout.svelte
│   │   │   ├── +page.svelte          # Dashboard / home
│   │   │   ├── login/
│   │   │   ├── receipts/
│   │   │   │   ├── +page.svelte      # Receipt list
│   │   │   │   ├── new/+page.svelte  # Upload receipt
│   │   │   │   └── [id]/+page.svelte # Receipt detail
│   │   │   └── settings/
│   │   │       └── cards/            # Manage credit cards
│   │   └── lib/
│   │       ├── api.ts                # API client
│   │       ├── auth.ts               # WebAuthn client helpers
│   │       └── components/           # Shared UI components
│   ├── static/
│   ├── svelte.config.js
│   └── package.json
├── deploy/                   # Production deployment (Podman quadlets)
│   ├── hestia.network        # Podman network for inter-container comms
│   ├── hestia-api.container  # API container quadlet
│   ├── hestia-postgres.container
│   ├── hestia-caddy.container
│   └── hestia-api.env        # Production environment variables (not committed)
├── docker/                   # Local development
│   ├── Dockerfile.api
│   ├── Dockerfile.web        # Multi-stage: build + copy to Caddy
│   └── docker-compose.yml
├── Caddyfile
└── ROADMAP.md
```

---

## Phase 1: Receipt Capture (MVP)

The goal is a working app where both users can log in, upload receipt photos, record the total, and tag which credit card was used.

### Milestone 1.1 — Project Scaffolding

- [x] Initialize Rust project with Axum, sqlx, tokio
- [x] Initialize SvelteKit project with static adapter
- [x] Docker Compose with Postgres, Caddy, API container
- [x] Basic health check endpoint (`GET /api/health`)
- [x] Database connection and migration setup (pool + `sqlx::migrate!`; migration files added per milestone)
- [x] CI: cargo check, cargo test, clippy, svelte check, svelte build
- [x] Vitest setup in web/ with `@testing-library/svelte`
- [x] Integration test: health check endpoint returns `{"status": "ok"}`
- [x] Unit tests: config parsing (valid env, missing vars, defaults)

### Milestone 1.2 — Authentication

- [x] User model and migration (id, username, display_name, password_hash, created_at)
- [x] WebAuthn credential storage (user_id, credential_id, public_key, etc.)
- [x] Registration flow: username + password + passkey registration
- [x] Login flow: passkey challenge/response with password fallback
- [x] Server-side session management with secure cookies
- [x] Auth middleware for protected routes
- [x] Frontend: login page, registration page, session management
- [x] Seed script or admin CLI to create initial user accounts
- [x] Unit tests: password hashing/verification, session token generation
- [x] Integration tests: register → login → access protected route → logout
- [x] Integration tests: unauthenticated requests get 401
- [x] Frontend tests: login form renders and validates

### Milestone 1.3 — Credit Card Management

- [x] Card model and migration (id, user_id, nickname, last_four, created_at)
- [x] CRUD API endpoints for cards
- [x] Frontend: settings page to add/edit/remove cards
- [x] Cards are per-user (each user manages their own list)
- [x] Integration tests: card CRUD lifecycle (create, read, update, delete)
- [x] Integration tests: cards scoped to authenticated user
- [x] Unit tests: card validation (last_four format, required fields)
- [x] Frontend tests: card management form renders and validates

### Milestone 1.4 — Receipt Upload & Management

- [x] Receipt model and migration (id, user_id, card_id, total_amount, photo_path, notes, purchased_at, created_at)
- [x] Category/tag model and migration (id, name, color, created_at; receipt_tags join table)
- [x] Photo upload endpoint with file validation (JPEG/PNG, size limit)
- [x] Receipt CRUD API endpoints (include category/tag assignment)
- [x] Category/tag CRUD API endpoints
- [x] Photo storage on local filesystem with unique naming
- [x] Photo retrieval endpoint (serves stored images, auth-gated)
- [x] Frontend: receipt list view (sortable by date, filterable by card and category)
- [x] Frontend: new receipt form — camera/file input, amount, card selector, date, categories, optional notes
- [x] Frontend: receipt detail view with photo display
- [x] Frontend: category/tag management UI
- [x] Mobile-optimized upload UX (camera capture via `accept="image/*" capture="environment"`)
- [x] All receipts visible to all users (shared family view)
- [x] Integration tests: receipt CRUD lifecycle (create with photo, read, update, delete)
- [x] Integration tests: file upload validation (valid JPEG/PNG accepted, oversized/wrong-type rejected)
- [x] Integration tests: category/tag CRUD and assignment to receipts
- [x] Integration tests: photo retrieval requires authentication
- [x] Unit tests: storage file naming, content-type validation
- [x] Frontend tests: receipt upload form renders required fields and validates
- [x] Frontend tests: receipt list renders and filters correctly

### Milestone 1.5 — Polish & Deploy

- [x] Responsive design pass — ensure all views work well on phone
- [x] Error handling and user-friendly error messages
- [x] Loading states and optimistic UI where appropriate
- [x] Docker production build (multi-stage Rust build, static frontend baked into Caddy image)
- [x] CI: build and push container images to `ghcr.io/Zighome24` on main branch
- [x] Podman quadlet files (`deploy/`) — `.container`, `.network` units for production
- [x] Caddy config with Tailscale HTTPS
- [ ] Deploy quadlets to Proxmox LXC, verify systemd starts all services on boot
- [x] Backup strategy for Postgres data and receipt photos (bind mounts under `/var/lib/hestia/`)

---

## Future Phases (Not in Scope for Phase 1)

### Phase 2: Receipt OCR & Data Extraction

- On-device or server-side OCR to extract line items, totals, dates, merchant names
- Automatic field population from photo
- Searchable receipt content

### Phase 3: Grocery & Recipe Management

- Recipe storage with ingredient lists
- Grocery list generation from recipes
- Shared family grocery list
- Store/aisle organization

### Phase 4: iOS Native App

- Swift/SwiftUI iOS app
- Reuse Hestia API backend
- Native camera integration for receipt capture
- On-device OCR via Apple Vision framework
- Push notifications for shared lists

### Phase 5: Analytics & Insights

- Spending trends by card, category, time period
- Semantic search across receipts (pgvector)
- Budget tracking and alerts

---

## Testing Strategy

### Backend (Rust)

- **Framework**: Built-in `#[tokio::test]` + `axum::test` helpers
- **Unit tests**: Co-located in each module (`#[cfg(test)] mod tests`)
  - Model layer: query builders, validation logic, error mapping
  - Auth: password hashing/verification, session token generation, WebAuthn challenge construction
  - Storage: file naming, path resolution, content-type validation
  - Config: env parsing, defaults, missing-var errors
- **Integration tests**: `api/tests/` directory, each test spins up a real test server + Postgres
  - Health check returns 200
  - Auth flows: register → login → access protected route → logout
  - CRUD lifecycle for cards, receipts, categories/tags
  - File upload: valid image accepted, oversized/wrong-type rejected
  - Auth middleware: unauthenticated requests get 401, wrong-user access gets 403
  - Error responses: 404 for missing resources, 400 for invalid input, proper JSON shape
- **Database**: Use a dedicated test database (e.g., `hestia_test`), run migrations before each test suite, wrap each test in a transaction that rolls back
- **Test utilities**: Shared helpers in `api/tests/common/` for creating test users, authenticated clients, and seeding data

### Frontend (SvelteKit)

- **Framework**: Vitest + `@testing-library/svelte`
- **Unit tests**: `web/src/lib/**/*.test.ts`
  - API client: mock fetch, verify correct URL/method/headers, error handling for non-2xx
  - Auth helpers: WebAuthn credential formatting, challenge encoding
- **Component tests**: `web/src/routes/**/*.test.ts`
  - Key forms render required fields (login, receipt upload, card management)
  - Form validation: required fields, amount format, file type restrictions
  - Navigation: layout renders nav links, active state
- **No E2E tests in Phase 1** — deferred to Phase 2 (Playwright)

### CI Requirements

- All backend tests must pass (`cargo test`)
- All frontend tests must pass (`npm run test`)
- Clippy clean (`cargo clippy -- -D warnings`)
- SvelteKit type check passes (`npm run check`)
- SvelteKit build succeeds (`npm run build`)

### Per-Milestone Test Requirements

Each milestone's tasks include writing tests for the functionality introduced. Specifically:

- **Milestone 1.1**: Health check integration test, config unit tests
- **Milestone 1.2**: Auth unit tests (password hashing, session management), auth integration tests (register, login, logout, middleware rejection)
- **Milestone 1.3**: Card CRUD integration tests, card validation unit tests
- **Milestone 1.4**: Receipt CRUD integration tests, file upload integration tests (valid/invalid), category CRUD integration tests, storage unit tests
- **Milestone 1.5**: No new tests — focus on manual QA and responsive testing

---

## Key Dependencies (Rust Crates)

| Crate | Purpose |
|-------|---------|
| `axum` | Web framework |
| `tokio` | Async runtime |
| `sqlx` | Database access (Postgres, compile-time checked) |
| `webauthn-rs` | WebAuthn/passkey authentication |
| `argon2` | Password hashing |
| `tower-http` | CORS, tracing, compression middleware |
| `serde` / `serde_json` | Serialization |
| `uuid` | ID generation |
| `tracing` / `tracing-subscriber` | Structured logging |
| `dotenvy` | Environment configuration |

---

## Resolved Decisions

- **Shared receipts**: All receipts are visible to all users. The `user_id` on a receipt tracks who uploaded it, but all users can view and search all receipts.
- **Receipt categories/tags**: Included in Phase 1. Users can create categories/tags and assign them to receipts for organization and filtering.
- **Backup automation**: Deferred — not in scope for Phase 1.
