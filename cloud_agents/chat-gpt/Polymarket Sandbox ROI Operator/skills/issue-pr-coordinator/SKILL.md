---
name: issue-pr-coordinator
description: Use when the task involves creating, continuing, reviewing, or coordinating GitHub issues and pull requests across Xellos1010/Polymarket-Trader and Xellos1010/lambda-tradingviewer-ws, especially when progress must persist across runs or shared-file edits could create merge conflicts.
---

# Issue PR Coordinator

## Overview

Use this skill when the agent needs to turn repo work into issue-tracked execution, maintain continuity between runs, and coordinate pull requests safely across the two repositories.

Primary systems:

- {{label:GitHub,id:connector_76869538009648d5b282a4bb21c3d157,type:app}}
- {{label:Memory,id:file_persistence,type:file_persistence}}
- {{label:Adversarial Review and Strategic Architecture_ Personal Crypto Algorithmic Trading System.txt,id:6a07959bbecc81918f70c5f22c96454c,type:file}}

Treat these repositories as one coordinated engineering surface:

- Xellos1010/Polymarket-Trader
- Xellos1010/lambda-tradingviewer-ws

## When To Use This Skill

Use `$issue-pr-coordinator` when the request is about any of the following:

- creating the next issue from known bugs, roadmap items, or review findings
- deciding whether to continue the active issue or move to another one
- checking implementation progress since the last tracked event
- reviewing issue, branch, and PR status together
- deciding whether a task is ready for validation, review, or merge waiting
- detecting shared-file overlap across multiple issues or PRs
- aggregating dependent file edits into one isolated coordinating PR to reduce merge-conflict risk
- generating daily or weekly execution summaries grounded in issue and PR state

Do not use this skill for generic repo auditing when no issue, PR, or continuity decision is needed.

## Request Shapes

### 1. Start or resume work

Example prompts:

- "Start the next task from the adversarial review."
- "Continue the current Lambda websocket issue."
- "Should we stay on the active issue or switch to another one?"

Success criteria:

- identify the active or next issue
- decide whether to create a new issue or continue an existing one
- write or update a timestamped tracked event
- name one next safe action

### 2. Review issue and PR state

Example prompts:

- "Review open PRs and linked issues."
- "What changed since the last tracked event?"
- "Is this issue ready for validation or still implementation work?"

Success criteria:

- reconcile issue state, branch state, PR state, and continuity memory
- identify the current gate or blocker
- recommend one next step

### 3. Coordinate conflicting file edits

Example prompts:

- "Check whether these issues should share one PR."
- "We have two issues editing the same file; what should we do?"
- "Prevent merge conflicts across related PRs."

Success criteria:

- identify shared-file overlap clearly
- decide whether changes stay issue-scoped or move into a coordinating PR
- reference all related issues and PRs in the recommendation
- explain the merge-conflict rationale briefly and concretely

## Core Workflow

1. Identify the active request type: start, resume, review, or coordinate.
2. Determine the affected repo or repos.
3. Inspect the latest available state from GitHub, the adversarial review document when relevant, and continuity memory.
4. Reconcile these artifacts before making a recommendation:
   - issue state
   - branch state
   - PR state
   - last tracked timestamped event
   - file-overlap or dependency state
5. Name the current gate explicitly, such as intake, implementation, validation, review, blocked, or waiting on merge.
6. Recommend one best next action.
7. Update durable continuity so the next run can continue from the new state.

Do not recommend multiple equally weighted next steps unless the user explicitly asks for options.

## Issue Creation Rules

Before implementation begins:

- check whether a suitable issue already exists
- if not, create one before treating the task as active implementation work
- link the issue clearly to the relevant repo and task scope
- record a timestamped tracked event when work begins

Each tracked issue should maintain, at minimum:

- repo
- issue number
- issue title
- status
- created timestamp
- last touched timestamp
- active branch, if any
- linked PR, if any
- current blocker or gate
- next safe action

Do not begin implementation as an untracked stream of work unless the user explicitly asks for a one-off exception.

## Resume And Continuity Rules

When a run starts, first determine whether there is an already active issue lane.

Use this decision order:

1. If there is an active issue with incomplete implementation or validation work, continue it unless a blocker prevents safe progress.
2. If the active issue is waiting on review or approval, do not invent new implementation inside the same lane; report the waiting state and pick the next safe action only if the user asked for a lane switch.
3. If the active issue is blocked, name the blocker and the evidence needed to clear it.
4. Only move to another issue when the prior lane is complete, explicitly paused, blocked, or waiting in a way that makes another bounded task the safer choice.

Every resume decision should be grounded in both GitHub state and persisted tracked events, not memory alone.

## PR Coordination Rules

Default assumption:

- one issue maps to one bounded PR

Override that assumption when there is a real shared-file dependency.

If the same file is being changed across multiple issues or PRs:

- detect and name the overlapping file or files explicitly
- determine whether the overlap is sequential, concurrent, or logically coupled
- if the overlap is likely to create merge conflicts or broken sequencing, aggregate those file edits into one isolated coordinating PR
- ensure the coordinating PR references every related issue and PR touching that file set
- explain that the coordination exists to prevent competing edits and integration churn

Keep unrelated work out of the coordinating PR. Aggregate only the overlapping or dependency-bound surfaces.

If there is no meaningful overlap, keep PRs issue-scoped.

## Daily And Weekly Reporting Rules

### Daily report

Include:

- active issue and repo
- what changed since the last tracked event
- implementation, validation, or review status
- PR state
- blocker or dependency risk
- one next safe action

### Weekly report

Include:

- completed issues
- in-progress issues by repo
- blocked issues and reasons
- PR movement and approval state
- cross-repo dependencies
- shared-file or merge-conflict risks
- coordinating PR decisions
- recommended sequence for the next week

Keep reports concise and operator-friendly.

## Output Contract

Default output structure:

### Active lane

State the issue, repo, and current gate.

### Current state

Summarize the reconciled issue, branch, PR, and tracked-event status.

### Merge or dependency risk

State whether shared-file overlap exists and whether aggregation is needed.

### Recommended next action

Give one best next step.

### Continuity update

State what should be recorded in Memory for the next run.

If the task is specifically a reporting request, collapse the output into the requested daily or weekly report format.

## Memory Updates

Whenever this skill is used for a meaningful state transition, update the relevant continuity files in Memory, especially:

- `issue-ledger.yaml`
- `pr-ledger.yaml`
- `file-overlap-map.yaml`
- `agent-continuity.md`

Timestamp every meaningful event update.

Memory should summarize durable execution state, but GitHub remains the source of truth for live issue and PR reality when the two differ.

## Example

### Example input

"Continue the current Lambda issue and tell me whether we should keep the existing PR or create an isolated coordinating PR for the shared websocket handler file."

### Example output

- Active lane: `lambda-tradingviewer-ws`, Issue #42, implementation gate
- Current state: existing branch is active, linked PR is open, last tracked event shows websocket handler edits in progress
- Merge or dependency risk: the same handler file is also being changed by Issue #39 in another open PR, so independent PRs are likely to conflict
- Recommended next action: keep Issue #42 active but move the overlapping handler changes into one isolated coordinating PR that references Issues #39 and #42 plus both related PRs
- Continuity update: record the new overlap decision, coordinating PR requirement, and next validation step in Memory
