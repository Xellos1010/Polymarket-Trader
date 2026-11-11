# Infra

This directory is the source of truth for host-level deployment artifacts used by the Raspberry Pi dev path.

It intentionally mirrors the useful part of the external Homo Plasticus `infra/` layout:

- `infra/systemd/` holds service templates.
- `infra/pi/` holds Pi-specific env examples.
- `infra/project.json` exposes Nx targets to render and inspect those assets.

This repo does not port the external AWS or Terraform layers because the active deployment path here is the barebones Pi flow:

- local frontend build
- `rsync` to the Pi
- on-device `cargo build`
- `systemd` service for `pt-cli`
- optional `cloudflared` adapter service for remote frontend access

Useful commands:

```bash
pnpm exec nx run infra:build
pnpm exec nx run infra:print
pnpm exec nx run infra:print-service
pnpm exec nx run infra:print-cloudflare
```

Rendered artifacts land in `.local/infra/` and are consumed by the Pi deployment scripts.
