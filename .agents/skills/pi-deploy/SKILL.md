---
name: pi-deploy
description: >
  End-to-end runbook for deploying the Polymarket-Trader Rust engine + dashboard to a Raspberry Pi
  dev target. Use when the user says "deploy to pi", "ship to pi", "push to pi", "set up pi service",
  "pi systemd", "rebuild on pi", "pi watch", "redeploy pi", "tear down pi service", "pi unreachable",
  or asks to verify the Pi-side `pt-engine-dev` service. This skill orchestrates: SSH key + IP
  discovery, fresh-Pi support install, secret-aware sync, systemd unit rendering, build, restart,
  and `/healthz` verification — and tells you how to respond when the Pi is unreachable. Always
  load alongside `pi-ops` (which is the surface index of all `pi-dev-*` Nx targets).
---

## Goal

Take a clean code change in the local Polymarket-Trader workspace and reach a green
`http://${PI_HOST}:8080/healthz` on the target Raspberry Pi, with secrets staying off
disk in the repo and the systemd unit installed from a reviewable template.

## Prerequisites checklist

Before running ANY deploy command, confirm:

- [ ] You can resolve the Pi: `pnpm exec nx run polymarket-trader:pi-dev-scan` returns an IP that answers SSH.
- [ ] An SSH key at one of the paths `pi_lib.sh` checks (`~/.ssh/id_ed25519_polymarket_trader`, `~/.ssh/id_ed25519_raspberrypi`, `~/.ssh/id_ed25519`, `~/.ssh/id_rsa`) — or `PI_KEY=/path/...` is exported.
- [ ] `.env` file exists locally for `dotenvy` to pick up secrets if needed (copy from `config/pi.env.example`). The repo's `.gitignore` excludes it.
- [ ] If first time on this Pi: have run `pi-dev-install-support` to provision rustup + Node 22 + pnpm.

## Deployment flow

### Step 1 — Resolve the Pi

```bash
pnpm exec nx run polymarket-trader:pi-dev-scan
```

If the LAN scan fails:

1. Try Tailscale: `PI_TAILSCALE_HOST=<dns-or-host-hint> ./scripts/pi_dev_tailscale.sh ssh`
2. If that fails too, surface the failure to the user (do not loop). The Pi is offline or behind a different network.

### Step 2 — Provision (only on a fresh board)

```bash
pnpm exec nx run polymarket-trader:pi-dev-install-support
```

Installs `build-essential`, `pkg-config`, `libssl-dev`, rustup stable, Node 22, pnpm. Idempotent.

### Step 3 — Push secrets

```bash
# In the calling shell, export only what you intend to set (everything else is empty in .env.pi):
export POLYMARKET_PRIVATE_KEY=...
export COINBASE_API_KEY=...
export COINBASE_API_SECRET=...
export TRADINGVIEW_ENDPOINT_SECRET=...
pnpm exec nx run polymarket-trader:pi-dev-init-env
```

This writes `$PI_DEST/.env.pi` with mode `0600`. The repo never contains a copy.

### Step 4 — Deploy

```bash
pnpm exec nx run polymarket-trader:pi-dev-deploy
```

The script:

1. `pnpm run build` for the dashboard frontend (local).
2. `rsync -az --delete` of the workspace, excluding `.env*`, `config/config.toml`, credential JSON files, `target/`, `node_modules/`, etc.
3. `cargo build -p pt-cli` on the Pi (or `cargo build --release` if `PI_BUILD_PROFILE=release`).
4. Renders `infra/systemd/pt-engine-dev.service.template` with `${PI_SERVICE}/${PI_USER}/${PI_DEST}` substitution.
5. `sudo cp` the rendered unit, `daemon-reload`, `enable`, `restart`.
6. `curl -fsS $PI_HEALTH_URL` (default `http://127.0.0.1:8080/healthz`).

### Step 5 — Verify

```bash
pnpm exec nx run polymarket-trader:pi-dev-health
pnpm exec nx run polymarket-trader:pi-dev-urls
```

Then optionally tail logs in another shell:

```bash
pnpm exec nx run polymarket-trader:pi-dev-watch-logs       # local watch loop log
./scripts/pi_dev_sync.sh remote-logs                        # journalctl -u pt-engine-dev -f
```

## Iterate

For a tight inner loop:

```bash
pnpm exec nx run polymarket-trader:pi-dev-watch-start
```

This polls `$PI_POLL_SECONDS` (default 2s), shashes the workspace, and redeploys when it changes. Stop with `pi-dev-watch-stop`.

## Tailscale fallback flow

When LAN is unreachable but Tailscale is up:

```bash
export PI_TAILSCALE_HOST=polymarket-pi   # or the Pi's tailnet hostname
pnpm exec nx run polymarket-trader:pi-dev-deploy-tailscale
pnpm exec nx run polymarket-trader:pi-dev-status-tailscale
```

The wrapper resolves a stable 100.x IPv4 and sets `PI_HOST` for the inner script. All other targets work via the script directly:

```bash
./scripts/pi_dev_tailscale.sh health
./scripts/pi_dev_tailscale.sh -- access urls
```

## Tear-down

| Goal | Command |
|------|---------|
| Stop the service, keep files | `pnpm exec nx run polymarket-trader:pi-dev-down` |
| Remove the deploy directory and unit | `pnpm exec nx run polymarket-trader:pi-dev-remove` |

`pi-dev-remove` is destructive (deletes `$PI_DEST`). Confirm with the user before running.

## Failure-mode playbook

| Symptom | Likely cause | Action |
|---------|--------------|--------|
| `Could not locate the Pi on the local network` | Pi is off, on a different LAN, or stale ARP cache | Try Tailscale (`pi-dev-deploy-tailscale`); ask the user for the Pi's current IP |
| SSH key not found | No `~/.ssh/id_ed25519*` and `PI_KEY` unset | Ask user to set `PI_KEY=/path/to/key` or generate one |
| `cargo build` slow | First build on a Pi is 5–15 min for `pt-cli` | Acceptable; use `PI_BUILD_PROFILE=release` only when intentionally smoke-testing release perf |
| `curl /healthz` fails after restart | Engine fails fast on missing config or bad bind | `./scripts/pi_dev_sync.sh remote-logs` and inspect; check `.env.pi` was written |
| Port 8080 already bound | Another listener (old service, dashboard already running) | `sudo systemctl stop pt-engine-dev` on the Pi, retry; or change `PI_PUBLIC_PORT` |
| `tailscale ip --4 <hint>` returns nothing | Hostname not on tailnet, or device offline | Run `tailscale status` locally; ask user to verify |

## Things this skill does NOT do

- Live trading. `engine.mode = "live"` is an explicit operator decision gated by `cargo run -p pt-cli -- preflight-live`. This skill never sets it.
- Production deployment. `pt-engine-dev` is a dev unit; production uses `scripts/pt-engine.release.service` and the EC2 path in `scripts/deploy_ec2.sh`.
- Secret generation. Operators bring their own keys; this skill only forwards them through `init-env`.

## After-deploy summary template

When the deploy completes, report to the user:

```
Pi: <user>@<host>
Service: pt-engine-dev (active/running)
Health: http://<host>:8080/healthz   HTTP 200
Dashboard: http://<host>:8080/
Logs: pnpm exec nx run polymarket-trader:pi-dev-watch-logs
Remote logs: ./scripts/pi_dev_sync.sh remote-logs
Tear down: pnpm exec nx run polymarket-trader:pi-dev-down
```
