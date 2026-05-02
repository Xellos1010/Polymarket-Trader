# Agent Instructions

Repository: /workspace/Polymarket-Trader

This is a Rust-first trading workspace. Prefer small, PR-sized changes. Do not enable live mode, do not inject live credentials, and do not raise risk caps for validation convenience.

Before editing:
- Read README.md.
- Read docs/LOCAL_VALIDATION.md.
- Check whether the task touches crates/pt-dashboard/frontend.

Baseline commands:
- cargo fmt --all
- cargo check --workspace
- cargo clippy --workspace --all-targets --all-features -- -D warnings
- cargo test --workspace
- cargo build --workspace
- cargo audit
- ./scripts/generate_sbom.sh artifacts

Canonical validation:
- ./scripts/local_validation_ladder.sh

If frontend files under crates/pt-dashboard/frontend changed:
- RUN_FRONTEND=1 ./scripts/local_validation_ladder.sh

Do not claim full validation passed unless the full relevant ladder completed. Partial runs must be reported as partial.
