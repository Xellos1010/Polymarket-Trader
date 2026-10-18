---
name: pi-ops
description: >
  REQUIRED for ALL Raspberry Pi operations in the Polymarket-Trader workspace. You MUST consult this
  skill before doing anything with the Pi — discovery (scan/SSH/tunnel), deployment (init-env, deploy,
  install-support, watch start/stop/status/logs), service control (status, health, down, remove,
  remote-logs), or Tailscale-based access. Do not run pi_dev_*.sh scripts or `pnpm exec nx run
  polymarket-trader:pi-dev-*` targets without reading this skill first — the available targets live
  in `project.json` and change as the repo evolves; this skill maps tasks to the right command.
  Triggers on: "deploy to pi", "ssh into pi", "scan pi", "find pi ip", "tunnel pi dashboard",
  "pi healthz", "pi journalctl", "pi systemd", "tear down pi service", "install rust on pi",
  "pi watch start", "pi tailscale", "polymarket pi", or any Pi hardware/deployment interaction
  for this repo. Does NOT cover camera streams (use pi-camera-stream skill if that ever lands here).
---

## Source of truth

**Always read the canonical target list first** when picking a command:

```
project.json   (top-level Polymarket-Trader project, search for "pi-dev-")
```

The `pi-dev-*` Nx targets are the supported entrypoints. Backing scripts:

| Script | What it owns |
|--------|--------------|
| `scripts/pi_lib.sh` | Sourced library: Pi discovery (ARP→mDNS→hostname→/24 sweep), SSH key resolution, ssh/rsync helpers. Cache: `~/.cache/polymarket-trader/pi-ip` (legacy `~/.cache/night-agent/pi-ip` is read but not written). |
| `scripts/pi_dev_access.sh` | `scan`, `urls`, `tunnel`, `ssh`. |
| `scripts/pi_dev_sync.sh`   | `deploy`, `init-env`, `install-support`, `status`, `health`, `remote-logs`, `down`, `remove`, `watch start/stop/status/logs`. |
| `scripts/pi_dev_tailscale.sh` | Resolves Pi over Tailscale and forwards args to the two scripts above. |
| `infra/systemd/pt-engine-dev.service.template` | systemd unit template; `${PI_SERVICE}/${PI_USER}/${PI_DEST}` are substituted at deploy time. |

## Invoke pattern

All operations run from the workspace root:

```bash
pnpm exec nx run polymarket-trader:pi-dev-<target>
```

Direct shell access for ad-hoc args:

```bash
./scripts/pi_dev_sync.sh   <command> [args]
./scripts/pi_dev_access.sh <command> [args]
PI_TAILSCALE_HOST=<hint> ./scripts/pi_dev_tailscale.sh <command>
```

## Target map

### Discovery & access (`scripts/pi_dev_access.sh`)

| Nx target              | Action |
|------------------------|--------|
| `pi-dev-scan`          | Print Pi IPv4 (ARP cache → mDNS → hostname → /24 ping sweep). Updates `~/.cache/polymarket-trader/pi-ip`. |
| `pi-dev-urls`          | Print direct + tunnel dashboard URLs (LAN). |
| `pi-dev-tunnel`        | `ssh -N -L 18080:127.0.0.1:8080` — interactive, exits on Ctrl-C. |
| `pi-dev-ssh`           | Interactive SSH session (operator types `exit` to leave). |

### Deployment lifecycle (`scripts/pi_dev_sync.sh`)

| Nx target                  | Action |
|----------------------------|--------|
| `pi-dev-install-support`   | Bring up a fresh Pi: `apt` build deps, rustup stable, Node 22, pnpm. Idempotent. |
| `pi-dev-init-env`          | Render `.env.pi` from current shell vars and `scp` it to `$PI_DEST/.env.pi` (chmod 600). |
| `pi-dev-deploy`            | Build dashboard frontend, rsync repo (excluding secrets), build Rust on Pi, install systemd unit, restart, curl `/healthz`. |
| `pi-dev-status`            | Treated as `systemctl status pt-engine-dev` via `pi-dev-watch-status`. |
| `pi-dev-health`            | `curl -fsS http://127.0.0.1:8080/healthz` from inside the Pi over SSH. |
| `pi-dev-down`              | `systemctl stop && disable pt-engine-dev`. Files preserved. |
| `pi-dev-remove`            | Stop + disable + delete unit + `rm -rf $PI_DEST`. Destructive. |

