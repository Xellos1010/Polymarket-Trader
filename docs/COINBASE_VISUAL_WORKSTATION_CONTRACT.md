# Coinbase Visual Workstation Contract

## Phase

Phase 0 exit / Phase 1 preparation.

This contract defines the first bounded Coinbase visual workstation slice for the dashboard frontend. It is sandbox and paper only.

## Purpose

Align the existing multi-workspace dashboard shell with one chart-first product review surface that stays grounded in the current backend API.

## Source of truth

Primary API contract:

- `GET /api/v1/scanner`
- `GET /api/v1/products/{product_id}`
- `GET /api/v1/orders`
- `GET /api/v1/strategies`
- `POST /api/v1/strategy-lab/import`
- `POST /api/v1/mode`
- `POST /api/v1/live/arm`
- `POST /api/v1/live/disarm`

Related documented support endpoints:

- `GET /state/history`
- `GET /state/risk`

Additional current frontend surfaces that are already used in the shell but are not yet represented in `docs/api/dashboard-openapi.yaml` should be treated as provisional until the API documentation catches up:

- `GET /api/v1/listings`
- `GET /api/v1/listings/{product_id}`
- `GET /api/v1/risk/overview`
- `GET /api/v1/agent/console`

## First-slice workstation panels

### 1. Scanner lane

Status: current-API-backed.

Inputs:

- `GET /api/v1/scanner`

Responsibilities:

- rank products
- show spread, imbalance, fill rate, and active strategy
- drive selected product context

### 2. Selected product visual workstation

Status: mixed.

Current-API-backed inputs:

- `GET /api/v1/products/{product_id}`

Fixture-backed or derived inputs for the first slice:

- intraday chart bars derived from current product detail and scanner state
- derived spread and volume overlays
- derived strategy markers

Responsibilities:

- show a chart-first product review surface
- keep one price pane and one secondary activity pane
- expose whether the chart is current-API-backed or fixture-backed

### 3. Strategy review rail

Status: current-API-backed with imported lineage from existing strategy responses.

Inputs:

- `GET /api/v1/products/{product_id}`
- `GET /api/v1/strategies`

Responsibilities:

- show active strategy name
- show score components and threshold posture
- show imported strategy-lab lineage when present
- show eligibility and paper guardrails in one compact rail

### 4. Orders table

Status: current-API-backed.

Inputs:

- `GET /api/v1/orders`

Responsibilities:

- show queued and recent workstation orders
- keep order review visible without widening execution authority

## Explicit first-slice non-goals

- no new live trading authority
- no autonomous execution expansion
- no Pine compatibility layer
- no hidden chart state that outruns backend truth
- no implication that fixture-backed bars are replay evidence

## Presentation rules

- every workstation panel must identify whether it is current-API-backed or fixture-backed when that distinction matters
- chart review and strategy review should sit together in the command workspace
- policy holds and scan-only reasons must remain visible and not be hidden behind decorative UI
- imports and strategy lineage must remain audit-friendly

## Safe expansion path

1. start with derived chart bars in the frontend to establish layout, pacing, and benchmark hooks
2. wire `/state/history` or a product-history adapter when the backend contract is ready
3. add explicit strategy markers sourced from promoted artifacts after issue `#53` strengthens the runtime handoff
4. keep replay and paper attribution in the Rust runtime rather than the chart layer
