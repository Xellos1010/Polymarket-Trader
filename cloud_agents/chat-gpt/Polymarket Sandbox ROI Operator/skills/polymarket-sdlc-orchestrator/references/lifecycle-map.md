# Polymarket SDLC Lifecycle Map

Use this reference when a request needs explicit lifecycle routing.

## Phase alignment

- Phase 0: repo readiness, buildability, validation, observability, safety scaffolding
- Phase 1: replay and paper evidence, cost-adjusted ROI assessment, repeatability, risk discipline
- Phase 2: tiny live pilot readiness after prior gates pass and explicit human approval exists

## Lifecycle stage hints

- Intake: normalize goal, repo, constraints, approvals
- Discover: inspect repo, docs, CI, branch/PR state, metrics, known failures
- Define: success criteria, non-goals, unknowns, risk posture
- Design: contracts, boundaries, observability, interfaces, rollout shape
- Plan: bounded work orders, acceptance criteria, validation commands
- Implement: one scoped change set
- Verify: evidence, tests, replay/paper checks, blast-radius review
- Integrate: consolidate into the active PR lane
- Release: handoff, rollback, readiness summary
- Operate: scheduled audits, cleanup, roadmap refresh, no-op rationale when appropriate
- Diagnose: explain failure, drift, or gate miss
- Improve: propose the next smallest safe gain
