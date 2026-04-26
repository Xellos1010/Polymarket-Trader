# Session Context

Generated at UNIX epoch seconds: `1777223346`

## Note
Control-tower audit and queue start for the next Phase 1 round.

What is grounded from the current repo state:
- `main` is still in **Phase 1: sandbox trading / paper ROI**.
- Real active implementation tracks are draft PRs `#11`, `#12`, `#13`, and `#18`.
- Draft PRs `#14` through `#17` overlap as planning/checkpoint work and should not become the long-term source of truth.
- Older open PRs `#4` and `#8` remain stale/superseded until explicitly rebased and revalidated.
- The next concrete runtime blocker is issue `#9`: approval-queue/workstation-order state is still in-memory inside the Coinbase workstation runtime.

Next round that is now queued and started:
- persist only operator-review queue states (`draft`, `cancel_requested`) via `storage.sqlite_path`
- hydrate those rows on startup so restart/reload behavior is deterministic
- keep `/api/v1/orders` and `/api/v1/approval-queue` read-only
- add focused tests for create/update/reload behavior
- do not widen live authority, credentials, or risk caps

## Last Known Local Runtime
This audit cycle was remote-first through GitHub inspection and did **not** rerun cargo locally.

Prior saved local toolchain snapshot:
- `rustc`: `rustc 1.93.1 (01f6ddf75 2026-02-11)`
- `cargo`: `cargo 1.93.1 (083ac5135 2025-12-15)`

## Core Commands
- Run engine: `cargo run -p pt-cli -- run --config config/config.toml`
- Live preflight: `cargo run -p pt-cli -- preflight-live --config config/config.toml --timeout-ms 3000`
- Dashboard: `http://127.0.0.1:8080/`
- Health: `cargo run -p pt-cli -- status --url http://127.0.0.1:8080/health`

### Strategy Lab
- Generate dashboard: `python3 tools/coinbase_strategy_lab.py dashboard --config config/coinbase_strategy_lab.json --serve 9090`
- Overlap with auto-discovery: `python3 tools/coinbase_strategy_lab.py overlap --config config/coinbase_strategy_lab.json --auto-discovery`
- Backtest without journal writes: `python3 tools/coinbase_strategy_lab.py backtest --config config/coinbase_strategy_lab.json --disable-journal`

### Promotion / Replay
- Promote lab result: `./scripts/promote_strategy_lab.sh data/strategy_lab/<report>.json BTC-USD sma_baseline`
- Apply replay config:
  - `engine.mode = "replay"`
  - `engine.replay_path = "data/replay/strategy_lab_promoted.ndjson"`

### Immediate Validation Ladder For The Next Code Slice
- `cargo fmt --all`
- `cargo check --workspace`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test -p pt-cli`
- `cargo test -p pt-dashboard`
- `cargo test --workspace`
- `cargo build --workspace`
- `cargo audit`
- `./scripts/generate_sbom.sh artifacts`

## Live Prerequisites
- Set `engine.mode = "live"` in config.
- Set `venues.polymarket.private_key` or `POLYMARKET_PRIVATE_KEY`.
- Set `venues.coinbase.api_key`/`api_secret` or `COINBASE_API_KEY`/`COINBASE_API_SECRET`.
- Keep hard risk caps enabled for tiny-live rollout.
- Require explicit human approval before any live credential use, live mode, deployment, or order execution.
