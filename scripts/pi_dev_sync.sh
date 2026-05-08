#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# shellcheck source=scripts/pi_lib.sh
source "${ROOT_DIR}/scripts/pi_lib.sh"

LOCAL_DIR="$ROOT_DIR/.local"
WATCH_PID_FILE="$LOCAL_DIR/pi-dev-watch.pid"
WATCH_LOG_FILE="$LOCAL_DIR/pi-dev-watch.log"

PI_DEST="${PI_DEST:-/home/$PI_USER/Polymarket-Trader-dev}"
PI_SERVICE="${PI_SERVICE:-pt-engine-dev}"
PI_PUBLIC_PORT="${PI_PUBLIC_PORT:-8080}"
PI_HEALTH_URL="${PI_HEALTH_URL:-http://127.0.0.1:${PI_PUBLIC_PORT}/healthz}"
PI_POLL_SECONDS="${PI_POLL_SECONDS:-2}"
PI_SERVICE_TEMPLATE="${PI_SERVICE_TEMPLATE:-${ROOT_DIR}/infra/systemd/pt-engine-dev.service.template}"
PI_BUILD_PROFILE="${PI_BUILD_PROFILE:-debug}"  # debug | release

usage() {
  cat <<'EOF'
Usage:
  scripts/pi_dev_sync.sh deploy             Build frontend, sync, rebuild on Pi, restart service.
  scripts/pi_dev_sync.sh init-env           Render Pi-side .env.pi from current shell environment.
  scripts/pi_dev_sync.sh install-support    Install Rust/Node/build deps on a fresh Pi.
  scripts/pi_dev_sync.sh status             Show systemd status of the Pi service.
  scripts/pi_dev_sync.sh health             Curl /healthz on the Pi (LAN) and report status.
  scripts/pi_dev_sync.sh remote-logs        Tail journalctl for the Pi service.
  scripts/pi_dev_sync.sh down               Stop and disable the Pi service (leaves files).
  scripts/pi_dev_sync.sh remove             Remove the deployed workspace, env, and unit.
  scripts/pi_dev_sync.sh watch start        Start background sync/watch loop.
  scripts/pi_dev_sync.sh watch stop         Stop background sync/watch loop.
  scripts/pi_dev_sync.sh watch status       Print watch state and remote service status.
  scripts/pi_dev_sync.sh watch logs         Tail the local watch log.

Environment (see scripts/pi_lib.sh for discovery defaults):
  PI_HOST              Static host or IP (LAN or Tailscale 100.x).
  PI_USER              Default: pi
  PI_PORT              Default: 22
  PI_DEST              Default: /home/$PI_USER/Polymarket-Trader-dev
  PI_SERVICE           Default: pt-engine-dev
  PI_PUBLIC_PORT       Default: 8080
  PI_HEALTH_URL        Default: http://127.0.0.1:$PI_PUBLIC_PORT/healthz
  PI_POLL_SECONDS      Default: 2
  PI_SERVICE_TEMPLATE  Default: ./infra/systemd/pt-engine-dev.service.template
  PI_BUILD_PROFILE     debug | release (default: debug)
  PI_KEY               SSH private key path (auto-detected from ~/.ssh otherwise)
EOF
}

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "missing required command: $1" >&2
    exit 1
  fi
}

frontend_build() {
  local frontend_dir="$ROOT_DIR/crates/pt-dashboard/frontend"
  if [ ! -d "$frontend_dir/node_modules" ] && [ ! -d "$ROOT_DIR/node_modules" ]; then
    echo "missing frontend dependencies" >&2
    echo "run: pnpm install" >&2
    exit 1
  fi
  echo "[local] building dashboard frontend"
  (
    cd "$frontend_dir"
    pnpm run build
  )
}

rsync_repo() {
  local pi_host
  pi_host="$(resolve_pi_host)"
  local rsh
  rsh="$(pi_rsync_e)"

  echo "[sync] rsync -> $PI_USER@$pi_host:$PI_DEST"
  rsync -az --delete \
    -e "$rsh" \
    --exclude '.git/' \
    --exclude '.nx/' \
    --exclude '.local/' \
    --exclude '.env' \
    --exclude '.env.*' \
    --exclude 'artifacts/' \
    --exclude 'dist/' \
    --exclude 'node_modules/' \
    --exclude 'target/' \
    --exclude 'config/config.toml' \
    --exclude 'config/*api_key*.json' \
    --exclude 'config/*secret*.json' \
    --exclude 'config/*credentials*.json' \
    --exclude 'config/*_token.json' \
    --exclude 'crates/pt-dashboard/frontend/node_modules/' \
    "$ROOT_DIR/" "$PI_USER@$pi_host:$PI_DEST/"
}

