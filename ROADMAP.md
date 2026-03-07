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

### Deployment: Docker Compose on Proxmox LXC

- **Host**: Debian LXC on Proxmox
- **Networking**: Tailscale for device access (no public internet exposure)
- **Reverse proxy**: Caddy — auto-TLS (Tailscale HTTPS certs), static file serving, API proxying
- **Containers**:
  - `hestia-api` — Rust binary in distroless image
  - `postgres` — PostgreSQL 16 + pgvector extension
  - `caddy` — Reverse proxy + static frontend serving

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
├── docker/
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

- [ ] Initialize Rust project with Axum, sqlx, tokio
- [ ] Initialize SvelteKit project with static adapter
- [ ] Docker Compose with Postgres, Caddy, API container
- [ ] Basic health check endpoint (`GET /api/health`)
- [ ] Database connection and migration setup
- [ ] CI: cargo check, cargo test, clippy, svelte build

### Milestone 1.2 — Authentication

- [ ] User model and migration (id, username, display_name, password_hash, created_at)
- [ ] WebAuthn credential storage (user_id, credential_id, public_key, etc.)
- [ ] Registration flow: username + password + passkey registration
- [ ] Login flow: passkey challenge/response with password fallback
- [ ] Server-side session management with secure cookies
- [ ] Auth middleware for protected routes
- [ ] Frontend: login page, registration page, session management
- [ ] Seed script or admin CLI to create initial user accounts

### Milestone 1.3 — Credit Card Management

- [ ] Card model and migration (id, user_id, nickname, last_four, created_at)
- [ ] CRUD API endpoints for cards
- [ ] Frontend: settings page to add/edit/remove cards
- [ ] Cards are per-user (each user manages their own list)

### Milestone 1.4 — Receipt Upload & Management

- [ ] Receipt model and migration (id, user_id, card_id, total_amount, photo_path, notes, purchased_at, created_at)
- [ ] Category/tag model and migration (id, name, color, created_at; receipt_tags join table)
- [ ] Photo upload endpoint with file validation (JPEG/PNG, size limit)
- [ ] Receipt CRUD API endpoints (include category/tag assignment)
- [ ] Category/tag CRUD API endpoints
- [ ] Photo storage on local filesystem with unique naming
- [ ] Photo retrieval endpoint (serves stored images, auth-gated)
- [ ] Frontend: receipt list view (sortable by date, filterable by card and category)
- [ ] Frontend: new receipt form — camera/file input, amount, card selector, date, categories, optional notes
- [ ] Frontend: receipt detail view with photo display
- [ ] Frontend: category/tag management UI
- [ ] Mobile-optimized upload UX (camera capture via `accept="image/*" capture="environment"`)
- [ ] All receipts visible to all users (shared family view)

### Milestone 1.5 — Polish & Deploy

- [ ] Responsive design pass — ensure all views work well on phone
- [ ] Error handling and user-friendly error messages
- [ ] Loading states and optimistic UI where appropriate
- [ ] Docker production build (multi-stage Rust build, static frontend baked into Caddy image)
- [ ] Docker Compose production config (volumes, restart policies, resource limits)
- [ ] Caddy config with Tailscale HTTPS
- [ ] Deploy to Proxmox LXC
- [ ] Backup strategy for Postgres data and receipt photos

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
