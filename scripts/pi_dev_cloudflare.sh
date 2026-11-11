#!/usr/bin/env bash
# Manage Cloudflare Tunnel on the Raspberry Pi for remote dashboard access.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=pi_lib.sh
source "$SCRIPT_DIR/pi_lib.sh"

PI_CLOUDFLARE_USER="${PI_CLOUDFLARE_USER:-$PI_USER}"
PI_CLOUDFLARE_KEY="${PI_CLOUDFLARE_KEY:-}"
PI_CLOUDFLARE_MODE="${PI_CLOUDFLARE_MODE:-auto}"
PI_CLOUDFLARE_SERVICE="${PI_CLOUDFLARE_SERVICE:-cloudflared}"
PI_CLOUDFLARE_CONFIG="${PI_CLOUDFLARE_CONFIG:-/etc/cloudflared/config.yml}"
PI_CLOUDFLARE_LOCAL_URL="${PI_CLOUDFLARE_LOCAL_URL:-http://127.0.0.1:${PI_PUBLIC_PORT:-8080}}"
PI_CLOUDFLARE_TUNNEL_SERVICE="${PI_CLOUDFLARE_TUNNEL_SERVICE:-pt-dashboard-cloudflare-tunnel}"
PI_CLOUDFLARE_TUNNEL_TEMPLATE="${PI_CLOUDFLARE_TUNNEL_TEMPLATE:-$SCRIPT_DIR/../infra/systemd/pt-dashboard-cloudflare-tunnel.service.template}"
PI_INFRA_RENDER="${PI_INFRA_RENDER:-$SCRIPT_DIR/infra_render.sh}"

