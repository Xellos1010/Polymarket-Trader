#!/usr/bin/env bash
# Polymarket-Trader Raspberry Pi discovery and SSH helpers.
# Source this file from other scripts; do not execute it directly.
#
# Inspired by tools/scripts/pi-lib.sh from the Homo-Plasticus monorepo.
# Adapted for the Polymarket-Trader Pi dev deployment flow.
#
# Required environment (can be overridden by callers or .env):
#   PI_USER          SSH user                        (default: pi)
#   PI_HOST          Static host or IPv4 (skips discovery; preferred)
#   PI_IP            Synonym for PI_HOST (set by tailscale resolver)
#   PI_HOSTNAME      mDNS hostname to probe          (default: raspberrypi)
#   PI_PORT          SSH port                        (default: 22)
#   PI_KEY           Optional explicit SSH key path
#   PI_CACHE_FILE    Cached IP path                  (default: ~/.cache/polymarket-trader/pi-ip)
#   PING_TIMEOUT     Per-host ping timeout (seconds) (default: 1)
#   SWEEP_PARALLEL   Parallel ping fanout            (default: 64)
#   SSH_CONNECT_TIMEOUT  ssh -o ConnectTimeout       (default: 8)

# shellcheck disable=SC2034 # variables here are consumed by sourcing scripts
PI_USER="${PI_USER:-pi}"
PI_PORT="${PI_PORT:-22}"
PI_HOSTNAME="${PI_HOSTNAME:-raspberrypi}"
PI_KEY="${PI_KEY:-}"
PI_CACHE_FILE="${PI_CACHE_FILE:-${HOME}/.cache/polymarket-trader/pi-ip}"
PING_TIMEOUT="${PING_TIMEOUT:-1}"
SWEEP_PARALLEL="${SWEEP_PARALLEL:-64}"
SSH_CONNECT_TIMEOUT="${SSH_CONNECT_TIMEOUT:-8}"

# Known Raspberry Pi MAC OUI prefixes (lowercase).
PI_MAC_PREFIXES=("b8:27:eb" "dc:a6:32" "e4:5f:01" "d8:3a:dd" "2c:cf:67")

# Legacy night-agent cache may still hold a usable IP — read it as a hint, but
# only if our own cache is empty. Never write back to the legacy location.
LEGACY_PI_CACHE_FILE="${HOME}/.cache/night-agent/pi-ip"

_PT_PI_PLATFORM=""
case "$(uname -s)" in
  Linux) _PT_PI_PLATFORM=linux ;;
  Darwin) _PT_PI_PLATFORM=macos ;;
  *) echo "[pi-lib] Unsupported OS: $(uname -s)" >&2 ;;
esac

_pi_log() { echo "[pi-lib] $*" >&2; }

_pi_ping_host() {
  local host="$1"
  if [[ "$_PT_PI_PLATFORM" == "linux" ]]; then
    ping -c 1 -W "$PING_TIMEOUT" "$host" >/dev/null 2>&1
  else
    ping -c 1 -W "$((PING_TIMEOUT * 1000))" "$host" >/dev/null 2>&1
  fi
}

_pi_check_ssh_port() {
  local host="$1"
  if command -v nc >/dev/null 2>&1; then
    nc -z -w 3 "$host" "$PI_PORT" >/dev/null 2>&1
  else
    # Fallback: use bash /dev/tcp where available.
    timeout 3 bash -c ">/dev/tcp/${host}/${PI_PORT}" >/dev/null 2>&1
  fi
}

_pi_local_subnet24() {
  if [[ "$_PT_PI_PLATFORM" == "linux" ]]; then
    local iface_ip
    iface_ip=$(ip route get 8.8.8.8 2>/dev/null | grep -oP 'src \K[\d.]+' | head -1 || true)
    [[ -n "$iface_ip" ]] && echo "${iface_ip%.*}.0/24"
    return
  fi

  local iface_ip
  iface_ip=$(route get default 2>/dev/null | awk '/interface:/{print $2}' \
    | xargs -I{} ifconfig {} 2>/dev/null \
    | awk '/inet /{print $2}' \
    | head -1 || true)
  [[ -n "$iface_ip" ]] && echo "${iface_ip%.*}.0/24"
}

