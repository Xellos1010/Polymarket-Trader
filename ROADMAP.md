# Roadmap — Polymarket Trader

**Status:** This document is the **source of truth for product and technical direction**. Execution state (active issue, validation evidence, queue) lives in [`docs/development/WORK_STATUS.md`](docs/development/WORK_STATUS.md). When direction or phased outcomes change, update this file first, then align trackers and issues.

**Research baseline:** [`docs/research/deep-research-report (3).md`](docs/research/deep-research-report%20(3).md) — Rust-first core, swappable presentation, strategy outside the chart, research vs live planes.

**Reference strategy (Pine):** [`pine-scripts/multi-indicator-daterange.pine`](pine-scripts/multi-indicator-daterange.pine) (CTAPv6) and visual target [`pine-scripts/Example-TradingStrategy.png`](pine-scripts/Example-TradingStrategy.png). Pine is a **specification and parity reference**, not the runtime source of truth.

---

## 1. Vision

Build a **Rust-first** trading workspace where:

- **Strategy math, backtests, and optimization** run in validated, auditable engines (Rust and/or bounded lab tooling), not in the browser chart.
- **Charts and dashboards** are **subscribers** to engine outputs: candles, overlays, markers, equity curves, and run metadata—suitable for discretionary review and for AI-assisted search, without coupling refresh rate to execution or research throughput.
- **Coinbase (and later other venues)** are accessed through **normalized contracts** (metadata, candles, execution, risk) so the same strategy artifacts can be replayed, paper-traded, and (only with explicit gates) considered for live.
- Over time, the platform supports **TradingView-class** workflows: multi-pane studies, strategy report surfaces, parameter tuning, and **bounded** automated search—with every promoted path traceable to **artifact IDs** and evidence (replay / paper).

---

## 2. Principles (non-negotiables)