### Continuous sync (`scripts/pi_dev_sync.sh watch …`)

| Nx target                | Action |
|--------------------------|--------|
| `pi-dev-watch-start`     | Background loop: hashes the workspace every `$PI_POLL_SECONDS`, redeploys on change. |
| `pi-dev-watch-status`    | Local watch state + `systemctl status` on the Pi. |
| `pi-dev-watch-logs`      | `tail -f .local/pi-dev-watch.log`. |
| `pi-dev-watch-stop`      | Kill the watch loop. The Pi service keeps running. |

### Tailscale fallback (`scripts/pi_dev_tailscale.sh`)

| Nx target                  | Action |
|----------------------------|--------|
| `pi-dev-deploy-tailscale`  | Resolve via `tailscale ip --4` (falls back to `tailscale status --json`), then `pi-dev-deploy` against `PI_HOST=<resolved>`. |
| `pi-dev-status-tailscale`  | Same resolver, then `pi_dev_sync.sh status`. |

For ad-hoc commands over Tailscale, use the script directly:

```bash
./scripts/pi_dev_tailscale.sh health
./scripts/pi_dev_tailscale.sh -- access urls       # delegate to pi_dev_access.sh
./scripts/pi_dev_tailscale.sh watch start
```

## Common workflows

**First-time bring-up of a fresh Pi:**

```bash
export PI_HOST=<lan-or-tailscale-ip>     # or rely on pi_lib.sh discovery
export PI_USER=pi
cp config/pi.env.example .env             # edit secrets locally
pnpm exec nx run polymarket-trader:pi-dev-install-support
pnpm exec nx run polymarket-trader:pi-dev-init-env
pnpm exec nx run polymarket-trader:pi-dev-deploy
pnpm exec nx run polymarket-trader:pi-dev-health
```

**Iterate while editing locally:**

```bash
pnpm exec nx run polymarket-trader:pi-dev-watch-start
pnpm exec nx run polymarket-trader:pi-dev-watch-status
pnpm exec nx run polymarket-trader:pi-dev-watch-logs
# When done:
pnpm exec nx run polymarket-trader:pi-dev-watch-stop
```

**Pi unreachable on LAN, but on Tailscale:**

```bash
PI_TAILSCALE_HOST=polymarket-pi pnpm exec nx run polymarket-trader:pi-dev-deploy-tailscale
```

**Tear down for the day:**

```bash
pnpm exec nx run polymarket-trader:pi-dev-down       # service stops, files stay
# or, fully:
pnpm exec nx run polymarket-trader:pi-dev-remove     # deletes $PI_DEST and the unit
```

## Discovery rules (pi_lib.sh)

`resolve_pi_host` priority:

1. `PI_HOST` env var (LAN or Tailscale IP/hostname)
2. `PI_IP` env var (set by `pi_dev_tailscale.sh`)
3. Cached IP at `~/.cache/polymarket-trader/pi-ip` (only if it answers SSH)
4. Legacy cache at `~/.cache/night-agent/pi-ip` (read, never written)
5. Active discovery: ARP cache (Pi MAC OUIs) → mDNS `${PI_HOSTNAME}.local` → hostname direct → parallel /24 sweep on the default-route subnet

`resolve_pi_key` priority:

1. `PI_KEY` env var
2. `~/.ssh/id_ed25519_polymarket_trader`
3. `~/.ssh/id_ed25519_raspberrypi`
4. `~/.ssh/id_ed25519`
5. `~/.ssh/id_rsa`

If discovery fails the script prints actionable next steps; do not retry blindly — surface the failure to the user.

## Guidance

- If the user's request maps to a named Nx target, run the Nx target — don't shell out directly.
- If no target fits, run the script directly with the appropriate command.
- Before `pi-dev-deploy`, confirm `pi-dev-health` is currently failing or that you intend to redeploy; deploy is destructive on the Pi-side `target/` directory due to rebuild.
- Interactive targets (`pi-dev-ssh`, `pi-dev-tunnel`, `pi-dev-watch-logs`, `pi-dev-remote-logs`) block — tell the user they need to type `exit` or `Ctrl-C` to release the terminal.
- Secrets stay out of the repo: the rsync exclude list drops `.env*`, `config/config.toml`, and credential JSON files. Real secrets live in `$PI_DEST/.env.pi` written by `pi-dev-init-env`.
- Do not raise `engine.mode` to `live` from this skill — that is an explicit operator action gated by `pt-cli preflight-live`.
