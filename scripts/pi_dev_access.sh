#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# shellcheck source=scripts/pi_lib.sh
source "${ROOT_DIR}/scripts/pi_lib.sh"

PI_PUBLIC_PORT="${PI_PUBLIC_PORT:-8080}"
PI_LOCAL_TUNNEL_PORT="${PI_LOCAL_TUNNEL_PORT:-18080}"

usage() {
  cat <<'EOF'
Usage: scripts/pi_dev_access.sh <command>

Commands:
  scan           Discover and print the Pi IP (uses ARP→mDNS→hostname→sweep).
  urls           Print direct + tunneled dashboard URLs.
  tunnel         Open an SSH tunnel: localhost:$PI_LOCAL_TUNNEL_PORT → Pi:$PI_PUBLIC_PORT.
  ssh            Open an interactive SSH session to the Pi.

Environment:
  PI_HOST                Static host or IP (skips discovery).
  PI_USER                SSH user             (default: pi).
  PI_PORT                SSH port             (default: 22).
  PI_KEY                 SSH key path         (default: ~/.ssh autodetect).
  PI_PUBLIC_PORT         Pi-side dashboard port (default: 8080).
  PI_LOCAL_TUNNEL_PORT   Local tunnel port      (default: 18080).
EOF
}

print_urls() {
  local host
  host="$(resolve_pi_host)"
  cat <<EOF
Dashboard: http://${host}:${PI_PUBLIC_PORT}/
Health:    http://${host}:${PI_PUBLIC_PORT}/healthz
Ready:     http://${host}:${PI_PUBLIC_PORT}/ready
Metrics:   http://${host}:${PI_PUBLIC_PORT}/metrics

Tunnel access (after \`scripts/pi_dev_access.sh tunnel\`):
  http://127.0.0.1:${PI_LOCAL_TUNNEL_PORT}/
EOF
}

start_tunnel() {
  local host
  host="$(resolve_pi_host)"
  local opts=()
  while IFS= read -r line; do
    opts+=("$line")
  done < <(pi_ssh_opts)
  exec ssh "${opts[@]}" -N -L "${PI_LOCAL_TUNNEL_PORT}:127.0.0.1:${PI_PUBLIC_PORT}" "${PI_USER}@${host}"
}

scan_pi() {
  local host
  host="$(resolve_pi_host)"
  echo "$host"
}

ssh_into_pi() {
  local host
  host="$(resolve_pi_host)"
  local opts=()
  while IFS= read -r line; do
    opts+=("$line")
  done < <(pi_ssh_opts)
  exec ssh "${opts[@]}" "${PI_USER}@${host}"
}

case "${1:-}" in
  scan)   scan_pi ;;
  urls)   print_urls ;;
  tunnel) start_tunnel ;;
  ssh)    ssh_into_pi ;;
  -h|--help|help|"") usage ;;
  *)
    echo "Unknown command: $1" >&2
    usage
    exit 1
    ;;
esac
