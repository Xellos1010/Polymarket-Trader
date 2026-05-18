# Cadence Guardrails

Use this reference when deciding what a recurring run may do.

## Good recurring outcomes

- tighten a work order
- verify a previously changed area
- refresh evidence or missing commands
- summarize branch or PR drift
- propose the next smallest safe task
- improve docs or continuity around the current gate

## Bad recurring outcomes

- invent a new large scope without authorization
- skip missing tests because the repo looks stable
- recommend live deployment from sparse evidence
- treat one profitable replay as sufficient live-readiness proof
- create competing lanes for the same objective

## Safe default

If in doubt, choose audit, verification, or continuity improvement over new implementation.
