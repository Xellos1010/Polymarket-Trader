---
name: polymarket-sdlc-orchestrator
description: Use when the task involves non-trivial repository work for the Polymarket trading system and the agent should route the request through SDLC stages, bounded work orders, evidence gates, continuity updates, and release-safe integration discipline.
---

# Polymarket SDLC Orchestrator

## Overview

Use this skill for substantive repository work on the Polymarket trading system.

This skill turns a request into a disciplined SDLC pass that stays aligned with the repo's safety-first trading mission. It is for engineering control-plane work, not for autonomous live trading.

Use it when the agent needs to inspect the repo, classify the lifecycle stage, identify the smallest safe next slice, define acceptance criteria, require evidence, and leave resumable continuity.

Do not invoke the full lifecycle for trivial wording changes or clearly isolated low-risk metadata edits.

## When To Use

Use this skill when the request involves one or more of the following:

- repository audit, readiness, or gap assessment
- replay, paper-trading, or deployment-readiness work
- architecture, risk, observability, or validation changes
- work that should be broken into PR-sized tasks
- work that needs acceptance criteria, explicit validation commands, or evidence
- work that should update durable continuity for future runs
- work that may affect deployment, integration, or release posture

Do not use this skill as the primary workflow for:

- casual explanation-only questions
- tiny edits that do not materially change behavior
- requests that are fully answered by current grounded evidence without a lifecycle pass

## Operating Priorities

Use this priority order whenever tradeoffs appear:

1. Safety
2. Reproducibility
3. Observability
4. Risk controls
5. Correctness
6. Small bounded work
7. Sandbox ROI improvement
8. Release and deployment readiness
9. Live-pilot readiness only after earlier gates pass

Never optimize for speed, novelty, or automation breadth ahead of trading-system safety and repeatable evidence.

## Lifecycle Routing

Route each substantive request into one primary lifecycle stage before proposing work:

1. Intake
2. Discover
3. Define
4. Design
5. Plan
6. Implement
7. Verify
8. Integrate
9. Release
10. Operate
11. Diagnose
12. Improve

For each request, explicitly determine:

- current Polymarket phase: Phase 0 repo readiness, Phase 1 sandbox ROI, or Phase 2 tiny live pilot readiness
- primary lifecycle stage
- affected repo surfaces
- risk level: trivial, standard, substantial, or high-risk
- whether the next step should be audit-first, plan-first, implement-first, verify-first, or operate-first
- whether a human decision gate is required

## Affected Surface Model

Separate affected surfaces instead of treating the repo as one generic codebase.

Use these surfaces when relevant:

- strategy research and selection
- replay and backtest tooling
- paper-trading loop and soak workflows
- execution and exchange adapters
- hedge logic and delta management
- risk engine and kill switch controls
- PnL, attribution, and cost modeling
- dashboard, metrics, alerts, and operator visibility
- configuration, secrets handling, and environment setup
- CI, release, deployment, and runtime operations
- docs, runbooks, and continuity artifacts

## Required Workflow

For each substantive request:

1. Identify the current Polymarket phase and lifecycle stage.
2. Inspect grounded repo evidence before recommending changes.
3. State the active gate, blocker, or operating objective.
4. Recommend one bounded next step, preferably PR-sized.
5. Define acceptance criteria.
6. Provide exact validation commands when validation matters.
7. Record the main operational, trading, or release risks.
8. If implementation is needed, produce a coding task or bounded work-order style plan.
9. If the work is interrupted or blocked, leave resumable continuity.

When evidence is incomplete, say what is unknown and default to the safest next action.

## Bounded Work Rules

For substantial work:

- prefer one active integration lane per feature set
- keep work orders small and reviewable
- do not mix unrelated fixes into one slice
- do not recommend deploy or merge if local validation status is unknown
- do not claim completion without evidence

If the request is too large, chunk it into the smallest safe sequence of next steps.

## Verification And Evidence Rules

Do not treat code edits, proposals, or green-looking diffs as completion.

Require evidence proportionate to the change. When relevant, ask for or specify:

- formatting, compile, lint, and test gates
- replay or paper validation commands
- risk-limit and kill-switch verification
- observability and attribution checks
- blast-radius analysis for shared components
- rollback considerations for deployment-facing changes

Evidence should be concrete and traceable: command, scope, result, and any remaining uncertainty.

## Continuity And Memory

After meaningful work, update durable continuity so the next run can resume safely.

Persist or refresh:

- current phase and active gate
- active branch or PR lane when known
- open blockers and required human decisions
- accepted next step
- validation status and missing evidence
- durable operating conventions that should carry across runs

If nothing durable changed, state that no continuity update is needed rather than inventing one.

## Human Decision Escalation

Escalate instead of guessing when:

- approval is required for deployment, merging, live credentials, or increased risk
- evidence is too thin for a safe recommendation
- multiple conflicting next steps are equally plausible
- the change could widen blast radius across shared execution or risk systems

When escalating, provide:

- decision needed
- options
- risks
- your recommendation
- the safest resume point after a choice is made

## Safety Guardrails

Never use this skill to justify:

- autonomous live trading
- bypassing replay, paper, or risk gates
- deploying unverified changes
- increasing risk caps without explicit approval
- presenting speculative ROI as proven
- hiding missing evidence or unresolved blockers

Always keep the Polymarket trading repo in a safety-first, evidence-first posture.

## Default Output Structure

For substantial requests, return:

- Phase
- Lifecycle stage
- Current audit finding
- Recommended next action
- Acceptance criteria
- Validation commands
- Risks and guardrails
- Coding task or work-order prompt when implementation is needed
- Operator decision needed
