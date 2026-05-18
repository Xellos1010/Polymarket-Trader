# Roadmap Issue Backlog

Generated on May 17, 2026 from:
- `ROADMAP.md`
- `docs/research/Adversarial Review and Strategic Architecture_ Personal Crypto Algorithmic Trading System.txt`
- current GitHub issue state

## Queue

| Queue | Type | Status mapping | Title / scope |
|---|---|---|---|
| 1 | existing child | reuse existing issue | `#65` Confirm and fix hardcoded deployed capital in engine startup |
| 2 | new child | create new child issue | `#74` Fix MACD `None -> 0.0` corruption in `pt-strategy-lab` indicators |
| 3 | new child | create new child issue | `#75` Fix shell portability in `pt-cli tune-pine` evaluation path |
| 4 | new child | create new child issue | `#76` Fix two-sided exposure reservation under-counting in `pt-risk` |
| 5 | new child | create new child issue | `#77` Remove deprecated Coinbase Advanced Trade passphrase header path |
| 6 | new epic | create new epic | `#66` Critical correctness and safety bug closure |
| 7 | new child | create new child issue | `#78` Pine parity: HTF confirmation, repaint, and entry semantics hardening |
| 8 | new child | create new child issue | `#79` Pine parity: frozen ATR/zone state and timeout semantics |
| 9 | new child | create new child issue | `#80` Pine parity: session, S/R break, and candlestick definition normalization |
| 10 | new child | create new child issue | `#81` Pine parity: bounded tuning groups, OOS hooks, and experimental pattern gates |
| 11 | new child | create new child issue | `#82` Pine parity: weight normalization and realistic fee/sizing assumptions |
| 12 | new epic | create new epic | `#67` Pine parity cleanup and signal-definition hardening |
| 13 | new child | create new child issue | `#83` Pi operator path: Cloudflare Tunnel and documented fallback topology |
| 14 | new child | create new child issue | `#84` TradingView allowlist and request hardening |
| 15 | new child | create new child issue | `#85` HMAC replay protection with nonce persistence for webhook ingestion |
| 16 | new child | create new child issue | `#86` Webhook failure fallback drill and resilience soak |
| 17 | new epic | create new epic | `#68` Webhook and Pi-hosted operator path hardening |
| 18 | new child | create new child issue | `#87` Scaffold `pt-ai-agent` crate and bounded proposal interfaces |
| 19 | new child | create new child issue | `#88` Add local model client boundary and runtime configuration policy |
| 20 | new child | create new child issue | `#89` Add OpenRouter client boundary with spend caps and routing policy |
| 21 | new child | create new child issue | `#90` Approval queue API and persistence hardening for agent proposals |
| 22 | new child | create new child issue | `#91` Agent Console dashboard surface for proposal review |
| 23 | new child | create new child issue | `#92` Model routing and AI cost telemetry dashboard |
| 24 | new epic | create new epic | `#69` AI agent foundation and approval-control plane |
| 25 | new child | create new child issue | `#93` AI management: signal staleness and regime-alignment checks |
| 26 | new child | create new child issue | `#94` AI management: position and anomaly monitoring summaries |
| 27 | new child | create new child issue | `#95` AI management: morning brief and end-of-day report generation |
| 28 | new child | create new child issue | `#96` AI management: mode transition proposal logic |
| 29 | new epic | create new epic | `#70` AI management layer |
| 30 | existing child | already complete / no new issue | `#59` Bounded AI optimizer objective and candidate sweep lane |
| 31 | existing child | already complete / no new issue | `#60` Strategy AI review surfaces |
| 32 | new child | create new child issue | `#97` AI improvement: attribution-driven parameter adjustment lane |
| 33 | new child | create new child issue | `#98` AI improvement: threshold calibration and ROC-style evaluation |
| 34 | new child | create new child issue | `#99` AI improvement: failure-mode clustering and error taxonomy |
| 35 | new child | create new child issue | `#100` AI improvement: walk-forward validation expansion |
| 36 | new child | create new child issue | `#101` AI improvement: optimizer governance and evaluation-budget enforcement |
| 37 | new epic | create new epic | `#71` Bounded AI improvement and validation |
| 38 | new child | create new child issue | `#102` AI discovery: compositional indicator synthesis framework |
| 39 | new child | create new child issue | `#103` AI discovery: supervised pattern-discovery framework |
| 40 | new child | create new child issue | `#104` AI discovery: sentiment integration framework |
| 41 | new child | create new child issue | `#105` AI discovery: regime-classification framework |
| 42 | new child | create new child issue | `#106` AI discovery: code generation with compile/backtest validation gates |
| 43 | new epic | create new epic | `#72` AI discovery layer |
| 44 | new child | create new child issue | `#107` Portfolio: capital allocation optimizer |
| 45 | new child | create new child issue | `#108` Portfolio: strategy correlation analysis |
| 46 | new child | create new child issue | `#109` Portfolio: rebalance policy and execution flow |
| 47 | new child | create new child issue | `#110` Portfolio: multi-strategy dashboard views |
| 48 | new child | create new child issue | `#111` Portfolio: strategy collision handling for parallel execution |
| 49 | new epic | create new epic | `#73` Multi-strategy portfolio management |