_pi_scan_arp_cache() {
  _pi_log "ARP cache check..."
  local neigh_output=""
  if [[ "$_PT_PI_PLATFORM" == "linux" ]]; then
    neigh_output=$(ip neigh show 2>/dev/null | grep -v "FAILED" || true)
  else
    neigh_output=$(arp -an 2>/dev/null || true)
  fi
  [[ -z "$neigh_output" ]] && return 0

  local prefix
  for prefix in "${PI_MAC_PREFIXES[@]}"; do
    if [[ "$_PT_PI_PLATFORM" == "linux" ]]; then
      echo "$neigh_output" | awk -v p="${prefix}" 'tolower($0) ~ p {print $1}'
    else
      echo "$neigh_output" | awk -v p="${prefix}" 'tolower($0) ~ p {gsub(/[()]/,"",$2); print $2}'
    fi
  done | awk 'NF' | head -1
}

_pi_scan_mdns() {
  _pi_log "mDNS: ${PI_HOSTNAME}.local..."
  _pi_ping_host "${PI_HOSTNAME}.local" || return 0
  ping -c 1 "${PI_HOSTNAME}.local" 2>&1 \
    | grep -oE '([0-9]{1,3}\.){3}[0-9]{1,3}' \
    | head -1
}

_pi_scan_hostname_direct() {
  _pi_log "Hostname: ${PI_HOSTNAME}..."
  _pi_ping_host "$PI_HOSTNAME" || return 0
  ping -c 1 "$PI_HOSTNAME" 2>&1 \
    | grep -oE '([0-9]{1,3}\.){3}[0-9]{1,3}' \
    | head -1
}

