# Phase 0 Compile Recovery Queue (2026-04-27)

## Phase
Phase 0: repo readiness

## Why this queue exists
Re-auditing the open PR stack, issue `#9`, recent CI runs, and the current `main` branch shows the repo is blocked before the next Phase 1 approval-queue runtime slice can safely continue.

Grounded findings:
- `crates/pt-cli/Cargo.toml` still contains a duplicate `chrono.workspace = true` entry.
- CI currently fails before meaningful Phase 1 validation because repo integrity is broken earlier in the ladder.
- Several Rust files on `main` contain duplicate imports, duplicate type blocks, or partially merged sections that prevent a reliable fmt/check/test run.
- A blanket restore from baseline commit `7da0bd8ba608f0f57e2edc83b7bf1f73cff955b1` would remove substantial newer intended work in multiple files.

This queue is therefore a sequencing document, not a claim that Phase 0 is already recovered.

## Safest next action
Merge the smallest blocker first, then recover compile integrity in narrow slices.

### Slice 0
PR `#40` `[codex] Remove duplicate chrono entry from pt-cli manifest`

Goal:
- restore manifest integrity so cargo metadata can run again

Acceptance criteria:
- `cargo metadata --format-version 1` succeeds

### Slice 1
Surgical syntax recovery for `crates/pt-cli/src/main.rs` and `crates/pt-coinbase/src/lib.rs`

Why grouped:
- both files are runtime-entry surfaces with duplicate imports and malformed merged sections
- both require preserving newer command/auth/websocket logic while removing duplicate blocks

Guardrails:
- no live-mode enablement
- no credential changes
- preserve newer command and auth/profile surfaces already intended on `main`
- use baseline commit `7da0bd8ba608f0f57e2edc83b7bf1f73cff955b1` only as fragment fallback, not blanket rollback

Acceptance criteria:
- one coherent import/header region per file
- no malformed enum or function boundaries
- file parses under `cargo fmt --all`

### Slice 2
Surgical syntax recovery for `crates/pt-core/src/config.rs`

Why isolated:
- this file has the highest merge-surface risk because it mixes config structs, defaults, validation, and live-auth rules
- a careless rollback would silently remove newer config surfaces

Guardrails:
- keep newer config families where they are clearly intentional
- remove duplicate struct fields, duplicate type definitions, and duplicate default blocks only
- do not relax live-auth or risk-cap validation

Acceptance criteria:
- exactly one definition per config type
- exactly one field per config struct member
- exactly one default/validation path per setting

### Slice 3
Surgical syntax recovery for `crates/pt-dashboard/src/lib.rs` and `crates/pt-dashboard/tests/api_contract.rs`

Why grouped:
- these files are tightly coupled
- the dashboard file currently mixes duplicate handlers/imports/frontend fallback sections
- the contract test file currently mixes duplicated imports, fixtures, and endpoint arrays

Guardrails:
- preserve current intended read-only dashboard surfaces
- do not widen authority on `/api/v1/orders` or `/api/v1/approval-queue`
- keep compile recovery separate from feature expansion

Acceptance criteria:
- one coherent router/handler set
- one coherent fixture/test harness
- OpenAPI contract tests are structurally runnable again

### Slice 4
Re-run the local-first validation ladder and record the result before resuming issue `#9`

Validation commands:
```bash
cargo fmt --all
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo build --workspace
cargo audit
./scripts/generate_sbom.sh artifacts
```

Exit condition:
- local and CI quality gates pass, or the remaining failures are reduced to a smaller, clearly isolated follow-up scope

## Files currently known to need recovery attention
- `crates/pt-cli/Cargo.toml`
- `crates/pt-cli/src/main.rs`
- `crates/pt-coinbase/src/lib.rs`
- `crates/pt-core/src/config.rs`
- `crates/pt-dashboard/src/lib.rs`
- `crates/pt-dashboard/tests/api_contract.rs`
- `crates/pt-quote/src/lib.rs`
- `crates/pt-risk/src/lib.rs`

Note:
`pt-quote` and `pt-risk` also show merge corruption, but they should be handled opportunistically inside the validation loop once the higher-risk runtime/config/dashboard files are stabilized. They are not good blanket-restore candidates without local verification.

## Phase 1 sequencing rule
Do not resume issue `#9` queue-runtime wiring until the Slice 4 validation ladder is green enough to trust local-first results again.

Issue `#9` remains the next Phase 1 code-bearing objective after Phase 0 is recovered:
- hydrate queue-relevant rows on startup
- reconcile after local lifecycle mutations
- reconcile after live-order sync / identity changes
- keep dashboard queue surfaces read-only

## Operator guardrails
- no live-mode enablement
- no credential changes
- no risk-cap increases
- no deployment changes
- do not treat docs-only progress as sandbox ROI progress
