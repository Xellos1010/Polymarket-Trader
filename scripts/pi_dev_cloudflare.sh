#!/usr/bin/env bash
# Install Cloudflare Tunnel on the Pi for stable remote dashboard access.
# Requires: CF_TUNNEL_TOKEN env var (from Cloudflare Zero Trust dashboard).
# Never commits the token to the repository.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=pi_lib.sh
source "$SCRIPT_DIR/pi_lib.sh"

: "${CF_TUNNEL_TOKEN:?CF_TUNNEL_TOKEN env var is required (from Cloudflare Zero Trust dashboard)}"

PI_HOST=$(resolve_pi_host)
PI_USER=$(resolve_pi_user)
PI_KEY=$(resolve_pi_key)
PI_SSH="ssh -i $PI_KEY -o StrictHostKeyChecking=no $PI_USER@$PI_HOST"

echo "==> Installing cloudflared on $PI_USER@$PI_HOST"

$PI_SSH "
  curl -fsSL https://pkg.cloudflare.com/cloudflare-main.gpg \
    | sudo tee /usr/share/keyrings/cloudflare-main.gpg > /dev/null
  echo 'deb [signed-by=/usr/share/keyrings/cloudflare-main.gpg] https://pkg.cloudflare.com/cloudflared any main' \
    | sudo tee /etc/apt/sources.list.d/cloudflared.list
  sudo apt-get update -qq
  sudo apt-get install -y cloudflared
"

echo "==> Registering tunnel"
$PI_SSH "sudo cloudflared service install '$CF_TUNNEL_TOKEN'"

echo "==> Verifying tunnel status"
$PI_SSH "cloudflared tunnel info 2>/dev/null || echo '(info unavailable — check Cloudflare Zero Trust dashboard)'"

echo "==> Cloudflare Tunnel installed."