| Principle | Implication |
|-----------|-------------|
| Chart ≠ strategy | Frontend chart state is never authoritative for indicator or strategy logic. |
| No Pine clone first | Typed strategy IR / graph and artifacts (#58, #53) before full CTAPv6 parity in Rust. |
| Sandbox / paper default | No live expansion without explicit human approval; do not raise risk caps or inject credentials for convenience. |
| Presentation ≠ evidence | Synthetic or chart-only bars are not replay, backtest, or paper evidence (`WORK_STATUS` stage contract). |
| Bounded optimization | Candidate sweeps and AI proposals are **capped**, logged, and scored; promotion requires defined gates. |
| Replaceable UI | Engine APIs stable enough to swap Vite web, Tauri + Lightweight Charts, or native Rust UI later. |

---

## 3. North-star outcomes (“real results”)

These define success for the roadmap; phases below exist to reach them incrementally.

1. **Real market chart:** `BTC-USD` (and configurable products) with **user-selected granularity** and **historical range**, loaded from a **canonical Coinbase candle path**, visible in the operator dashboard or successor shell—not synthetic placeholder bars for that path.
2. **Reproducible backtest run:** A single command or API produces a **versioned artifact**: parameters hash, candle window, metrics (P&amp;L, drawdown, trade count, etc.), and optional trade list—consumable by UI and by CI-style harnesses.
3. **Visual strategy report:** Equity curve + trade markers + summary stats aligned with the same artifact as the chart (parity with the Pine reference **workflow**, not necessarily pixel-perfect TV clone on day one).
4. **Bounded search:** Optimizer or AI proposes **N** candidates per study; outputs are ranked, stored, and only **promoted** artifacts advance toward paper runtime per policy.
5. **Traceable paper handoff:** Imported / promoted strategies attach **artifact id**, provenance, and evidence fields through dashboard and APIs (issue #53 scope).

---

## 4. Architecture direction

- **Core:** Rust crates (`pt-core`, `pt-strategy-lab`, `pt-replay`, `pt-coinbase`, `pt-dashboard`, `pt-engine`, …) remain the system of record for behavior aligned with [`README.md`](README.md) crate layout.
- **Research plane:** Bulk history, indicator grids, walk-forward slices — columnar/batch-friendly patterns (existing `tools/coinbase_strategy_lab.py`, `pt-strategy-lab`); extend toward shared schemas and optional heavier analytics (Polars/Arrow) as needed per [`docs/research/deep-research-report (3).md`](docs/research/deep-research-report%20(3).md).
- **Live / stream plane (later):** Tokio-driven adapters, WebSocket market data, **coalesced** updates to the chart; REST for config, runs, and static metadata.
- **Presentation plane:** Dashboard Vite app and/or **Lightweight Charts** (or Tauri wrapper) as recommended in the research doc—**canvas-oriented** chart pipeline for dense updates; REST (then WS) for series data.

**Canonical Coinbase historical candles (MVP):** Public Exchange REST `GET /products/{product_id}/candles` with supported `granularity` and paginated `start`/`end` for long windows — already mirrored in [`crates/pt-strategy-lab/src/data.rs`](crates/pt-strategy-lab/src/data.rs) and [`tools/coinbase_strategy_lab.py`](tools/coinbase_strategy_lab.py). Advanced Trade candle APIs may coexist; **pick one canonical path per environment** and document it in the phase deliverables.

---

## 5. GitHub / work alignment

| Theme | Issues (indicative) |
|-------|---------------------|
| Artifact schema + paper handoff | #53 |
| Rust-native strategy IR | #58 |
| Bounded optimizer / AI sweep | #59 |
| Strategy AI review surfaces | #60 |
| Visual + benchmark harnesses | #61 |

Track B workstation UI work (#54–#56, merged via #57) is **foundation presentation**; this roadmap extends it with **real data**, **artifacts**, **IR**, and **measurement**.

---

## 6. Phased execution (implement in order; each phase has exit criteria)

Phases are **sequential gates**: skipping a gate risks unmaintainable UI-only strategy logic or non-reproducible “optimization.”

### Phase 0 — Baseline (maintain)

**Goal:** Repo stays safe, buildable, and validated.

**Includes:** `cargo fmt/check/clippy/test/build`, optional Nx + frontend build/test, `docs/LOCAL_VALIDATION.md` ladder; no live mode; credentials out of tree.

**Exit:** Green mainline per [`docs/development/WORK_STATUS.md`](docs/development/WORK_STATUS.md) validation commands.

---

### Phase 1 — Real chart data path (first “real result”)

**Goal:** Operator can see **actual** `BTC-USD` (default) candles for chosen **granularity** and **time range** in the product chart surface—not derived SVG placeholders for that data path.

**Deliverables:**

- HTTP API (or documented `pt-dashboard` / `pt-strategy-lab` extension) serving JSON OHLCV: `product_id`, `granularity`, `start`, `end`, pagination if over single-call limits.
- Dashboard or frontend consuming that API for at least one **chart-first** view; granularity selector limited to **Coinbase-supported** steps.
- README or config pointer: how to run engine + dashboard + frontend dev with proxy if needed.

**Exit criteria:**

- Loading `BTC-USD` 1h (or agreed default) shows **real** closes/highs/lows consistent with a spot-check against Exchange API.
- No claim that this path alone constitutes replay evidence.

**Dependencies:** None beyond Phase 0.

---

### Phase 2 — Strategy artifact spine (traceability)

**Goal:** Every serious backtest / optimization output is a **versioned artifact** with ids, provenance, and promotion fields—feeds dashboard and future paper runtime (`#53`).

**Deliverables:**

- Schema (JSON or Rust types + serialization) for: `artifact_id`, `source_run_id`, product mapping, `variant_id` / params hash, timeframe, scores, `promotion_status`, `replay_acceptance_status`, optional sizing hints.
- Wire imports from strategy lab outputs into dashboard/API surfaces as **read-only** context first.
- Single “run manifest” file produced by lab or Rust binary per run.

**Exit criteria:**

- Two runs with same params and data window produce **identical** deterministic hashes (where determinism applies).
- Dashboard can display **active artifact context** for a selected run.

**Dependencies:** Phase 1 optional for demo data, but artifact schema does not require chart.

---

### Phase 3 — Strategy IR + minimal Rust backtest (engine truth)

**Goal:** First **Rust-native** strategy slice: typed IR or graph, **vectorized or stepped** backtest, metrics + trade list JSON—**no** full CTAPv6 on day one (`#58`).

**Deliverables:**

- CTAPv6 **parameter catalog** derived from Pine inputs (groups, types, bounds)—as data, not Pine execution.
- Implement minimal path: e.g. OHLCV window + **RSI** + simple entry/exit + basic risk stub; tests against known toy series or frozen candle snippet.
- CLI or lab entry: `backtest --artifact …` / `--params-file …` returning artifact-linked output.

**Exit criteria:**

- Golden-file or numeric test asserts P&amp;L / trade count on a **fixed** candle fixture.
- Chart (Phase 1 surface) can **overlay** markers from this output (file or API).

**Dependencies:** Phase 2 strongly recommended so outputs land in artifact shape.

---

### Phase 4 — Visual workstation + benchmark harness (measurable UX)

**Goal:** TradingView-**class** workflow MVP: equity series, trade markers on time axis, summary panel numbers driven by **same** artifact as chart (`#61` + presentation).

**Deliverables:**

- Integrate **Lightweight Charts** (or agreed library) for main pane + at least one subpane (e.g. RSI) if engine emits series.
- “Strategy report” panel: drawdown, total trades, win rate, profit factor—mapped from artifact JSON.
- Harness: script or `cargo test` that fails if regression thresholds trip (smoke, not full stat arb).

**Exit criteria:**

- End-to-end demo: fetch candles → run backtest → open dashboard → see chart + report **without manual copy-paste** of JSON between tools (one script or documented compose target acceptable).

**Dependencies:** Phases 1–3.

---

### Phase 5 — Bounded optimization + AI review lane

**Goal:** **Systematic** parameter search and optional AI ranking **without** unbounded combinatorial explosion (`#59`, `#60`).

**Deliverables:**

- Search space definition from Phase 3 parameter catalog (discrete steps).
- Evaluator invoking **same** backtest binary / library as Phase 3 per candidate.
- Caps: max evaluations, wall time, concurrency; persisted study id + candidate rows.
- Thin “AI review” surface: narrative or structured critique of **promoted** or **candidate** sets from logs (no autonomous live orders).

**Exit criteria:**

- Optimize run produces ranked table + **artifact references**; top-k promotable only through defined gate (replay/paper flags from Phase 2 schema).

**Dependencies:** Phases 2–4.

---

### Phase 6 — Stream path + paper promotion hardening (“HFT-adjacent feel”)

**Goal:** **Sub-100 ms–class** updates where needed: WS (or SSE) candle/trade path, **coalescing**, optional compact/binary frames; REST unchanged for config and runs. Deepen **paper runtime** wiring for promoted artifacts only (`#53` continuation + live plane from research doc).

**Deliverables:**

- Market data adapter: subscribe, heartbeat, reconnect policy; server or client coalesce to chart-safe rate.
- Optional Tauri shell if web proxy/dev ergonomics block operators.
- Paper: promoted artifact drives signals with full traceability.

**Exit criteria:**

- Soak or scripted load test documents latency budget (p95 chart update under target for N updates/sec).
- Promotion path requires evidence fields non-empty per policy.

**Dependencies:** Phases 1–5.

---

## 7. CTAPv6 parity ladder (within Phase 3–5, not a single drop)

Implement incrementally inside the Rust IR / backtester:

1. Time window + execution flags (confirmed bar, direction, setup/confirm modes).  
2. RSI + BB / Fib BB family (simplify basis types if needed initially).  
3. Regime: chart MA + **HTF** alignment (e.g. 240) + volume / ATR filters.  
4. Full risk model (percent vs ATR, breakeven, opposite exit).  
5. Ichimoku + remaining Pine edge cases.  
6. Alert / webhook nodes as adapters (research: signals plane).

Visuals in Pine (`show*` inputs) map to **presentation flags** on artifacts, not to core P&amp;L.

---

## 8. Out of scope (until explicitly re-scoped)

- Live trading enablement without governance review.  
- Full TradingView Advanced Charts / Trading Platform licensing or embedding (legal review required per research doc).  
- Pixel-perfect clone of every TV drawing tool in early phases.  
- “AI places live orders” without hard policy and artifact promotion gates.

---

## 9. Document hierarchy

| Document | Role |
|----------|------|
| **ROADMAP.md** (this file) | Direction, phases, north stars, architecture commitments. |
| [`docs/development/WORK_STATUS.md`](docs/development/WORK_STATUS.md) | Current stage, active issue, queue, validation evidence. |
| [`AGENTS.md`](AGENTS.md) | Agent and contributor operational rules. |
| [`README.md`](README.md) | Onboarding, crates, commands; links here for **why** and **what’s next**. |
| [`docs/research/deep-research-report (3).md`](docs/research/deep-research-report%20(3).md) | External research synthesis and citations. |

When closing a phase, update **WORK_STATUS**, relevant **GitHub issues**, and the **Validation** subsection in README if commands or defaults change.

---

## 10. Validation commands (summary)

Rust:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

Frontend (when touched):

```bash
cd crates/pt-dashboard/frontend && npm test && npm run build
```

Full ladder: [`docs/LOCAL_VALIDATION.md`](docs/LOCAL_VALIDATION.md) or `pnpm exec nx run polymarket-trader:local-validation`.

Strategy lab (existing tooling):

```bash
python3 tools/coinbase_strategy_lab.py backtest --config config/coinbase_strategy_lab.json
python3 tools/coinbase_strategy_lab.py optimize --config config/coinbase_strategy_lab.json
```

---

*Last updated: roadmap file creation aligns execution with issues #53–#61 and research doc (3); keep dates in git history or bump explicitly when phases complete.*
