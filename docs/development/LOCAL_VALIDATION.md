# Local Validation

This document is the canonical local-first validation ladder for Phase 0 and Phase 1 work.

Use it before merge, deployment, or any claim that replay or paper results are ready for operator review.

## Purpose

The repository uses a local-first rule:
- GitHub Actions is a useful subset.
- Local validation is the full pre-merge gate.
- Replay, paper, and evidence checks are not considered complete just because CI is green.

## Prerequisites

Install or confirm these tools before running the ladder:

- Rust stable with `cargo`, `rustfmt`, and `clippy`
- `python3`
- `curl`
- `jq` for richer paper-soak signal collection
- `git`
- `cargo-audit`
- optional but recommended: `cargo-cyclonedx`
- optional for frontend changes: `node`, `pnpm`

Example setup:

```bash
rustup toolchain install stable
rustup component add rustfmt clippy
cargo install cargo-audit --locked
cargo install cargo-cyclonedx --locked
```

## Config bootstrap

Copy baseline configs once per fresh checkout:

```bash
cp config/config.example.toml config/config.toml
cp config/coinbase_strategy_lab.example.json config/coinbase_strategy_lab.json
cp config/prompt_bundle.example.json config/prompt_bundle.json
```

Do not put live credentials in these files for local validation.
Use environment variables or a local `.env` file for secrets instead.

## Canonical ladder

Run in this order:

```bash
cargo fmt --all
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo build --workspace
cargo audit
./scripts/generate_sbom.sh artifacts
python3 tools/coinbase_strategy_lab.py backtest --config config/coinbase_strategy_lab.json
python3 tools/coinbase_strategy_lab.py overlap --config config/coinbase_strategy_lab.json --auto-discovery
python3 tools/coinbase_strategy_lab.py optimize --config config/coinbase_strategy_lab.json
cargo run -p pt-cli -- run --config config/config.toml
./scripts/paper_soak.sh 86400 30 config/config.toml
```

## Frontend checks

When a change touches `crates/pt-dashboard/frontend`, also run:

```bash
pnpm exec nx run pt-dashboard-frontend:test
pnpm exec nx run pt-dashboard-frontend:build
```

## What CI covers

GitHub Actions currently covers:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo build --workspace
cargo audit
./scripts/generate_sbom.sh artifacts
```

GitHub Actions does not replace the local replay, strategy-lab, runtime, paper-soak, or optional frontend checks.

## Expected artifacts

Useful outputs to keep when a run fails or when Phase 1 evidence is being reviewed:

- `artifacts/` SBOM output
- `data/strategy_lab/` reports and exports
- `data/replay/` replay artifacts
- `data/soak/` paper-soak JSON and logs
- `docs/SESSION_CONTEXT.md` checkpoints when preserving context

## Common failure points

- `cargo audit` missing locally:
  install `cargo-audit` first.
- SBOM falls back to cargo metadata:
  install `cargo-cyclonedx` if you want CycloneDX output.
- `paper_soak.sh` probe quality is reduced without `jq`.
- Frontend checks fail if dependencies have not been installed in `crates/pt-dashboard/frontend`.
- Replay or paper runs are not valid evidence unless the config remains sandbox-only.

## One-command runner

Use the wrapper script when you want stage-labeled, fail-fast execution:

```bash
./scripts/local_validation_ladder.sh
```

Optional frontend stage:

```bash
RUN_FRONTEND=1 ./scripts/local_validation_ladder.sh
```

## Safety

- Do not enable live mode.
- Do not inject live credentials.
- Do not raise risk caps for validation convenience.
- Do not treat partial runs as full gate passes.
