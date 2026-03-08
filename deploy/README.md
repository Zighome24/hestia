# Hestia Deployment

Production deployment on a Debian LXC (Proxmox) using Podman quadlet files and Tailscale for private HTTPS access.

## Prerequisites

- **Debian 12+ LXC** on Proxmox (unprivileged or privileged)
- **Podman 4.4+** (for quadlet support)
- **Tailscale** installed and authenticated

```bash
# Install Podman
apt update && apt install -y podman

# Install Tailscale
curl -fsSL https://tailscale.com/install.sh | sh
tailscale up
```

## Directory Layout

```
/var/lib/hestia/
  hestia.env          # environment variables (secrets)
  postgres/           # PostgreSQL data
  uploads/            # receipt photo uploads
  caddy/data/         # Caddy TLS certificates
  caddy/config/       # Caddy runtime config
  backups/            # database & upload backups
```

## Initial Setup

### 1. Create directories

```bash
mkdir -p /var/lib/hestia/{postgres,uploads,caddy/data,caddy/config,backups}
```

### 2. Configure environment

```bash
cp deploy/hestia.env.example /var/lib/hestia/hestia.env
# Edit the file — set strong passwords and a session secret:
#   openssl rand -hex 32   # for SESSION_SECRET
#   openssl rand -base64 24  # for POSTGRES_PASSWORD
nano /var/lib/hestia/hestia.env
chmod 600 /var/lib/hestia/hestia.env
```

### 3. Configure Caddy for your Tailnet

Edit `deploy/Caddyfile.prod` and replace `hestia.YOUR_TAILNET.ts.net` with your actual Tailscale machine hostname. You can find it with:

```bash
tailscale status --self
```

Then place the Caddyfile where your Caddy container image expects it (typically baked into the image at build time via `docker/Dockerfile.web`).

### 4. Install quadlet files

Copy the quadlet files into Podman's systemd generator directory:

```bash
cp deploy/hestia.network /etc/containers/systemd/
cp deploy/hestia-postgres.container /etc/containers/systemd/
cp deploy/hestia-api.container /etc/containers/systemd/
cp deploy/hestia-caddy.container /etc/containers/systemd/

# Reload systemd so it picks up the new units
systemctl daemon-reload
```

### 5. Pull images and start

```bash
# Pull latest images
podman pull docker.io/pgvector/pgvector:pg16
podman pull ghcr.io/zighome24/hestia-api:latest
podman pull ghcr.io/zighome24/hestia-caddy:latest

# Start all services (postgres starts first due to dependencies)
systemctl start hestia-caddy
```

Systemd will start the dependency chain: postgres -> api -> caddy.

### 6. Verify

```bash
systemctl status hestia-postgres hestia-api hestia-caddy
podman ps
curl -k https://hestia.YOUR_TAILNET.ts.net/api/health
```

## Updating

When new images are pushed to `ghcr.io`:

```bash
podman pull ghcr.io/zighome24/hestia-api:latest
podman pull ghcr.io/zighome24/hestia-caddy:latest
systemctl restart hestia-caddy
```

## Backups

The `backup.sh` script dumps PostgreSQL and archives uploads, keeping 7 days of history.

### Manual backup

```bash
# Source the env file so the script has POSTGRES_USER and POSTGRES_DB
set -a && source /var/lib/hestia/hestia.env && set +a
bash deploy/backup.sh
```

### Automated daily backup via cron

```bash
# Add a cron job (runs at 02:00 daily)
cat > /etc/cron.d/hestia-backup << 'CRON'
0 2 * * * root set -a && . /var/lib/hestia/hestia.env && /opt/hestia/deploy/backup.sh >> /var/log/hestia-backup.log 2>&1
CRON
```

### Restoring from backup

```bash
# Restore PostgreSQL
gunzip -c /var/lib/hestia/backups/postgres-YYYYMMDD-HHMMSS.sql.gz \
  | podman exec -i hestia-postgres psql -U hestia -d hestia

# Restore uploads
tar xzf /var/lib/hestia/backups/uploads-YYYYMMDD-HHMMSS.tar.gz \
  -C /var/lib/hestia/
```

## Troubleshooting

```bash
# View container logs
podman logs hestia-postgres
podman logs hestia-api
podman logs hestia-caddy

# Check systemd journal
journalctl -u hestia-api --since "10 minutes ago"

# Enter a container
podman exec -it hestia-api /bin/sh
podman exec -it hestia-postgres psql -U hestia -d hestia
```