_pi_scan_subnet_24() {
  local subnet24
  subnet24=$(_pi_local_subnet24)
  [[ -z "$subnet24" ]] && return 0

  _pi_log "Sweeping ${subnet24} (parallel=${SWEEP_PARALLEL})..."

  local base_ip="${subnet24%.*}"
  local tmpdir
  tmpdir="$(mktemp -d /tmp/pt-pi-scan.XXXXXX)"
  trap 'rm -rf "$tmpdir"' RETURN

  local pids=()
  local count=0
  local i
  for i in $(seq 1 254); do
    (
      _pi_ping_host "${base_ip}.${i}" && echo "${base_ip}.${i}" > "${tmpdir}/${i}.hit"
    ) &
    pids+=($!)
    count=$((count + 1))
    if [[ $count -ge $SWEEP_PARALLEL ]]; then
      wait "${pids[@]}" 2>/dev/null || true
      pids=()
      count=0
    fi
  done
  [[ ${#pids[@]} -gt 0 ]] && wait "${pids[@]}" 2>/dev/null || true

  local hit
  for hit in "${tmpdir}"/*.hit; do
    [[ -f "$hit" ]] || continue
    local ip
    ip="$(cat "$hit")"
    if _pi_check_ssh_port "$ip"; then
      echo "$ip"
      return 0
    fi
  done
}

# Resolve the SSH key path. Honors $PI_KEY, then common Polymarket-Trader paths,
# then the legacy night-agent location for shared boards.
resolve_pi_key() {
  if [[ -n "${PI_KEY:-}" ]]; then
    if [[ ! -f "$PI_KEY" ]]; then
      _pi_log "PI_KEY is set but the file does not exist: $PI_KEY"
      return 1
    fi
    echo "$PI_KEY"
    return 0
  fi

  local candidate
  for candidate in \
    "${HOME}/.ssh/id_ed25519_polymarket_trader" \
    "${HOME}/.ssh/id_ed25519_raspberrypi" \
    "${HOME}/.ssh/id_ed25519" \
    "${HOME}/.ssh/id_rsa"
  do
    if [[ -f "$candidate" ]]; then
      _pi_log "SSH key: $candidate"
      echo "$candidate"
      return 0
    fi
  done

  _pi_log "No SSH key found. Set PI_KEY=/path/to/private_key or place one in ~/.ssh."
  return 1
}

# Probe whether the Pi answers an SSH banner via the resolved key.
pi_is_reachable() {
  local host="$1"
  local key
  key="$(resolve_pi_key)" || return 1
  ssh -i "$key" \
    -o IdentitiesOnly=yes \
    -o StrictHostKeyChecking=no \
    -o UserKnownHostsFile=/dev/null \
    -o ConnectTimeout="$SSH_CONNECT_TIMEOUT" \
    -o LogLevel=ERROR \
    -p "$PI_PORT" \
    "${PI_USER}@${host}" true >/dev/null 2>&1
}

find_pi() {
  local ip

  ip="$(_pi_scan_arp_cache || true)"
  [[ -n "$ip" ]] && { echo "$ip"; return 0; }

  ip="$(_pi_scan_mdns || true)"
  [[ -n "$ip" ]] && { echo "$ip"; return 0; }

  ip="$(_pi_scan_hostname_direct || true)"
  [[ -n "$ip" ]] && { echo "$ip"; return 0; }

  ip="$(_pi_scan_subnet_24 || true)"
  [[ -n "$ip" ]] && { echo "$ip"; return 0; }

  return 1
}

# Resolve the Pi host.
# Priority: $PI_HOST > $PI_IP > $PI_CACHE_FILE > legacy cache > active discovery.
# Cached IPs are confirmed reachable before reuse; stale entries trigger a rescan.
resolve_pi_host() {
  if [[ -n "${PI_HOST:-}" ]]; then
    echo "$PI_HOST"
    return 0
  fi
  if [[ -n "${PI_IP:-}" ]]; then
    echo "$PI_IP"
    return 0
  fi

  local cached=""
  if [[ -f "$PI_CACHE_FILE" ]]; then
    cached="$(tr -d '[:space:]' < "$PI_CACHE_FILE")"
  elif [[ -f "$LEGACY_PI_CACHE_FILE" ]]; then
    cached="$(tr -d '[:space:]' < "$LEGACY_PI_CACHE_FILE")"
  fi

  if [[ -n "$cached" ]] && pi_is_reachable "$cached"; then
    echo "$cached"
    return 0
  fi
  if [[ -n "$cached" ]]; then
    _pi_log "Cached Pi IP ${cached} is unreachable; rescanning..."
  else
    _pi_log "No cached Pi IP; scanning network..."
  fi

  local found
  found="$(find_pi || true)"
  if [[ -z "$found" ]]; then
    _pi_log "Could not locate the Pi on the local network."
    _pi_log "Set PI_HOST=<address> (LAN or Tailscale IP) and retry."
    return 1
  fi

  mkdir -p "$(dirname "$PI_CACHE_FILE")"
  echo "$found" > "$PI_CACHE_FILE"
  _pi_log "Pi found at ${found} and cached."
  echo "$found"
}

# Build a stable SSH option array. Callers should pass user@host as the last arg.
# Emits one option per line so callers can read it into an array via:
#   while IFS= read -r line; do opts+=("$line"); done < <(pi_ssh_opts)
pi_ssh_opts() {
  local key
  key="$(resolve_pi_key)" || return 1
  printf -- '-i\n%s\n-p\n%s\n-o\nIdentitiesOnly=yes\n-o\nStrictHostKeyChecking=no\n-o\nUserKnownHostsFile=/dev/null\n-o\nConnectTimeout=%s\n-o\nServerAliveInterval=30\n-o\nServerAliveCountMax=3\n' \
    "$key" "$PI_PORT" "$SSH_CONNECT_TIMEOUT"
}

# Build an scp option array. Same semantics as pi_ssh_opts but uses `-P`
# (uppercase) for port, since that is scp's convention. Re-using pi_ssh_opts
# for scp will fail because scp's lowercase `-p` is the "preserve" flag.
pi_scp_opts() {
  local key
  key="$(resolve_pi_key)" || return 1
  printf -- '-i\n%s\n-P\n%s\n-o\nIdentitiesOnly=yes\n-o\nStrictHostKeyChecking=no\n-o\nUserKnownHostsFile=/dev/null\n-o\nConnectTimeout=%s\n-o\nServerAliveInterval=30\n-o\nServerAliveCountMax=3\n' \
    "$key" "$PI_PORT" "$SSH_CONNECT_TIMEOUT"
}

# Format the rsync `-e` ssh wrapper string. Quote each field for correctness
# even when paths contain spaces.
pi_rsync_e() {
  local key
  key="$(resolve_pi_key)" || return 1
  printf 'ssh -i %q -p %q -o IdentitiesOnly=yes -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -o ConnectTimeout=%q' \
    "$key" "$PI_PORT" "$SSH_CONNECT_TIMEOUT"
}

# Convenience wrapper around scp with our stable option set. Callers should
# pass standard scp arguments; the flags are prepended automatically.
#   pi_scp local-file user@host:/remote/path
pi_scp() {
  local opts=()
  while IFS= read -r line; do
    opts+=("$line")
  done < <(pi_scp_opts)
  scp "${opts[@]}" "$@"
}

# Run a remote command via SSH. Use this instead of inlining ssh flags everywhere.
#   pi_ssh "$host" "uptime"
pi_ssh() {
  local host="$1"
  shift
  local opts=()
  while IFS= read -r line; do
    opts+=("$line")
  done < <(pi_ssh_opts)
  ssh "${opts[@]}" "${PI_USER}@${host}" "$@"
}

# Run a remote bash -lc command, properly quoting the script.
pi_ssh_bash() {
  local host="$1"
  local script="$2"
  pi_ssh "$host" "bash -lc $(printf '%q' "$script")"
}
