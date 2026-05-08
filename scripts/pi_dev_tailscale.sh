#!/usr/bin/env bash
# Resolve the Pi's Tailscale IPv4 and forward all args to scripts/pi_dev_sync.sh
# (or scripts/pi_dev_access.sh) with PI_HOST set. Use this when the LAN path is
# unreachable but the Pi is on your tailnet.
#
# Usage:
#   scripts/pi_dev_tailscale.sh deploy
#   scripts/pi_dev_tailscale.sh status
#   scripts/pi_dev_tailscale.sh -- access urls          # delegate to pi_dev_access.sh
#   PI_TAILSCALE_HOST=polymarket-pi scripts/pi_dev_tailscale.sh health
#
# Environment:
#   PI_TAILSCALE_HOST  Hint for `tailscale ip --4 <hint>` and JSON peer match.
#                      Defaults to PI_HOSTNAME if set, else "polymarket-pi".
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "missing required command: $1" >&2
    exit 1
  fi
}

resolve_tailscale_ip() {
  local hint="$1"
  hint="${hint%.local}"

  local ip=""
  if ip="$(tailscale ip --4 "$hint" 2>/dev/null | awk 'NF' | head -1)"; then
    if [[ "$ip" =~ ^([0-9]{1,3}\.){3}[0-9]{1,3}$ ]]; then
      echo "$ip"
      return 0
    fi
  fi

  # Fallback: parse `tailscale status --json` for any peer whose DNSName or
  # HostName contains the hint.
  if command -v jq >/dev/null 2>&1; then
    ip="$(tailscale status --json 2>/dev/null | jq -r --arg h "$hint" '
      ([.Peer // {}] | .[] | to_entries[]?.value)
      + (if .Self then [.Self] else [] end)
      | map(select((.DNSName // "") | ascii_downcase | contains($h | ascii_downcase))
            // select((.HostName // "") | ascii_downcase | contains($h | ascii_downcase)))
      | first.TailscaleIPs[]?
      | select(test("^[0-9]+\\.[0-9]+\\.[0-9]+\\.[0-9]+$"))' \
      | head -1)"
    if [[ -n "$ip" ]]; then
      echo "$ip"
      return 0
    fi
  fi

  echo "[pi-tailscale] could not resolve Tailscale IP for hint: $hint" >&2
  echo "[pi-tailscale] check 'tailscale status' and that the Pi is online." >&2
  return 1
}

main() {
  require_cmd tailscale

  local hint="${PI_TAILSCALE_HOST:-${PI_HOSTNAME:-polymarket-pi}}"
  local ip
  ip="$(resolve_tailscale_ip "$hint")"
  echo "[pi-tailscale] Using Pi over Tailscale at $ip"

  local target_script="$ROOT_DIR/scripts/pi_dev_sync.sh"
  local args=("$@")

  # Allow `-- access urls` to dispatch to pi_dev_access.sh instead of pi_dev_sync.sh.
  if [[ ${#args[@]} -ge 2 && "${args[0]}" == "--" && "${args[1]}" == "access" ]]; then
    target_script="$ROOT_DIR/scripts/pi_dev_access.sh"
    args=("${args[@]:2}")
  fi

  PI_HOST="$ip" exec "$target_script" "${args[@]}"
}

main "$@"