# Render .env.pi locally from the calling shell, scp it to the Pi, and lock perms.
remote_write_env_file() {
  mkdir -p "$LOCAL_DIR"
  local pi_host
  pi_host="$(resolve_pi_host)"
  local tmp_env
  tmp_env="$(mktemp "$LOCAL_DIR/pi-env.XXXXXX")"

  umask 077
  cat > "$tmp_env" <<EOF
RUST_LOG=info
POLYMARKET_PRIVATE_KEY=${POLYMARKET_PRIVATE_KEY:-}
COINBASE_API_KEY=${COINBASE_API_KEY:-}
COINBASE_API_SECRET=${COINBASE_API_SECRET:-}
COINBASE_PASSPHRASE=${COINBASE_PASSPHRASE:-}
COINBASE_AUTH_PROFILE=${COINBASE_AUTH_PROFILE:-}
COINBASE_CDP_KEY_FILE=${COINBASE_CDP_KEY_FILE:-}
COINBASE_CDP_SECRET_ID=${COINBASE_CDP_SECRET_ID:-}
COINBASE_EXPECTED_KEY_ID=${COINBASE_EXPECTED_KEY_ID:-}
TRADINGVIEW_ENDPOINT_SECRET=${TRADINGVIEW_ENDPOINT_SECRET:-}
OPS_DASHBOARD_BIND=${OPS_DASHBOARD_BIND:-0.0.0.0:${PI_PUBLIC_PORT}}
TRADINGVIEW_BIND_ADDR=${TRADINGVIEW_BIND_ADDR:-127.0.0.1:8090}
EOF

  pi_ssh "$pi_host" "mkdir -p '$PI_DEST' && chmod 700 '$PI_DEST'"

  pi_scp "$tmp_env" "$PI_USER@$pi_host:$PI_DEST/.env.pi"
  pi_ssh "$pi_host" "chmod 600 '$PI_DEST/.env.pi' && echo '[remote] wrote $PI_DEST/.env.pi'"
  rm -f "$tmp_env"
}

# Render the systemd unit on the dev box (so we can review with `nx fmt` etc.),
# then copy and install it on the Pi. Substitution uses POSIX `sed` and is
# intentionally limited to the three placeholders the template documents.
render_service_unit() {
  local out_path="$1"
  if [ ! -f "$PI_SERVICE_TEMPLATE" ]; then
    echo "missing service template: $PI_SERVICE_TEMPLATE" >&2
    exit 1
  fi
  sed \
    -e "s|\${PI_SERVICE}|${PI_SERVICE}|g" \
    -e "s|\${PI_USER}|${PI_USER}|g" \
    -e "s|\${PI_DEST}|${PI_DEST}|g" \
    "$PI_SERVICE_TEMPLATE" > "$out_path"
}

