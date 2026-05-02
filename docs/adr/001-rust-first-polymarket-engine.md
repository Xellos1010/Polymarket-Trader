# ADR-001: Rust-First Single-Host Phase-1 Trading Engine

## Status
Accepted

## Context
The system needs low-latency market-making and hedge execution with strict risk controls under a low-cost single-host deployment model. Existing prior work spans Node/Python repos, but phase-1 target runtime is Rust.

## Decision
- Build a Rust workspace with focused crates (`pt-core`, `pt-engine`, `pt-polymarket`, `pt-coinbase`, etc.).
- Use async Tokio orchestration in one process on one EC2 host.
- Maintain hard risk gates and kill-switch at runtime.
- Expose an operator dashboard HTTP surface for health/risk/state/ops controls.
- Add replay/paper mode before tiny live mode.

## Consequences
### Positive
- Better runtime safety and lower overhead for long-lived trading loops.
- Clear modularity via crate boundaries.
- Stronger operational controls through explicit risk and dashboard surfaces.

### Negative
- More up-front engineering effort than a single-script bot.
- Requires Rust tooling familiarity for contributors.

## Alternatives Considered
- **Node-only runtime**: faster initial iteration, weaker performance predictability.
- **Python-only runtime**: fastest prototyping, weakest latency envelope and type safety.

## Implementation
- Phase-1 workspace implemented with live/paper/replay execution support.
- Added preflight command for live readiness checks.
- Added API contracts + schema documentation and SDLC checklist tracking.
