# Agent Instructions

Repository: /workspace/Polymarket-Trader

This is a Rust-first trading workspace. Prefer small, PR-sized changes. Do not enable live mode, do not inject live credentials, and do not raise risk caps for validation convenience.

Before editing:
- Read README.md.
- Read ROADMAP.md (product and technical direction; phased plan).
- Read docs/LOCAL_VALIDATION.md.
- Check whether the task touches crates/pt-dashboard/frontend.

Nx (task orchestration; from repo root after `pnpm install`):

- `pnpm exec nx show projects`
- `pnpm exec nx run-many -t fmt check clippy test build --projects=polymarket-trader`
- `pnpm exec nx run polymarket-trader:local-validation` (full ladder; set `RUN_FRONTEND=1` for dashboard)
- Dashboard only: `pnpm exec nx run pt-dashboard-frontend:test` (run `npm install` in `crates/pt-dashboard/frontend` first)

Skills live under `.cursor/skills/` and `.claude/skills/` (continuity, SDLC routing, nx-verification, ADR/handoff helpers, etc.). A few skills (`define-intent`, `scope-chunker`, `model-router`) reference optional `flagship-foundry-work/` SSOT files that are not vendored here; use them for process guidance only, or copy those assets from your Flagship Foundry workspace if you need schema-backed outputs.

Baseline commands (equivalent to Nx targets on `polymarket-trader`):

- cargo fmt --all
- cargo check --workspace
- cargo clippy --workspace --all-targets --all-features -- -D warnings
- cargo test --workspace
- cargo build --workspace
- cargo audit
- ./scripts/generate_sbom.sh artifacts

Canonical validation:

- ./scripts/local_validation_ladder.sh
- or `pnpm exec nx run polymarket-trader:local-validation`

If frontend files under crates/pt-dashboard/frontend changed:
- RUN_FRONTEND=1 ./scripts/local_validation_ladder.sh

Do not claim full validation passed unless the full relevant ladder completed. Partial runs must be reported as partial.
