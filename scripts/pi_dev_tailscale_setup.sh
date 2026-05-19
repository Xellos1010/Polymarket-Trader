#!/usr/bin/env bash
# Install Tailscale auto-reauth systemd timer on the Pi.
# Requires: PI_TAILSCALE_AUTH_KEY env var (ephemeral key from Tailscale admin panel).
# Never commits the auth key to the repository.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=pi_lib.sh
source "$SCRIPT_DIR/pi_lib.sh"

: "${PI_TAILSCALE_AUTH_KEY:?PI_TAILSCALE_AUTH_KEY env var is required (ephemeral key from Tailscale admin)}"

PI_HOST=$(resolve_pi_host)
PI_USER=$(resolve_pi_user)
PI_KEY=$(resolve_pi_key)
PI_SSH="ssh -i $PI_KEY -o StrictHostKeyChecking=no $PI_USER@$PI_HOST"

echo "==> Installing Tailscale auto-reauth on $PI_USER@$PI_HOST"

# Render service file with key substituted (stored only on Pi, never in repo).
SERVICE_CONTENT=$(sed "s|\${TS_AUTH_KEY}|$PI_TAILSCALE_AUTH_KEY|g" \
  "$SCRIPT_DIR/../infra/systemd/tailscale-reauth.service.template")

$PI_SSH "sudo tee /etc/systemd/system/tailscale-reauth.service > /dev/null" <<< "$SERVICE_CONTENT"

scp -i "$PI_KEY" -o StrictHostKeyChecking=no \
  "$SCRIPT_DIR/../infra/systemd/tailscale-reauth.timer" \
  "$PI_USER@$PI_HOST:/tmp/tailscale-reauth.timer"

$PI_SSH "sudo mv /tmp/tailscale-reauth.timer /etc/systemd/system/ && \
  sudo chmod 600 /etc/systemd/system/tailscale-reauth.service && \
  sudo systemctl daemon-reload && \
  sudo systemctl enable --now tailscale-reauth.timer && \
  tailscale status | head -3"

echo "==> Tailscale auto-reauth installed and timer enabled."
