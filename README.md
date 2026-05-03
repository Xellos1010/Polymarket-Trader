# Polymarket Trader (Rust)

Rust-first trading workspace with a Coinbase-native workstation, replay/paper/live modes, legacy Polymarket support, hard risk controls, and operator tooling.

**Direction and phased outcomes:** [`ROADMAP.md`](ROADMAP.md) is the source of truth for where the project is going. **Current execution stage** (active issue, queue, validation): [`docs/development/WORK_STATUS.md`](docs/development/WORK_STATUS.md).

## Workspace crates

- `pt-core`: shared config, domain types, metrics, math, errors.
- `pt-market-discovery`: paginated Gamma market discovery and filtering/tiering.
- `pt-polymarket`: Polymarket REST + websocket orderbook client.
- `pt-coinbase`: Coinbase hedge adapter interfaces and implementations.
- `pt-kraken`, `pt-gemini`: optional REST clients for additional spot venues (see `config.example.toml`).
- `pt-wallet-intel`: wallet discovery, profile/positions/trades ingestion, bias scoring.
- `pt-signal`: wallet + TradingView bias fusion.
- `pt-quote`: quote intent generation and edge gate support.
- `pt-risk`: risk limits and kill-switch state machine.
- `pt-replay`: replay/paper simulation engine.
- `pt-route`: cross-venue route opportunity scan helpers.
- `pt-order-manager`: resting-order preview/reprice/cancel-replace policy helpers.
- `pt-strategy-lab`: strategy lab engine (indicators, backtest, optimize) and Axum `/lab/*` API surface (used from tooling and tests; no dedicated `pt-cli` server subcommand today).
- `pt-engine`: async orchestration runtime and task graph.
- `pt-dashboard`: health/metrics/risk/ops HTTP endpoints and bundled operator UI.
- `pt-cli`: command-line entrypoint.

## Quick start (once Rust is installed)

```bash
cp config/config.example.toml config/config.toml
# Optional: cp .env.example .env
cargo run -p pt-cli -- coinbase up --config config/config.toml --mode paper
```

Full engine + dashboard (matches `./scripts/local_validation_ladder.sh` runtime smoke):

```bash
cargo run -p pt-cli -- run --config config/config.toml
```

Open dashboard (default bind from `config.example.toml` → `[ops].dashboard_bind`):

```text
http://127.0.0.1:8080/
```

## Monorepo setup with Nx and pnpm

The repository root is a **pnpm workspace** with **Nx** used for task orchestration (Rust `cargo` targets on the workspace root project, and optional Vite/Vitest targets for the dashboard frontend). Rust remains the primary toolchain; Node is required to run Nx and frontend checks.

### Prerequisites

