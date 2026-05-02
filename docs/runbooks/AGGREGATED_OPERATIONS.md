# Aggregated operations runbook

Single entry point for operators and maintainers. Deep detail stays in linked docs.

## 1) Preconditions

- Rust toolchain (`cargo`, `rustfmt`, `clippy`) per [LOCAL_VALIDATION.md](../LOCAL_VALIDATION.md).
- `python3` for strategy lab and replay tooling.
- Local secrets via environment variables (see root `README.md` and `.env.example`). Do not commit `config/config.toml` with live material.

## 2) Config bootstrap (fresh clone)

```bash
cp config/config.example.toml config/config.toml
cp config/coinbase_strategy_lab.example.json config/coinbase_strategy_lab.json
cp config/prompt_bundle.example.json config/prompt_bundle.json
# Optional: cp .env.example .env
```

## 3) Pre-merge / pre-release validation

```bash
./scripts/local_validation_ladder.sh
```

Optional frontend gate: `RUN_FRONTEND=1 ./scripts/local_validation_ladder.sh` (see [LOCAL_VALIDATION.md](../LOCAL_VALIDATION.md)).

## 4) Startup modes

**Full engine + ops dashboard** (matches ladder “runtime smoke”):

```bash
cargo run -p pt-cli -- run --config config/config.toml
```

**Coinbase workstation** (paper / replay / live per `--mode`):

```bash
cargo run -p pt-cli -- coinbase up --config config/config.toml --mode paper
```

Dashboard bind defaults are in `config/config.toml` → `[ops].dashboard_bind` (see `config.example.toml`).

**Strategy lab** (backtest / overlap / optimize / dashboard):

```bash
python3 tools/coinbase_strategy_lab.py backtest --config config/coinbase_strategy_lab.json
python3 tools/coinbase_strategy_lab.py dashboard --config config/coinbase_strategy_lab.json --serve 9090
```

## 5) Live readiness

```bash
cargo run -p pt-cli -- preflight-live --config config/config.toml --timeout-ms 3000
cargo run -p pt-cli -- coinbase preflight --config config/config.toml --mode live --timeout-ms 3000
```

**Tiny live pilot** (threshold script):

```bash
./scripts/tiny_live_pilot.sh config/config.toml 3000
```

See [TINY_LIVE_PILOT.md](../TINY_LIVE_PILOT.md).

## 6) Deployment (single EC2 host)

High level:

1. Provision host and security groups (SSH, dashboard bind port from config — commonly `8080` — restricted to your IP/VPN).
2. `./scripts/bootstrap_ubuntu.sh`
3. Inject secrets via environment / host manager (not git).
4. `./scripts/build_release_bundle.sh` then `./scripts/deploy_ec2.sh …` per [DEPLOYMENT.md](../../DEPLOYMENT.md).
5. `systemctl` restart of `pt-engine` where configured.

## 7) Health and metrics

Prefer the OpenAPI document for exact paths. Typical smoke:

```bash
curl -fsS http://127.0.0.1:8080/health
curl -fsS http://127.0.0.1:8080/healthz
curl -fsS http://127.0.0.1:8080/ready
curl -fsS http://127.0.0.1:8080/metrics | head
curl -fsS http://127.0.0.1:8080/state/risk | jq
```

Replace host/port if `dashboard_bind` differs.

## 8) Emergency controls

```bash
curl -fsS -X POST http://127.0.0.1:8080/ops/halt
curl -fsS -X POST http://127.0.0.1:8080/ops/resume
curl -fsS -X POST http://127.0.0.1:8080/ops/flatten
```

## 9) Incident triage (short)

1. `GET /state/risk` and kill-switch fields.
2. `GET /state/executions` for recent activity.
3. `GET /state/books` / `GET /state/history` for feed sanity.
4. Workstation orders: `GET /api/v1/orders` (see OpenAPI for schema).
5. Preserve context: `./scripts/save_context.sh "note" docs/SESSION_CONTEXT.md config/config.toml`
6. If unsafe: **halt** first, then rollback binary/config per [DEPLOYMENT.md](../../DEPLOYMENT.md) §8.

## 10) Promotion and replay evidence

```bash
./scripts/promote_strategy_lab.sh data/strategy_lab/<report>.json BTC-USD sma_baseline
./scripts/replay_acceptance.sh data/replay/strategy_lab_promoted.ndjson data/tuning/strategy_lab_promoted.json data/output/engine.sqlite
```

Adjust paths to your artifacts; SQLite is optional.

## 11) Evidence bundles (optional program gate)

See [RUNBOOK.md](../RUNBOOK.md) for `phase1_evidence_bundle.sh` and `tools/phase1_gate_report.py` when collecting formal evidence directories.

## 12) External AI handoff

```bash
./scripts/export_prompt_bundle.sh
```

See [PROMPT_BUNDLE.md](../PROMPT_BUNDLE.md).
