---
name: polymarket-cadence-runner
description: Use when the Polymarket trading agent is operating on an hourly, daily, or recurring cadence and must safely choose improvement, verification, reconciliation, or deployment-readiness work without drifting into unsafe autonomous behavior.
---

# Polymarket Cadence Runner

## Overview

Use this skill for recurring improvement loops on the Polymarket trading system.

This skill governs what the agent should do on scheduled or repeated runs so it keeps the system improving over time while staying conservative about deployment, live trading, and destructive cleanup.

The goal is not constant autonomous coding for its own sake. The goal is safe, resumable progress on verification, readiness, small fixes, roadmap advancement, and deployment preparation.

## When To Use

Use this skill when a run is triggered by an hourly, daily, or other recurring cadence, or when the user asks the agent to continuously improve the repo over time.

Typical use cases:

- reconcile active work and unfinished validation
- refresh the safest next task from current repo state
- continue the next approved bounded slice
- run audit-first loops that improve readiness and evidence quality
- prepare deployment or release tasks only after earlier gates pass

## Scheduled Priority Order

On a recurring run, choose work in this order:

1. unfinished active work in the current integration lane
2. failed or missing validation for already-started work
3. blocked items that can now be unblocked from new evidence
4. the next approved, bounded work order
5. roadmap refinement if no safe scoped work exists
6. no-op or digest output if nothing safe is authorized

Do not start broad new implementation just because the schedule fired.

## Allowed Scheduled Work

Recurring runs may:

- inspect repo and branch state
- audit missing validation evidence
- tighten acceptance criteria and work-order quality
- continue previously approved bounded work
- propose small next tasks
- refresh continuity and memory
- prepare deployment-readiness checklists
- summarize blockers and decisions needed

Recurring runs should prefer reconciliation, verification, and bounded advancement over open-ended expansion.

## Disallowed Scheduled Behavior

Do not let cadence alone authorize:

- live trading activation
- use of live credentials
- deployment without verified gates and approval
- merging PRs without approval
- creation of multiple competing integration lanes for the same objective
- destructive branch cleanup when authority is ambiguous
- raising risk limits or relaxing safeguards

## Cadence Execution Loop

Follow this loop:

1. Read current continuity and the active repo objective.
2. Inspect active branch or PR state when available.
3. Identify the current Polymarket phase and gate.
4. Classify candidate work into verify, reconcile, implement, operate, or improve.
5. Pick the smallest safe task.
6. State the exact validation or evidence expected.
7. Execute or recommend that bounded task.
8. Update continuity with outcome, blockers, and next action.

If there is no safe authorized task, produce a concise no-op digest that says why.

## Deployment-Readiness Rule

Treat deployment automation as a gated outcome, not the default scheduled action.

Before recommending deploy-oriented work, confirm that:

- local and CI-quality gates are known
- relevant replay or paper checks have been considered when trading behavior is affected
- rollback thinking exists
- the requested environment and approval boundary are explicit

If those conditions are missing, the scheduled run should improve readiness rather than push deployment.

## Improvement Doctrine

"Working more every day and hour" means improving the system's verified readiness and execution quality over time, not blindly producing more code.

Prefer improvements that:

- reduce risk or ambiguity
- strengthen observability and attribution
- tighten replay and paper validation
- simplify the next human review
- leave better continuity for the next run

## Output Contract

For recurring runs, return:

- Current phase and gate
- Run classification
- Safest task selected
- Why it outranked other options
- Validation or evidence expected
- Blockers or approvals needed
- Next recurring step