## Completed foundations reused

- `#53` strategy-lab handoff into Coinbase paper runtime
- `#58` Rust-native strategy IR and adapter layer
- `#59` bounded optimizer objective and candidate sweep lane
- `#60` strategy candidate review surfaces
- `#61` benchmark harnesses
- `#10` repeatable Phase 1 gate report

These are not recreated. They are referenced as prerequisites where they materially overlap the memo.

## Coverage Matrix

| Memo finding / phase | Roadmap phase | Issue mapping | Notes |
|---|---|---|---|
| deployed capital hardcode | post-Phase 1 correctness | `#65` reused | only open issue already matching memo |
| MACD `None -> 0.0` corruption | post-Phase 1 correctness | `#74` | strategy-lab correctness bug |
| zsh / shell portability | post-Phase 1 correctness | `#75` | scoped to `tune-pine` path |
| two-sided exposure under-counting | post-Phase 1 correctness | `#76` | risk engine correctness |
| deprecated Coinbase passphrase header | post-Phase 1 correctness | `#77` | Coinbase API hygiene and auth correctness |
| Pine repaint / HTF confirmation | parity / spec work | `#78` | first parity slice |
| Pine ATR freeze / timeout / S-R semantics | parity / spec work | `#79`-`#80` | grouped by behavior, not one bug per sentence |
| Pine tuning bounds / OOS validation | parity / spec work | `#81` | bounded optimization guardrail for Pine research |
| Pine fee realism / sizing realism | parity / spec work | `#82` | research-only assumptions, not runtime truth |
| Cloudflare Tunnel / Pi operator path | infrastructure hardening | `#83` | aligned to memo Phase C |
| TradingView allowlist / request hardening | infrastructure hardening | `#84` | additive to existing webhook path |
| HMAC replay protection / nonce store | infrastructure hardening | `#85` | explicit replay defense |
| fallback drill / resilience soak | infrastructure hardening | `#86` | failure path and soak evidence |
| AI agent crate skeleton | AI foundation | `#87` | aligned to Rust-first repo direction |
| local LLM client | AI foundation | `#88` | boundary first, not model tuning |
| OpenRouter integration / spend caps | AI foundation | `#89` | bounded, policy-driven |
| approval queue / API | AI foundation | `#90` | builds on existing queue concepts without duplicating `#9` |
| Agent Console UI | AI foundation | `#91` | dashboard surface |
| routing / cost telemetry | AI foundation | `#92` | observability before deeper AI automation |
| signal validation / anomaly monitoring | AI management | `#93`-`#94` | advisory-only |
| daily brief / EOD report | AI management | `#95` | grouped reporting slice |
| mode management proposals | AI management | `#96` | human-gated |
| bounded optimization foundation | completed Phase 1 | `#59`, `#60` reused | no duplicate issues |
| attribution / ROC / clustering / walk-forward | AI improvement | `#97`-`#101` | next layer after optimizer foundation |
| indicator synthesis / pattern discovery / sentiment / regime / codegen | AI discovery | `#102`-`#106` | explicit high-risk late-phase work |
| capital allocation / correlation / rebalance / multi-strategy UI / collision handling | multi-strategy | `#107`-`#111` | only after single-strategy governance remains stable |

## Validation of the issue set

- Coverage: every major memo theme maps to an epic, a child issue, or a completed reused issue.
- Deduplication: no new issue duplicates `#65`, `#53`, `#58`, `#59`, `#60`, `#61`, or `#10`.
- Ordering: all safety and reproducibility work precedes infrastructure, AI foundation, AI improvement, AI discovery, and portfolio work.
- Granularity: Pine and AI themes are compressed into coherent PR-sized slices rather than sentence-sized tickets.
- Direction alignment: every issue assumes Rust-first core, Pine as parity/spec, sandbox/paper-only guardrails, and approval-gated AI promotion.
