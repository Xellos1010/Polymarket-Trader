# Implementation Work Orders

## WO-001: Command-center shell

- Goal: Turn the current dashboard into a multi-workspace trading terminal shell without requiring new backend endpoints.
- Scope:
  - workspace navigation
  - mission band and operator summary
  - listing, risk, strategy, and agent workspace scaffolds
  - responsive layout polish
- Validation:
  - frontend tests for workspace rendering
  - manual dashboard smoke run in Codespaces

## WO-002: Listing-radar backend model

- Goal: Add typed listing lifecycle and token profile models to the dashboard API.
- Scope:
  - listing lifecycle state machine
  - asset metadata summary
  - provider provenance
  - sample fixtures for replay and frontend development
- Validation:
  - Rust unit tests
  - OpenAPI schema updates
  - frontend fixture rendering

## WO-003: Execution policy visibility

- Goal: Surface maker/taker policy decisions and route eligibility in the dashboard.
- Scope:
  - execution policy event store
  - `/state/execution-policy` endpoint
  - table and drilldown UI
- Validation:
  - replay evidence
  - dashboard contract tests

## WO-004: Agent governance

- Goal: Add autonomy tiers, approval queue, and decision log surfaces.
- Scope:
  - bounded autonomy state machine
  - approval-required actions
  - operator evidence links
- Validation:
  - unit tests for policy transitions
  - replay scenarios covering approve and reject flows

## WO-005: Provider adapter foundation

- Goal: Add interface crates and fixture-driven tests for research providers.
- Scope:
  - Dune adapter contract
  - DeFiLlama fundamentals adapter
  - 0x and Jupiter quote adapter interfaces
- Validation:
  - adapter tests against canned responses
  - no live trading enablement
