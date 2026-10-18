#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# shellcheck source=scripts/pi_lib.sh
source "${ROOT_DIR}/scripts/pi_lib.sh"

PI_DEST="${PI_DEST:-/home/$PI_USER/Polymarket-Trader-dev}"
PI_SERVICE="${PI_SERVICE:-pt-engine-dev}"
PI_SERVICE_TEMPLATE="${PI_SERVICE_TEMPLATE:-${ROOT_DIR}/infra/systemd/pt-engine-dev.service.template}"
PI_CLOUDFLARE_TUNNEL_SERVICE="${PI_CLOUDFLARE_TUNNEL_SERVICE:-pt-dashboard-cloudflare-tunnel}"
PI_CLOUDFLARE_TUNNEL_TEMPLATE="${PI_CLOUDFLARE_TUNNEL_TEMPLATE:-${ROOT_DIR}/infra/systemd/pt-dashboard-cloudflare-tunnel.service.template}"
INFRA_BUILD_DIR="${INFRA_BUILD_DIR:-${ROOT_DIR}/.local/infra}"

usage() {
  cat <<'EOF'
Usage: scripts/infra_render.sh <command> [args]

Commands:
  build                               Render all managed infra assets into .local/infra.
  print [all|service|cloudflare|manifest]
                                      Print rendered artifacts to stdout.
  render-service-unit <output-path>   Render the Pi app systemd unit.
  render-cloudflare-unit <output-path>
                                      Render the barebones Cloudflare adapter unit.

Environment:
  PI_USER                            Default: polytrader
  PI_DEST                            Default: /home/$PI_USER/Polymarket-Trader-dev
  PI_SERVICE                         Default: pt-engine-dev
  PI_CLOUDFLARE_TUNNEL_SERVICE       Default: pt-dashboard-cloudflare-tunnel
  INFRA_BUILD_DIR                    Default: ./.local/infra
EOF
}

require_template() {
  local path="$1"
  if [[ ! -f "$path" ]]; then
    echo "missing template: $path" >&2
    exit 1
  fi
}

render_service_unit() {
  local out_path="$1"
  require_template "$PI_SERVICE_TEMPLATE"
  sed \
    -e "s|\${PI_SERVICE}|${PI_SERVICE}|g" \
    -e "s|\${PI_USER}|${PI_USER}|g" \
    -e "s|\${PI_DEST}|${PI_DEST}|g" \
    "$PI_SERVICE_TEMPLATE" > "$out_path"
}

render_cloudflare_unit() {
  local out_path="$1"
  require_template "$PI_CLOUDFLARE_TUNNEL_TEMPLATE"
  sed \
    -e "s|\${PI_TUNNEL_SERVICE}|${PI_CLOUDFLARE_TUNNEL_SERVICE}|g" \
    -e "s|\${PI_SERVICE}|${PI_SERVICE}|g" \
    -e "s|\${PI_USER}|${PI_USER}|g" \
    -e "s|\${PI_DEST}|${PI_DEST}|g" \
    "$PI_CLOUDFLARE_TUNNEL_TEMPLATE" > "$out_path"
}

render_manifest() {
  local out_path="$1"
  cat > "$out_path" <<EOF
PI_USER=${PI_USER}
PI_DEST=${PI_DEST}
PI_SERVICE=${PI_SERVICE}
PI_SERVICE_TEMPLATE=${PI_SERVICE_TEMPLATE}
PI_CLOUDFLARE_TUNNEL_SERVICE=${PI_CLOUDFLARE_TUNNEL_SERVICE}
PI_CLOUDFLARE_TUNNEL_TEMPLATE=${PI_CLOUDFLARE_TUNNEL_TEMPLATE}
EOF
}

build_bundle() {
  mkdir -p "${INFRA_BUILD_DIR}/systemd" "${INFRA_BUILD_DIR}/pi"
  render_service_unit "${INFRA_BUILD_DIR}/systemd/${PI_SERVICE}.service"
  render_cloudflare_unit "${INFRA_BUILD_DIR}/systemd/${PI_CLOUDFLARE_TUNNEL_SERVICE}.service"
  render_manifest "${INFRA_BUILD_DIR}/infra.env"
  cp "${ROOT_DIR}/infra/pi/env.cloudflared.example" "${INFRA_BUILD_DIR}/pi/env.cloudflared.example"

  echo "Rendered infra bundle:"
  echo "  ${INFRA_BUILD_DIR}/systemd/${PI_SERVICE}.service"
  echo "  ${INFRA_BUILD_DIR}/systemd/${PI_CLOUDFLARE_TUNNEL_SERVICE}.service"
  echo "  ${INFRA_BUILD_DIR}/pi/env.cloudflared.example"
  echo "  ${INFRA_BUILD_DIR}/infra.env"
}

print_rendered() {
  local target="${1:-all}"
  local tmp_dir
  tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/pt-infra-render.XXXXXX")"

  local service_out="${tmp_dir}/${PI_SERVICE}.service"
  local cloudflare_out="${tmp_dir}/${PI_CLOUDFLARE_TUNNEL_SERVICE}.service"
  local manifest_out="${tmp_dir}/infra.env"

  render_service_unit "$service_out"
  render_cloudflare_unit "$cloudflare_out"
  render_manifest "$manifest_out"

  case "$target" in
    all)
      echo "--- ${PI_SERVICE}.service ---"
      cat "$service_out"
      echo
      echo "--- ${PI_CLOUDFLARE_TUNNEL_SERVICE}.service ---"
      cat "$cloudflare_out"
      echo
      echo "--- infra.env ---"
      cat "$manifest_out"
      ;;
    service)
      cat "$service_out"
      ;;
    cloudflare)
      cat "$cloudflare_out"
      ;;
    manifest)
      cat "$manifest_out"
      ;;
    *)
      rm -rf "$tmp_dir"
      echo "unknown print target: $target" >&2
      exit 1
      ;;
  esac

  rm -rf "$tmp_dir"
}

main() {
  case "${1:-}" in
    build)
      build_bundle
      ;;
    print)
      print_rendered "${2:-all}"
      ;;
    render-service-unit)
      [[ $# -eq 2 ]] || { usage >&2; exit 1; }
      render_service_unit "$2"
      ;;
    render-cloudflare-unit)
      [[ $# -eq 2 ]] || { usage >&2; exit 1; }
      render_cloudflare_unit "$2"
      ;;
    -h|--help|help|"")
      usage
      ;;
    *)
      echo "unknown command: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
}

main "$@"
