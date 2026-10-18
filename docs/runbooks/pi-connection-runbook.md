# Raspberry Pi Connection Runbook

**Hostname:** `night-agent` (`night-agent.local` on LAN)  
**Primary user:** `evan`  
**SSH key:** `~/.ssh/id_ed25519_raspberrypi`  
**Tailscale IP:** `100.69.84.33`  
**Tailnet:** `mccallevansoftware@`  
**Funnel:** `https://homoplasticus-pi.tail20da7c.ts.net`

---

## Connection methods

### 1. LAN (same network) — preferred

```bash
ssh -i ~/.ssh/id_ed25519_raspberrypi evan@night-agent.local
```

Or by cached IP (may change on DHCP renewal):

```bash
ssh -i ~/.ssh/id_ed25519_raspberrypi evan@192.168.4.47
```

**Status as of 2026-05-18:** confirmed working, ~7ms RTT.

---

### 2. Off-LAN via Tailscale

Tailscale is installed and connected on the Pi as `homoplasticus-pi`. The `--ssh` flag is enabled, which means Tailscale intercepts SSH connections and may require a one-time browser authorization on first use from a new client.

```bash
ssh -i ~/.ssh/id_ed25519_raspberrypi evan@100.69.84.33
```

If you see `Tailscale SSH requires an additional check`, visit the URL printed in the terminal to authorize the session in your browser. This is a Tailscale ACL check, not a key problem.

To bypass Tailscale SSH interception (use raw SSH over Tailscale network instead):

```bash
# Re-up without --ssh flag
pnpm exec nx run raspberry-pi:tailscale:up
# Then SSH normally
ssh -i ~/.ssh/id_ed25519_raspberrypi evan@100.69.84.33
```

---

### 3. Nx targets (preferred for automation)

```bash
# Check Tailscale status + IP
pnpm exec nx run raspberry-pi:tailscale:status

# Get just the IP
pnpm exec nx run raspberry-pi:tailscale:ip

# Bring Tailscale up (interactive re-auth)
pnpm exec nx run raspberry-pi:tailscale:up

# Bring Tailscale up with auth key (non-interactive / CI)
bash support-scripts/deployment/raspberry-pi/pi-tailscale.sh up --authkey <TS_AUTHKEY>
```

---

## Diagnosing connectivity

### Pi unreachable on LAN?

1. Ping: `ping -c 3 night-agent.local`
2. Check cached IP: `cat ~/.cache/night-agent/pi-ip`
3. Try Tailscale IP directly: `ping 100.69.84.33`
4. Check Tailscale from Mac: `tailscale status`

### SSH key rejected?

```bash
# Verify key file exists
ls -la ~/.ssh/id_ed25519_raspberrypi

# Test verbose SSH
ssh -vvv -i ~/.ssh/id_ed25519_raspberrypi evan@night-agent.local
```

If key is missing, re-import it or generate a new one and copy it to the Pi:

```bash
ssh-copy-id -i ~/.ssh/id_ed25519_raspberrypi.pub evan@night-agent.local
```

### Tailscale not connected?

```bash
pnpm exec nx run raspberry-pi:tailscale:status
```

If status shows offline:

```bash
# Re-authenticate interactively (follow the URL printed)
pnpm exec nx run raspberry-pi:tailscale:up

# Or with a key from https://login.tailscale.com/admin/settings/keys
bash support-scripts/deployment/raspberry-pi/pi-tailscale.sh up --authkey tskey-auth-...
```

---

## Agent CA / ephemeral credentials (for agentuser)

The Pi has a separate restricted `agentuser` account for AI agent access, authenticated via SSH certificates (6-hour validity).

```bash
# Check existing certs
pnpm exec nx run raspberry-pi:ssh-ca:info

# Issue a fresh 6-hour cert
pnpm exec nx run raspberry-pi:ssh-ca:issue

# Connect as agentuser (use the key path printed by issue-cert)
ssh -i ~/.ssh/pi-agent-certs/agent-<timestamp> agentuser@night-agent.local
```

Certs expire automatically — no revocation needed after 6 hours. Run `ssh-ca:issue` any time you need fresh access.

---

## Environment variables (`.env`)

These control all Pi scripts:

| Variable | Current value | Purpose |
|----------|--------------|---------|
| `PI_USER` | `evan` | SSH login user |
| `PI_HOSTNAME` | `night-agent.local` | mDNS hostname for LAN access |
| `PI_TAILSCALE_IP` | `100.69.84.33` | Tailscale IP for off-LAN fallback |
| `PI_SSH_KEY` | `~/.ssh/id_ed25519_raspberrypi` | SSH private key path |

---

## Funnel endpoints

The Pi exposes services publicly via Tailscale Funnel:

| URL | Purpose |
|-----|---------|
| `https://homoplasticus-pi.tail20da7c.ts.net` | Root (port 443) |
| `https://homoplasticus-pi.tail20da7c.ts.net:8443` | Port 8443 |

To reconfigure funnel routes, SSH into the Pi and run `tailscale funnel`.