usage() {
  cat <<'EOF'
Usage: scripts/pi_dev_cloudflare.sh <command>

Commands:
  ensure         Install/enable cloudflared using the selected mode and print status.
  status         Print installation, service, URL, and config summary.
  url            Print public Cloudflare URLs when they can be derived.
  print-config   Print the effective local Cloudflare config (sanitized).

Environment:
  PI_HOST                 Static host or IP for the Pi.
  PI_USER                 Default Pi SSH user from pi_lib.sh.
  PI_CLOUDFLARE_USER      SSH user for Cloudflare admin actions.
                          Must have passwordless sudo for ensure/status config reads.
                          Defaults to PI_USER.
  PI_CLOUDFLARE_KEY       Optional SSH key path for PI_CLOUDFLARE_USER.
                          Defaults to PI_KEY resolution from pi_lib.sh.
  PI_CLOUDFLARE_MODE      auto | anon-systemd | managed-service (default: auto).
  PI_CLOUDFLARE_SERVICE   Systemd service name (default: cloudflared).
  PI_CLOUDFLARE_CONFIG    Config path on the Pi (default: /etc/cloudflared/config.yml).
  PI_CLOUDFLARE_LOCAL_URL Origin exposed through Cloudflare (default: http://127.0.0.1:8080).
  PI_CLOUDFLARE_TUNNEL_SERVICE   Local tunnel adapter service name
                                 (default: pt-dashboard-cloudflare-tunnel).
  PI_CLOUDFLARE_TUNNEL_TEMPLATE  Local systemd template used for anon-systemd mode.
  CF_TUNNEL_TOKEN         Required for `ensure` when the service is not already installed.
EOF
}

resolve_cloudflare_key() {
  local key
  if [[ -n "$PI_CLOUDFLARE_KEY" ]]; then
    key="$PI_CLOUDFLARE_KEY"
  else
    key="$(resolve_pi_key)"
  fi
  printf '%s' "$key"
}

cloudflare_ssh() {
  local host="$1"
  shift
  local key
  key="$(resolve_cloudflare_key)"
  local opts=(
    -i "$key"
    -p "$PI_PORT"
    -o IdentitiesOnly=yes
    -o StrictHostKeyChecking=no
    -o UserKnownHostsFile=/dev/null
    -o ConnectTimeout="$SSH_CONNECT_TIMEOUT"
    -o ServerAliveInterval=30
    -o ServerAliveCountMax=3
  )
  ssh "${opts[@]}" "${PI_CLOUDFLARE_USER}@${host}" "$@"
}

cloudflare_scp() {
  local key
  key="$(resolve_cloudflare_key)"
  local opts=(
    -i "$key"
    -P "$PI_PORT"
    -o IdentitiesOnly=yes
    -o StrictHostKeyChecking=no
    -o UserKnownHostsFile=/dev/null
    -o ConnectTimeout="$SSH_CONNECT_TIMEOUT"
    -o ServerAliveInterval=30
    -o ServerAliveCountMax=3
  )
  scp "${opts[@]}" "$@"
}

require_remote_sudo() {
  local host="$1"
  if ! cloudflare_ssh "$host" "sudo -n true" >/dev/null 2>&1; then
    echo "[pi-cloudflare] ${PI_CLOUDFLARE_USER}@${host} needs passwordless sudo for Cloudflare admin actions." >&2
    echo "[pi-cloudflare] Set PI_CLOUDFLARE_USER to a sudo-capable account and retry." >&2
    return 1
  fi
}

cloudflare_unit_exists() {
  local host="$1"
  local service_name="$2"
  cloudflare_ssh "$host" "sudo -n systemctl cat '${service_name}' >/dev/null 2>&1"
}

remote_tunnel_service_stage() {
  printf '/tmp/%s-%s.service' "$PI_CLOUDFLARE_USER" "$PI_CLOUDFLARE_TUNNEL_SERVICE"
}

render_cloudflare_tunnel_unit() {
  local out_path="$1"
  if [[ ! -x "$PI_INFRA_RENDER" ]]; then
    echo "missing infra renderer: $PI_INFRA_RENDER" >&2
    exit 1
  fi
  "$PI_INFRA_RENDER" render-cloudflare-unit "$out_path"
}

install_cloudflared_if_missing() {
  local host="$1"
  if cloudflare_ssh "$host" "command -v cloudflared >/dev/null 2>&1"; then
    return 0
  fi

  echo "==> Installing cloudflared on ${PI_CLOUDFLARE_USER}@${host}"
  cloudflare_ssh "$host" "bash -s" <<'REMOTE'
set -euo pipefail
curl -fsSL https://pkg.cloudflare.com/cloudflare-main.gpg \
  | sudo -n tee /usr/share/keyrings/cloudflare-main.gpg > /dev/null
echo 'deb [signed-by=/usr/share/keyrings/cloudflare-main.gpg] https://pkg.cloudflare.com/cloudflared any main' \
  | sudo -n tee /etc/apt/sources.list.d/cloudflared.list > /dev/null
sudo -n apt-get update -qq
sudo -n apt-get install -y cloudflared
REMOTE
}

ensure_anonymous_tunnel_service() {
  local host="$1"
  install_cloudflared_if_missing "$host"

  local rendered_unit
  rendered_unit="$(mktemp "${TMPDIR:-/tmp}/pt-cloudflare-tunnel.XXXXXX")"
  render_cloudflare_tunnel_unit "$rendered_unit"

  local remote_stage
  remote_stage="$(remote_tunnel_service_stage)"
  cloudflare_scp "$rendered_unit" "${PI_CLOUDFLARE_USER}@${host}:${remote_stage}"

  cloudflare_ssh "$host" \
    PI_TUNNEL_STAGE="$remote_stage" \
    PI_TUNNEL_SERVICE="$PI_CLOUDFLARE_TUNNEL_SERVICE" \
    bash -s <<'REMOTE'
set -euo pipefail
sudo -n cp "$PI_TUNNEL_STAGE" "/etc/systemd/system/${PI_TUNNEL_SERVICE}.service"
rm -f "$PI_TUNNEL_STAGE"
sudo -n systemctl daemon-reload
sudo -n systemctl enable --now "$PI_TUNNEL_SERVICE" >/dev/null
sudo -n systemctl restart "$PI_TUNNEL_SERVICE"
REMOTE

  rm -f "$rendered_unit"
}

derive_cloudflare_mode() {
  local host="$1"
  if [[ "$PI_CLOUDFLARE_MODE" != "auto" ]]; then
    printf '%s' "$PI_CLOUDFLARE_MODE"
    return 0
  fi

  if cloudflare_unit_exists "$host" "$PI_CLOUDFLARE_TUNNEL_SERVICE"; then
    printf '%s' "anon-systemd"
    return 0
  fi
  if cloudflare_unit_exists "$host" "$PI_CLOUDFLARE_SERVICE" || cloudflare_ssh "$host" "sudo -n test -f '$PI_CLOUDFLARE_CONFIG'" >/dev/null 2>&1; then
    printf '%s' "managed-service"
    return 0
  fi
  if [[ -n "${CF_TUNNEL_TOKEN:-}" ]]; then
    printf '%s' "managed-service"
    return 0
  fi
  printf '%s' "anon-systemd"
}

print_service_status_block() {
  local host="$1"
  local service_name="$2"
  local label="$3"

  cloudflare_ssh "$host" \
    SERVICE_NAME="$service_name" \
    LABEL="$label" \
    bash -s <<'REMOTE'
set -euo pipefail
if sudo -n systemctl list-unit-files | grep -q "^${SERVICE_NAME}\.service"; then
  echo "${LABEL}: installed"
  echo "Enabled: $(sudo -n systemctl is-enabled "$SERVICE_NAME" 2>/dev/null || echo unknown)"
  echo "Active:  $(sudo -n systemctl is-active "$SERVICE_NAME" 2>/dev/null || echo inactive)"
else
  echo "${LABEL}: missing"
fi
REMOTE
}

print_trycloudflare_url() {
  local host="$1"
  local service_name="$2"
  cloudflare_ssh "$host" \
    SERVICE_NAME="$service_name" \
    bash -s <<'REMOTE'
set -euo pipefail
sudo -n journalctl -u "$SERVICE_NAME" -n 200 --no-pager 2>/dev/null \
  | grep -Eo 'https://[A-Za-z0-9.-]+trycloudflare.com' \
  | tail -1 || true
REMOTE
}

print_status() {
  local host
  host="$(resolve_pi_host)"
  local mode
  mode="$(derive_cloudflare_mode "$host")"

  cloudflare_ssh "$host" \
    PI_CLOUDFLARE_SERVICE="$PI_CLOUDFLARE_SERVICE" \
    PI_CLOUDFLARE_CONFIG="$PI_CLOUDFLARE_CONFIG" \
    PI_CLOUDFLARE_TUNNEL_SERVICE="$PI_CLOUDFLARE_TUNNEL_SERVICE" \
    PI_CLOUDFLARE_MODE="$mode" \
    bash -s <<'REMOTE'
set -euo pipefail

service_name="$PI_CLOUDFLARE_SERVICE"
config_path="$PI_CLOUDFLARE_CONFIG"
tunnel_service_name="$PI_CLOUDFLARE_TUNNEL_SERVICE"
mode="$PI_CLOUDFLARE_MODE"

echo "Host: $(hostname)"
echo "SSH user: $(whoami)"
echo "Mode: $mode"
if command -v cloudflared >/dev/null 2>&1; then
  echo "cloudflared: installed"
  cloudflared --version | head -1
else
  echo "cloudflared: missing"
fi

echo
if sudo -n systemctl cat "$service_name" >/dev/null 2>&1; then
  echo "Managed service: installed"
  echo "Enabled: $(sudo -n systemctl is-enabled "$service_name" 2>/dev/null || echo unknown)"
  echo "Active:  $(sudo -n systemctl is-active "$service_name" 2>/dev/null || echo inactive)"
else
  echo "Managed service: missing"
fi

echo
if sudo -n systemctl cat "$tunnel_service_name" >/dev/null 2>&1; then
  echo "Tunnel adapter service: installed"
  echo "Enabled: $(sudo -n systemctl is-enabled "$tunnel_service_name" 2>/dev/null || echo unknown)"
  echo "Active:  $(sudo -n systemctl is-active "$tunnel_service_name" 2>/dev/null || echo inactive)"
else
  echo "Tunnel adapter service: missing"
fi

echo
if sudo -n test -f "$config_path"; then
  echo "Config: $config_path"
  hostnames="$(sudo -n awk '/hostname:/ {print $NF}' "$config_path" || true)"
  if [[ -n "$hostnames" ]]; then
    echo "Public URLs:"
    while IFS= read -r hostname; do
      [[ -n "$hostname" ]] && echo "  https://$hostname"
    done <<< "$hostnames"
  else
    echo "Public URLs: none declared in local config"
  fi
else
  echo "Config: missing"
  ephemeral_url="$(sudo -n journalctl -u "$tunnel_service_name" -n 200 --no-pager 2>/dev/null | grep -Eo 'https://[A-Za-z0-9.-]+trycloudflare.com' | tail -1 || true)"
  [[ -z "$ephemeral_url" ]] && ephemeral_url="$(sudo -n journalctl -u "$service_name" -n 200 --no-pager 2>/dev/null | grep -Eo 'https://[A-Za-z0-9.-]+trycloudflare.com' | tail -1 || true)"
  if [[ -n "$ephemeral_url" ]]; then
    echo "Ephemeral URL: $ephemeral_url"
  else
    echo "Public URLs: unavailable from local state"
  fi
fi
REMOTE
}

print_urls() {
  local host
  host="$(resolve_pi_host)"
  local mode
  mode="$(derive_cloudflare_mode "$host")"

  if [[ "$mode" == "managed-service" ]]; then
    cloudflare_ssh "$host" \
      PI_CLOUDFLARE_SERVICE="$PI_CLOUDFLARE_SERVICE" \
      PI_CLOUDFLARE_CONFIG="$PI_CLOUDFLARE_CONFIG" \
      bash -s <<'REMOTE'
set -euo pipefail

service_name="$PI_CLOUDFLARE_SERVICE"
config_path="$PI_CLOUDFLARE_CONFIG"

if sudo -n test -f "$config_path"; then
  sudo -n awk '/hostname:/ {print "https://" $NF}' "$config_path"
else
  sudo -n journalctl -u "$service_name" -n 200 --no-pager 2>/dev/null \
    | grep -Eo 'https://[A-Za-z0-9.-]+trycloudflare.com' \
    | tail -1 || true
fi
REMOTE
    return 0
  fi

  print_trycloudflare_url "$host" "$PI_CLOUDFLARE_TUNNEL_SERVICE"
}

print_config() {
  local host
  host="$(resolve_pi_host)"
  require_remote_sudo "$host"

  cloudflare_ssh "$host" \
    PI_CLOUDFLARE_SERVICE="$PI_CLOUDFLARE_SERVICE" \
    PI_CLOUDFLARE_CONFIG="$PI_CLOUDFLARE_CONFIG" \
    PI_CLOUDFLARE_TUNNEL_SERVICE="$PI_CLOUDFLARE_TUNNEL_SERVICE" \
    PI_DEST="${PI_DEST:-/home/$PI_USER/Polymarket-Trader-dev}" \
    bash -s <<'REMOTE'
set -euo pipefail

service_name="$PI_CLOUDFLARE_SERVICE"
config_path="$PI_CLOUDFLARE_CONFIG"
tunnel_service_name="$PI_CLOUDFLARE_TUNNEL_SERVICE"
env_path="$PI_DEST/.env.pi"

echo "--- managed service ---"
sudo -n systemctl cat "$service_name" 2>/dev/null \
  | sed -E 's/(token )[A-Za-z0-9._-]+/\1[REDACTED]/g' \
  || echo "missing"

echo
echo "--- tunnel adapter service ---"
sudo -n systemctl cat "$tunnel_service_name" 2>/dev/null || echo "missing"

echo
echo "--- tunnel env ---"
if sudo -n test -f "$env_path"; then
  sudo -n awk -F= '/^(OPS_DASHBOARD_BIND|PT_CLOUDFLARE_ORIGIN_URL|TRADINGVIEW_BIND_ADDR)=/ {print}' "$env_path"
else
  echo "missing"
fi

echo
echo "--- config ---"
if sudo -n test -f "$config_path"; then
  sudo -n sed -n '1,220p' "$config_path"
else
  echo "missing"
fi
REMOTE
}

ensure_cloudflare() {
  local host
  host="$(resolve_pi_host)"
  require_remote_sudo "$host"
  local mode
  mode="$(derive_cloudflare_mode "$host")"

  case "$mode" in
    anon-systemd)
      echo "==> Ensuring barebones Cloudflare tunnel adapter on ${PI_CLOUDFLARE_USER}@${host}"
      ensure_anonymous_tunnel_service "$host"
      ;;
    managed-service)
      install_cloudflared_if_missing "$host"
      if ! cloudflare_unit_exists "$host" "$PI_CLOUDFLARE_SERVICE"; then
        : "${CF_TUNNEL_TOKEN:?CF_TUNNEL_TOKEN env var is required to install the Cloudflare tunnel service}"
        echo "==> Registering Cloudflare managed tunnel service on ${PI_CLOUDFLARE_USER}@${host}"
        cloudflare_ssh "$host" "sudo -n cloudflared service install '$CF_TUNNEL_TOKEN'"
      fi
      echo "==> Enabling and restarting ${PI_CLOUDFLARE_SERVICE}"
      cloudflare_ssh "$host" "sudo -n systemctl enable --now '$PI_CLOUDFLARE_SERVICE' >/dev/null && sudo -n systemctl restart '$PI_CLOUDFLARE_SERVICE'"
      ;;
    *)
      echo "unknown PI_CLOUDFLARE_MODE: $mode" >&2
      exit 1
      ;;
  esac

  echo "==> Cloudflare tunnel status"
  print_status
}

case "${1:-}" in
  ensure|setup)
    ensure_cloudflare
    ;;
  status)
    print_status
    ;;
  url)
    print_urls
    ;;
  print-config|config)
    print_config
    ;;
  -h|--help|help|"")
    usage
    ;;
  *)
    echo "Unknown command: $1" >&2
    usage
    exit 1
    ;;
esac
