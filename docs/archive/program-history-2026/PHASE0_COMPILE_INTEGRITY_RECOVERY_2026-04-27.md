## Phase

Phase 0: repo readiness

## Current audit finding

Re-auditing all previously completed work, the open PR queue, issue `#9`, and the latest CI state shows that the repository is currently blocked in Phase 0 before the next Phase 1 queue-runtime slice can safely continue.

Grounded findings as of 2026-04-27:
- PR `#40` is valid and still necessary, but it only removes the duplicate `chrono.workspace = true` entry in `crates/pt-cli/Cargo.toml`.
- Recent CI runs tied to the active queue work still fail at parser/format stages after that manifest fix path, including runs `24986437008`, `24989037786`, and `25000254220`.
- The next safe engineering move is not more queue-runtime wiring or more planning-only PRs.

## Broken files currently blocking compile integrity
- `crates/pt-cli/src/main.rs`
- `crates/pt-coinbase/src/lib.rs`
- `crates/pt-core/src/config.rs`
- `crates/pt-dashboard/src/lib.rs`
- `crates/pt-dashboard/tests/api_contract.rs`
- `crates/pt-quote/src/lib.rs`
- `crates/pt-risk/src/lib.rs`
