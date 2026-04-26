# Frontend UX Master Plan

## Design direction

Build an institutional, personal trading terminal rather than a marketing dashboard. The interface should feel dense, composed, calm, and decisive. It should be readable during stress and rich enough for long operator sessions.

## Workspace map

### Command Center

- Scanner-driven market ranking.
- Selected market detail with microstructure vectors.
- Manual order ticket with maker or taker route selection.
- Order queue and strategy deployment map.

### Listing Radar

- Coinbase listing lifecycle board.
- Token profile with chain, float, unlock, liquidity, and venue coverage.
- Provider-backed research cards for Dune, DeFiLlama, wallet labels, and route depth.
- Entry templates for maker accumulation, taker breakout, and taker flatten.

### Risk Cockpit

- Real-time eligibility reasons.
- Capital budgets by strategy, venue, and route.
- Exposure and route concentration.
- Kill-switch state, manual halt, resume, and flatten controls.

### Strategy Lab

- Import history and promoted variants.
- Replay readiness view.
- Paper and replay evidence summaries.
- Strategy parameter provenance and Pine/webhook evidence.

### Agent Console

- Agent autonomy tier.
- Approval queue and pending decisions.
- Recommendation timeline with evidence links.
- Natural-language explanation of why a trade is allowed or rejected.

## UX standards

- Default to full-width bands and panels, not floating cards everywhere.
- Use strong typography and restrained accent color, not toy-like saturation.
- Keep tables and metric strips scan-friendly.
- Use clear labels for mode, route, risk, and approval state.
- Every automated action should expose its evidence and policy reason.

## Frontend implementation sequence

1. Upgrade the current dashboard into a workspace-based shell.
2. Add shared data hooks and polling boundaries.
3. Introduce typed API client helpers derived from OpenAPI.
4. Add component tests for workspace navigation and critical state rendering.
5. Add a Listing Radar workspace backed by fixture data first.
6. Add risk and agent workspaces backed by real API endpoints as they land.

## Future scale path

- Keep `crates/pt-dashboard/frontend` as the initial package.
- If the UI outgrows the package, split into a dedicated app workspace with shared UI/domain packages.
- Preserve one domain model per backend contract so replay, paper, and live views remain aligned.
