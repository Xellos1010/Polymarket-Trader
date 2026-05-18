# Pi Operator Path: Cloudflare Tunnel Topology and Fallback

Issue #83 — Epic #68: Webhook and Pi-hosted operator path hardening.

## Overview

The Raspberry Pi is the primary operator workstation for receiving TradingView webhooks and running `pt-engine` in Paper mode. Direct LAN access is the normal path; Cloudflare Tunnel is the fallback when the Pi is not reachable via LAN (e.g., when operating remotely or when mDNS/ARP resolution fails).

This document describes the network topology, the SSH tunnel alternative, and the fallback drill procedure.

---

## Topology

```
TradingView (cloud)
        │  HTTPS POST /tradingview
        ▼
Cloudflare Tunnel (cloudflared daemon on Pi)
        │  localhost:8080
        ▼
pt-engine (axum, 0.0.0.0:8080 on Pi)
        │  parsed signal
        ▼
Approval queue → operator review → Paper order
```

```
Operator workstation (Mac)
        │  SSH over Cloudflare WARP or Tailscale
        ▼
Pi (sshd, port 22)
        │  pnpm exec nx run polymarket-trader:pi-dev-ssh
        ▼
pt-engine logs / systemctl status pt-engine-dev
```

### Components

| Component | Host | Port | Purpose |
|-----------|------|------|---------|
| `pt-engine` axum server | Pi (`0.0.0.0`) | `8080` | Receives TradingView webhooks |
| `cloudflared` tunnel daemon | Pi | — | Exposes `localhost:8080` to a stable `*.trycloudflare.com` or named-tunnel HTTPS URL |
| SSH jump / port-forward | Pi | `22` | Operator terminal access; LAN tunnel `ssh -N -L 18080:127.0.0.1:8080` |
| Tailscale (optional) | Pi + Mac | — | VPN-layer fallback when LAN mDNS fails |

---

## Normal Path (LAN)

1. Pi is on the local network (same subnet as operator Mac).
2. `pi_lib.sh` resolves Pi IP via ARP → mDNS → hostname → /24 sweep.
3. All `pi-dev-*` Nx targets work directly.
4. Dashboard reachable at `http://<pi-ip>:8080`.

```bash
pnpm exec nx run polymarket-trader:pi-dev-scan    # confirm Pi IP cached
pnpm exec nx run polymarket-trader:pi-dev-health  # curl /healthz
pnpm exec nx run polymarket-trader:pi-dev-urls    # print dashboard + tunnel URLs
```

---

## Cloudflare Tunnel Path (Remote / Off-LAN)

### One-time setup on Pi (run once, not automated by deploy scripts)

```bash
# On Pi — install cloudflared
curl -L https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-arm64.deb \
     -o /tmp/cloudflared.deb
sudo dpkg -i /tmp/cloudflared.deb

# Quick anonymous tunnel (no Cloudflare account required — URL rotates on restart)
cloudflared tunnel --url http://localhost:8080
# Note the *.trycloudflare.com URL printed to stdout — paste it into TradingView alert webhook URL.
```

### Named tunnel (stable URL, requires Cloudflare account)

1. `cloudflared tunnel login` — authenticates and writes `~/.cloudflared/cert.pem`.
2. `cloudflared tunnel create pt-engine` — creates a named tunnel; note the tunnel UUID.
3. Configure `~/.cloudflared/config.yml`:
   ```yaml
   tunnel: <tunnel-uuid>
   credentials-file: /home/pi/.cloudflared/<tunnel-uuid>.json
   ingress:
     - hostname: pt-engine.<your-zone>.workers.dev
       service: http://localhost:8080
     - service: http_status:404
   ```
4. `cloudflared tunnel route dns pt-engine pt-engine.<your-zone>` — creates CNAME.
5. `cloudflared tunnel run pt-engine` — start; or install as systemd service:
   ```bash
   sudo cloudflared service install
   sudo systemctl enable --now cloudflared
   ```

### TradingView webhook URL

Set the alert webhook URL to the Cloudflare Tunnel HTTPS URL + `/tradingview`:

```
https://pt-engine.<your-zone>.workers.dev/tradingview
```

or for the ephemeral anonymous tunnel:

```
https://<random>.trycloudflare.com/tradingview
```