- **Rust** — stable toolchain with `cargo` (see [rustup](https://rustup.rs/)).
- **Node.js** — v20 or newer (`package.json` `engines.node`).
- **pnpm** — v9 or newer; the repo pins a version via `packageManager` in `package.json`. Enable Corepack so that version is used automatically:

  ```bash
  corepack enable
  ```

### Install

From the repository root:

```bash
pnpm install
```

That installs Nx and wires the workspace packages defined in `pnpm-workspace.yaml` (the root package and `crates/pt-dashboard/frontend`).

Confirm Nx sees both projects:

```bash
pnpm exec nx show projects
```

You should see `polymarket-trader` and `pt-dashboard-frontend`.

### Common Nx commands

Rust pipeline (fmt, check, clippy, test, build) on the main workspace:

```bash
pnpm verify
# equivalent:
pnpm exec nx run-many -t fmt check clippy test build --projects=polymarket-trader
```

Full local validation ladder (matches `./scripts/local_validation_ladder.sh`):

```bash
pnpm verify:full
# equivalent:
pnpm exec nx run polymarket-trader:local-validation
```

With dashboard frontend checks (requires `npm install` inside `crates/pt-dashboard/frontend` first, or set `RUN_FRONTEND=1` when using the shell script — see `docs/LOCAL_VALIDATION.md`):

```bash
pnpm exec nx run polymarket-trader:local-validation-frontend
```

Individual dashboard targets after installing frontend dependencies:

```bash
cd crates/pt-dashboard/frontend && npm install && cd ../../..
pnpm exec nx run pt-dashboard-frontend:typecheck
pnpm exec nx run pt-dashboard-frontend:test
pnpm exec nx run pt-dashboard-frontend:build
```

Project layout: root `project.json` defines `polymarket-trader` targets; `crates/pt-dashboard/frontend/project.json` defines `pt-dashboard-frontend`. See `AGENTS.md` for the full verification matrix.

If pnpm reports ignored postinstall scripts for `nx` or `esbuild`, run `pnpm approve-builds` once and select the packages you trust so optional native installs can complete.

## Local validation

Use the canonical local-first ladder before merge or deployment decisions:

```bash
./scripts/local_validation_ladder.sh
```

Guide:

- `docs/LOCAL_VALIDATION.md`

## Developer setup

Install local git hooks:

```bash
./scripts/install_git_hooks.sh
```

For Nx and the pnpm workspace, follow **[Monorepo setup with Nx and pnpm](#monorepo-setup-with-nx-and-pnpm)** above.

Strategy lab UI and parity with CI use the Python driver (install `python3`; optional frontend work needs Node — see `docs/LOCAL_VALIDATION.md`).

## Environment variables

Use `.env.example` as the baseline for local or dev shells.

| Variable | Required | Purpose |
|---|---|---|
| `RUST_LOG` | No | Log level/filter for runtime diagnostics |
| `PT_CONFIG_PATH` | No | Optional override path to config TOML when the CLI default `config/config.toml` is used |
| `POLYMARKET_PRIVATE_KEY` | Live only | Wallet key material (inject securely) |
| `COINBASE_API_KEY` | Live only | Coinbase API key |
| `COINBASE_API_SECRET` | Live only | Coinbase API secret |
| `COINBASE_PASSPHRASE` | Optional | Coinbase passphrase for key types that require it |
| `TRADINGVIEW_ENDPOINT_SECRET` | Optional | Secret for TradingView webhook auth |

## Operator endpoints

Slim dashboard and workstation endpoints shipped by `pt-dashboard` are documented in **`docs/api/dashboard-openapi.yaml`** (title/version there are canonical). Commonly used paths include:

- `GET /` dashboard UI and static assets
- `GET /health`, `GET /healthz`, `GET /ready`, `GET /metrics`
- `GET /state/risk`, `/state/books`, `/state/markets`, `/state/history`, `/state/executions`, `/state/bias`, `/state/inventory`
- `POST /ops/halt`, `/ops/resume`, `/ops/flatten`
- `GET /api/v1/products`, `/api/v1/scanner`, `/api/v1/products/{product_id}`
- `GET|POST /api/v1/orders`, `POST /api/v1/orders/{order_id}/cancel`
- `GET /api/v1/strategies`, `POST /api/v1/mode`, `POST /api/v1/live/arm`, `POST /api/v1/live/disarm`
- `POST /api/v1/strategy-lab/import`

## `pt-cli` commands

Global option: `-c/--config <path>` (default `config/config.toml`). `PT_CONFIG_PATH` applies when you leave the default path unchanged (see `pt-cli` source).

| Subcommand | Role |
|---|---|
| `run` | Start `TradingEngine` from TOML (`engine.mode` selects paper/replay/live; dashboard on `ops.dashboard_bind`). |
| `status` | HTTP GET of `--url` (default `http://127.0.0.1:8080/health`) and print the body. |
| `preflight-live` | Live prerequisite checks (`--timeout-ms`). |
| `coinbase up` | Coinbase workstation (`--mode paper\|replay\|live`). |
| `coinbase preflight` | Workstation preflight (`--mode`, `--timeout-ms`). |
| `pine-params` | Extract tunable Pine inputs to JSON (`--path`, `--out`). |
| `tune-pine` | Random search over Pine parameters (`--path`, `--iterations`, `--top-k`, optional `--evaluate-cmd`, `--out`). |
| `save-context` | Write operator session context markdown (`--out`, optional `--note`). |
| `scan-markets` | Maker/spread scan (`--limit`, `--top`, cost estimate flags). |

**Strategy lab** (backtest, overlap, optimize, optional static server on a port): `python3 tools/coinbase_strategy_lab.py …` — see `docs/STRATEGY_LAB.md` and `docs/LOCAL_VALIDATION.md`.

**Replay acceptance** (promotion vs replay NDJSON, optional SQLite evidence): `./scripts/replay_acceptance.sh <replay.ndjson> [promotion.json] [engine.sqlite]`

### Common recipes

- Coinbase workstation paper: `cargo run -p pt-cli -- coinbase up --config config/config.toml --mode paper`
- Replay workstation: `cargo run -p pt-cli -- coinbase up --config config/config.toml --mode replay`
- Live preflight (workstation): `cargo run -p pt-cli -- coinbase preflight --config config/config.toml --mode live --timeout-ms 3000`
- Live preflight (standalone): `cargo run -p pt-cli -- preflight-live --config config/config.toml --timeout-ms 3000`
- Pine extract / tune: `cargo run -p pt-cli -- pine-params --path pine-scripts/<script> --out data/tuning/pine_params.json` and `cargo run -p pt-cli -- tune-pine --path pine-scripts/<script> --iterations 100 --top-k 10`
- Tuning with bundled evaluator: `PT_EVAL_OHLCV=data/ohlcv/btc_1m.csv cargo run -p pt-cli -- tune-pine --path pine-scripts/<script> --iterations 200 --top-k 20 --evaluate-cmd "python3 tools/evaluate_candidate.py --fee-bps 2.0 --slippage-bps 1.0 --fixed-trade-cost 0.00005 --price-col close --timestamp-col ts"`
- OHLCV fetch: `python3 tools/fetch_ohlcv.py --provider coinbase --symbol BTCUSD --interval 1m --limit 300 --out data/ohlcv/btcusd_1m.csv` (or `--provider kraken`)
- Strategy lab config + dashboard: `cp config/coinbase_strategy_lab.example.json config/coinbase_strategy_lab.json` then `python3 tools/coinbase_strategy_lab.py dashboard --config config/coinbase_strategy_lab.json --serve 9090`
- Promote strategy-lab JSON to replay: `./scripts/promote_strategy_lab.sh data/strategy_lab/<report>.json BTC-USD sma_baseline`
- Promote Pine tuning winner: `./scripts/promote_candidate.sh data/tuning/pine_tuning_results.json data/tuning/promoted_candidate.json BTC 15m`
- Paper soak (long-running): `./scripts/paper_soak.sh 86400 30 config/config.toml`
- Session context: `cargo run -p pt-cli -- save-context --config config/config.toml --out docs/SESSION_CONTEXT.md --note "checkpoint note"`
- Prompt bundle: `cp config/prompt_bundle.example.json config/prompt_bundle.json` then `./scripts/export_prompt_bundle.sh`

## Schemas and contracts

- OpenAPI spec: `docs/api/dashboard-openapi.yaml`
- Config schema: `schemas/config.schema.json`
- TradingView webhook schema: `schemas/tradingview-webhook.schema.json`
- Coinbase strategy lab config schema: `schemas/coinbase_strategy_lab.schema.json`
- Strategy lab engine schema: `schemas/strategy_lab.schema.json`
- Prompt bundle config schema: `schemas/prompt_bundle.schema.json`
- Runtime data model summary: `docs/data/SCHEMA.md`

## SDLC and architecture docs

- **Documentation portal** (runbooks, release audit, ADR index): `docs/README.md` — also links **Security**, **Code of Conduct**, and **Roadmap**
- Consolidated decisions: `docs/DECISIONS.md`
- Public release documentation audit: `docs/AUDIT_PUBLIC_RELEASE.md`
- SDLC checklist: `docs/SDLC_CHECKLIST.md`
- Architecture overview: `docs/architecture/system-overview.md`
- ADRs: `docs/adr/` (see `001-rust-first-polymarket-engine.md`)
- Operations runbook: `docs/RUNBOOK.md`
- Tiny live pilot guide: `docs/TINY_LIVE_PILOT.md`
- Contribution guide: `CONTRIBUTING.md`
- Strategy lab guide: `docs/STRATEGY_LAB.md`
- Prompt bundle guide: `docs/PROMPT_BUNDLE.md`
- Progress tracker: `docs/PROGRESS.md`
- Local instructions: `docs/INSTRUCTIONS.md`
- Local validation guide: `docs/LOCAL_VALIDATION.md`

## CI security gates

- Vulnerability scan: `cargo audit`
- SBOM generation: `./scripts/generate_sbom.sh artifacts` (CycloneDX when `cargo-cyclonedx` is installed)

## License

MIT — see [LICENSE](LICENSE).

## Security and community

- [SECURITY.md](SECURITY.md) — how to report vulnerabilities privately  
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) — contributor standards  

## Roadmap

See [ROADMAP.md](ROADMAP.md) for near-, mid-, and longer-term priorities (with links to product trackers).

## Notes

- This workspace is built for small-cap validation first (`$10-$50`) with strict safety limits.
- The initial implementation prioritizes plumbing, observability, and risk enforcement.
- See `docs/CONTEXT_PERSISTENCE.md` for resume instructions.
- See `docs/PINE_TUNING.md` for Pine parameter tuning workflow.