remote_deploy() {
  local pi_host
  pi_host="$(resolve_pi_host)"

  mkdir -p "$LOCAL_DIR"
  local rendered_unit
  rendered_unit="$(mktemp "$LOCAL_DIR/${PI_SERVICE}.service.XXXXXX")"
  render_service_unit "$rendered_unit"

  # Use a per-user staging path so two operators (or the same operator
  # switching `PI_USER`) cannot stomp on each other's `/tmp/<service>.service`.
  local remote_unit_stage="/tmp/${PI_USER}-${PI_SERVICE}.service"
  pi_scp "$rendered_unit" "$PI_USER@$pi_host:${remote_unit_stage}"

  # Build profile is forwarded so the Pi rebuilds whatever the operator wants.
  pi_ssh "$pi_host" \
    PI_DEST="$PI_DEST" \
    PI_SERVICE="$PI_SERVICE" \
    PI_USER_NAME="$PI_USER" \
    PI_HEALTH_URL="$PI_HEALTH_URL" \
    PI_BUILD_PROFILE="$PI_BUILD_PROFILE" \
    PI_UNIT_STAGE="$remote_unit_stage" \
    bash -s <<'REMOTE'
set -euo pipefail

# Non-login shells via `ssh ... bash -s` do not source ~/.bashrc, so cargo
# installed by rustup at $HOME/.cargo/bin is not on PATH. Source the env
# shim so `cargo` is available.
# shellcheck disable=SC1090,SC1091
[ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"

mkdir -p "$PI_DEST"
cd "$PI_DEST"

if [ ! -f config/config.toml ]; then
  cp config/config.example.toml config/config.toml
fi

# Bind the dashboard to all interfaces so the LAN can reach it. Idempotent.
python3 - <<'PY'
from pathlib import Path
path = Path("config/config.toml")
text = path.read_text()
text = text.replace('dashboard_bind = "127.0.0.1:8080"', 'dashboard_bind = "0.0.0.0:8080"')
path.write_text(text)
PY

if [ "${PI_BUILD_PROFILE}" = "release" ]; then
  echo "[remote] cargo build --release -p pt-cli"
  cargo build --release -p pt-cli
  BIN_PATH="$PI_DEST/target/release/pt-cli"
else
  echo "[remote] cargo build -p pt-cli"
  cargo build -p pt-cli
  BIN_PATH="$PI_DEST/target/debug/pt-cli"
fi

# Patch the rendered unit's ExecStart if release was selected.
if [ "${PI_BUILD_PROFILE}" = "release" ]; then
  sed -i "s#${PI_DEST}/target/debug/pt-cli#${BIN_PATH}#" "${PI_UNIT_STAGE}"
fi

sudo cp "${PI_UNIT_STAGE}" "/etc/systemd/system/${PI_SERVICE}.service"
rm -f "${PI_UNIT_STAGE}"
sudo systemctl daemon-reload
sudo systemctl enable "$PI_SERVICE" >/dev/null
sudo systemctl restart "$PI_SERVICE"

sleep 3
curl -fsS "$PI_HEALTH_URL" >/dev/null
echo "[remote] service healthy at $PI_HEALTH_URL"
REMOTE
  rm -f "$rendered_unit"
}

deploy_once() {
  require_cmd rsync
  require_cmd ssh
  require_cmd cargo
  require_cmd pnpm
  mkdir -p "$LOCAL_DIR"
  frontend_build
  rsync_repo
  remote_write_env_file
  remote_deploy
}

# Bring up a fresh Pi: APT base + rustup + Node 22 + pnpm.
# Idempotent: safe to re-run.
remote_install_support() {
  local pi_host
  pi_host="$(resolve_pi_host)"
  pi_ssh "$pi_host" bash -s <<'REMOTE'
set -euo pipefail

if ! command -v sudo >/dev/null 2>&1; then
  echo "[remote] sudo is required for install-support" >&2
  exit 1
fi

sudo apt-get update
sudo apt-get install -y \
  build-essential \
  pkg-config \
  libssl-dev \
  curl \
  git \
  rsync \
  ca-certificates

if ! command -v cargo >/dev/null 2>&1; then
  echo "[remote] installing rustup (stable toolchain)"
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal
fi
# shellcheck disable=SC1090,SC1091
source "$HOME/.cargo/env" 2>/dev/null || true

if ! command -v node >/dev/null 2>&1 || ! node -v | grep -q '^v22\.'; then
  echo "[remote] installing Node 22"
  curl -fsSL https://deb.nodesource.com/setup_22.x | sudo -E bash -
  sudo apt-get install -y nodejs
fi

if ! command -v pnpm >/dev/null 2>&1; then
  echo "[remote] installing pnpm"
  sudo npm install -g pnpm@10
fi

echo "[remote] toolchain summary:"
cargo --version || true
node --version || true
pnpm --version || true
REMOTE
}

remote_status() {
  local pi_host
  pi_host="$(resolve_pi_host)"
  pi_ssh "$pi_host" "sudo systemctl --no-pager --full status $PI_SERVICE || true"
}

remote_health() {
  local pi_host
  pi_host="$(resolve_pi_host)"
  echo "[health] $PI_USER@$pi_host -> $PI_HEALTH_URL"
  pi_ssh "$pi_host" "curl -fsS -o /dev/null -w 'HTTP %{http_code} in %{time_total}s\\n' '$PI_HEALTH_URL'"
}

remote_logs() {
  local pi_host
  pi_host="$(resolve_pi_host)"
  local opts=()
  while IFS= read -r line; do
    opts+=("$line")
  done < <(pi_ssh_opts)
  exec ssh "${opts[@]}" -t "$PI_USER@$pi_host" "sudo journalctl -u $PI_SERVICE -f"
}

remote_down() {
  local pi_host
  pi_host="$(resolve_pi_host)"
  pi_ssh "$pi_host" \
    PI_SERVICE="$PI_SERVICE" \
    bash -s <<'REMOTE'
set -euo pipefail
if systemctl list-unit-files | grep -q "^${PI_SERVICE}.service"; then
  sudo systemctl stop "$PI_SERVICE" || true
  sudo systemctl disable "$PI_SERVICE" || true
  echo "[remote] $PI_SERVICE stopped and disabled"
else
  echo "[remote] $PI_SERVICE.service not installed"
fi
REMOTE
}

remote_remove() {
  local pi_host
  pi_host="$(resolve_pi_host)"
  pi_ssh "$pi_host" \
    PI_DEST="$PI_DEST" \
    PI_SERVICE="$PI_SERVICE" \
    bash -s <<'REMOTE'
set -euo pipefail
if systemctl list-unit-files | grep -q "^${PI_SERVICE}.service"; then
  sudo systemctl stop "$PI_SERVICE" || true
  sudo systemctl disable "$PI_SERVICE" || true
  sudo rm -f "/etc/systemd/system/${PI_SERVICE}.service"
  sudo systemctl daemon-reload
fi
if [ -d "$PI_DEST" ]; then
  rm -rf "$PI_DEST"
  echo "[remote] removed $PI_DEST"
fi
echo "[remote] removal complete"
REMOTE
}

watch_signature() {
  find "$ROOT_DIR" \
    \( \
      -path "$ROOT_DIR/.git" -o \
      -path "$ROOT_DIR/.local" -o \
      -path "$ROOT_DIR/.nx" -o \
      -path "$ROOT_DIR/artifacts" -o \
      -path "$ROOT_DIR/dist" -o \
      -path "$ROOT_DIR/node_modules" -o \
      -path "$ROOT_DIR/target" -o \
      -path "$ROOT_DIR/crates/pt-dashboard/frontend/node_modules" -o \
      -path "$ROOT_DIR/crates/pt-dashboard/frontend/dist" \
    \) -prune -o -type f -print | LC_ALL=C sort | while IFS= read -r path; do
      shasum "$path"
    done | shasum | awk '{print $1}'
}

watch_loop() {
  local last_sig current_sig
  last_sig=""
  while true; do
    current_sig="$(watch_signature)"
    if [ "$current_sig" != "$last_sig" ]; then
      echo "[$(date '+%Y-%m-%d %H:%M:%S')] change detected; syncing"
      if deploy_once; then
        last_sig="$current_sig"
        echo "[$(date '+%Y-%m-%d %H:%M:%S')] sync ok"
      else
        echo "[$(date '+%Y-%m-%d %H:%M:%S')] sync failed"
      fi
    fi
    sleep "$PI_POLL_SECONDS"
  done
}

watch_start() {
  mkdir -p "$LOCAL_DIR"
  if [ -f "$WATCH_PID_FILE" ] && kill -0 "$(cat "$WATCH_PID_FILE")" >/dev/null 2>&1; then
    echo "watch already running with PID $(cat "$WATCH_PID_FILE")"
    exit 0
  fi
  nohup "$0" watch-loop >"$WATCH_LOG_FILE" 2>&1 &
  echo "$!" > "$WATCH_PID_FILE"
  echo "watch started with PID $(cat "$WATCH_PID_FILE")"
  echo "logs: $WATCH_LOG_FILE"
}

watch_stop() {
  if [ ! -f "$WATCH_PID_FILE" ]; then
    echo "watch is not running"
    exit 0
  fi
  if kill -0 "$(cat "$WATCH_PID_FILE")" >/dev/null 2>&1; then
    kill "$(cat "$WATCH_PID_FILE")" >/dev/null 2>&1 || true
  fi
  rm -f "$WATCH_PID_FILE"
  echo "watch stopped; Pi service left running"
}

watch_status() {
  if [ -f "$WATCH_PID_FILE" ] && kill -0 "$(cat "$WATCH_PID_FILE")" >/dev/null 2>&1; then
    echo "watch running with PID $(cat "$WATCH_PID_FILE")"
  else
    echo "watch not running"
  fi
  remote_status
}

watch_logs() {
  mkdir -p "$LOCAL_DIR"
  touch "$WATCH_LOG_FILE"
  tail -f "$WATCH_LOG_FILE"
}

main() {
  case "${1:-}" in
    deploy)            deploy_once ;;
    init-env)          remote_write_env_file ;;
    install-support)   remote_install_support ;;
    status)            remote_status ;;
    health)            remote_health ;;
    remote-logs)       remote_logs ;;
    down)              remote_down ;;
    remove)            remote_remove ;;
    watch)
      case "${2:-}" in
        start)  watch_start ;;
        stop)   watch_stop ;;
        status) watch_status ;;
        logs)   watch_logs ;;
        *) usage; exit 1 ;;
      esac
      ;;
    watch-loop) watch_loop ;;
    -h|--help|help|"") usage ;;
    *) usage; exit 1 ;;
  esac
}

main "$@"