Include the `x-tv-secret` header in every TradingView alert (set in the alert's "Message" or "Webhook URL" header config):

```json
{"action": "buy", "ticker": "BTC-USD", "x-tv-secret": "<your-endpoint-secret>"}
```

---

## Tailscale Fallback Path

When LAN mDNS fails but Pi has network access (e.g., behind NAT, different subnet, VPN):

```bash
# Resolve Pi IP via Tailscale and deploy
PI_TAILSCALE_HOST=<tailscale-hostname-or-ip> \
  pnpm exec nx run polymarket-trader:pi-dev-deploy-tailscale

# Ad-hoc commands
PI_TAILSCALE_HOST=<tailscale-hostname-or-ip> ./scripts/pi_dev_tailscale.sh health
PI_TAILSCALE_HOST=<tailscale-hostname-or-ip> ./scripts/pi_dev_tailscale.sh ssh
```

Tailscale hostname is typically the Pi's machine name as shown in `tailscale status`.

---

## SSH Port-Forward (LAN Tunnel)

Opens `localhost:18080` on the operator Mac → `pt-engine` on the Pi. Useful for inspecting the dashboard without exposing the Pi externally:

```bash
pnpm exec nx run polymarket-trader:pi-dev-tunnel
# Blocks — press Ctrl-C to release.
# Dashboard: http://localhost:18080
```

---

## Fallback Drill Procedure

Run this drill whenever the Pi is not reachable on LAN before going into a live paper session:

### Deterministic local drill

Run the local resilience drill first. It exercises the in-process TradingView webhook handler, HMAC rejection, nonce replay rejection, IP allowlist rejection, and a 50-message unique-nonce burst without requiring a reachable Pi or live credentials:

```bash
./scripts/webhook_resilience_drill.sh
```

Optional staging probe, only when a sandbox/paper Pi or tunnel endpoint is intentionally available:

```bash
WEBHOOK_DRILL_URL=https://<tunnel-url> \
WEBHOOK_DRILL_SECRET=<local-or-pi-endpoint-secret> \
  ./scripts/webhook_resilience_drill.sh
```

Expected local result: all `pt-engine` `webhook_` tests pass, including `webhook_burst_with_unique_nonces_stays_reliable`.

1. **Confirm Pi is unreachable on LAN:**
   ```bash
   pnpm exec nx run polymarket-trader:pi-dev-scan
   # Expected: "Pi not found" or no IP cached
   ```

2. **Connect via Tailscale (if available):**
   ```bash
   PI_TAILSCALE_HOST=<pi-tailscale-name> ./scripts/pi_dev_tailscale.sh health
   # Expected: {"status":"ok"} — proceed to step 3.
   # If Tailscale also fails, proceed to manual SSH.
   ```

3. **Manual SSH via known IP or Cloudflare Access SSH:**
   ```bash
   ssh -i ~/.ssh/id_ed25519_raspberrypi pi@<known-ip>
   # Inside Pi — check service:
   systemctl status pt-engine-dev
   journalctl -u pt-engine-dev -n 50 --no-pager
   ```

4. **Update TradingView webhook URL:**
   - If named Cloudflare Tunnel is running, the URL is stable — no change needed.
   - If using ephemeral tunnel, restart `cloudflared tunnel --url http://localhost:8080` on the Pi, note the new URL, and update TradingView alert webhook URL.

5. **Verify end-to-end:**
   ```bash
   # From operator Mac or any machine:
   curl -X POST https://<tunnel-url>/tradingview \
     -H "x-tv-secret: <your-endpoint-secret>" \
     -H "Content-Type: application/json" \
     -d '{"action":"test","ticker":"BTC-USD"}'
   # Expected: 200 with signal-queued response or approval-queue entry logged.
   ```

6. **Document the fallback in session notes** — record which path was used and whether the LAN issue was transient or requires Pi network config fix.

---

## Guardrails

- Do not put `endpoint_secret` in any repo-tracked file. It lives in `config/config.toml` (excluded from rsync by `.env*` and explicit exclude rules) or in `$PI_DEST/.env.pi` written by `pi-dev-init-env`.
- Do not enable `engine.mode = "live"` without completing the pre-flight checklist in `pt-cli preflight-live`.
- The Cloudflare Tunnel URL is effectively public — `x-tv-secret` header enforcement in `pt-engine` is the primary gate. HMAC replay protection (issue #85) adds a second layer.
- Sandbox/paper-only constraints apply on the Pi — see `WORK_STATUS.json` guardrails.

---

## See Also

- Pi deployment lifecycle: `docs/runbooks/AGGREGATED_OPERATIONS.md`
- Webhook HMAC hardening: `docs/development/INSTRUCTIONS.md` (issue #85)
- Pi deploy Nx targets: `.claude/skills/pi-ops/skill.md`
- TradingView alert configuration: `docs/development/RUNBOOK.md`
