#!/usr/bin/env bash
# Hestia backup script
# Backs up PostgreSQL database and uploaded receipt photos.
# Intended to be run daily via cron (see deploy/README.md).
set -euo pipefail

BACKUP_DIR="/var/lib/hestia/backups"
TIMESTAMP="$(date +%Y%m%d-%H%M%S)"
RETAIN_DAYS=7

mkdir -p "$BACKUP_DIR"

# ── PostgreSQL dump ──────────────────────────────────────────────────
echo "Backing up PostgreSQL..."
podman exec hestia-postgres \
  pg_dump -U "$POSTGRES_USER" -d "$POSTGRES_DB" --no-owner --clean \
  | gzip > "$BACKUP_DIR/postgres-$TIMESTAMP.sql.gz"
echo "  -> $BACKUP_DIR/postgres-$TIMESTAMP.sql.gz"

# ── Uploads directory ────────────────────────────────────────────────
echo "Backing up uploads..."
tar czf "$BACKUP_DIR/uploads-$TIMESTAMP.tar.gz" \
  -C /var/lib/hestia uploads
echo "  -> $BACKUP_DIR/uploads-$TIMESTAMP.tar.gz"

# ── Prune old backups ────────────────────────────────────────────────
echo "Pruning backups older than $RETAIN_DAYS days..."
find "$BACKUP_DIR" -type f \( -name "postgres-*.sql.gz" -o -name "uploads-*.tar.gz" \) \
  -mtime +"$RETAIN_DAYS" -delete

echo "Backup complete."
