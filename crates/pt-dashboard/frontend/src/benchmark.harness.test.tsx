import { afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { promises as fs } from "node:fs";
import { dirname } from "node:path";
import App from "./App";

function jsonResponse(body: unknown): Response {
  return new Response(JSON.stringify(body), {
    status: 200,
    headers: { "Content-Type": "application/json" },
  });
}

function scannerRows(count: number) {
  return Array.from({ length: count }, (_, index) => ({
    product_id: index === 0 ? "BTC-USD" : `ALT-${index}-USD`,
    instrument: "spot",
    live_tradable: true,
    scan_only: false,
    spread_bps: 8.4 + index,
    imbalance: 0.28,
    tape_direction: 0.61,
    realized_volatility: 0.12,
    fill_rate_estimate: 0.73,
    active_strategy: "microstructure",
    score: 0.78 - index * 0.01,
    current_risk_eligibility: {
      live_tradable: true,
      scan_only: false,
      eligible: true,
      reasons: [],
    },
    best_bid: 61000 + index,
    best_ask: 61005 + index,
    mid_price: 61002.5 + index,
    action: "buy",
    priority_fill: true,
    one_way_persistence: 3,
    ts: "2026-04-26T08:00:00Z",
  }));
}

function strategiesResponse() {
  return {
    mode: "paper",
    live_arm: {
      armed: false,
      mode: "paper",
      reason: "paper only",
      auto_disarm_reason: null,
      armed_at: null,
      updated_at: "2026-04-26T08:00:00Z",
    },
    strategies: [
      {
        product_id: "BTC-USD",
        strategy_name: "microstructure",
        enabled: true,
        live_enabled: false,
        score_threshold: 0.52,
        quote_size_usd: 25,
        plugin_signal: 0.91,
      },
    ],
    imports: [
      {
        import_id: "imp-1",
        path: "data/strategy_lab/dashboard-btc.json",
        markets: ["BTC-USD"],
        best_variants: ["BTC-USD:microstructure_v2"],
        artifact_id: "artifact-imp-1",
        source_run_id: "jr-test-1",
        promotion_status: "imported_only",
        objective_score: 0.55,
      },
    ],
  };
}

function productDetail(price: number) {
  return {
    product: {
      product_id: "BTC-USD",
      instrument: "spot",
      base_currency: "BTC",
      quote_currency: "USD",
      status: "online",
      price,
      volume_24h: 123456.78,
      live_tradable: true,
      scan_only: false,
    },
    microstructure: {
      best_bid: price - 2.5,
      best_ask: price + 2.5,
      spread_bps: 8.4,
      imbalance: 0.28,
      tape_direction: 0.61,
      realized_volatility: 0.12,
      fill_rate_estimate: 0.73,
      one_way_persistence: 3,
    },
    strategy: {
      strategy_name: "microstructure",
      microstructure_score: 0.84,
      momentum_score: 0.56,
      volatility_score: 0.42,
      plugin_score: 0.91,
      composite_score: 0.78,
      priority_fill: true,
    },
    eligibility: {
      live_tradable: true,
      scan_only: false,
      eligible: true,
      reasons: [],
    },
    orders: [],
    imports: [
      {
        import_id: "imp-detail-1",
        path: "data/strategy_lab/dashboard-btc.json",
        best_variants: ["BTC-USD:microstructure_v2"],
        artifact_id: "artifact-imp-1",
        source_run_id: "jr-test-1",
        promotion_status: "imported_only",
        objective_score: 0.55,
      },
    ],
  };
}

function listingRows() {
  return [
    {
      product_id: "BTC-USD",
      asset_symbol: "BTC",
      base_currency: "BTC",
      quote_currency: "USD",
      stage: "monitor",
      headline: "Liquidity stable and strategy aligned.",
      composite_score: 0.79,
      liquidity_score: 0.66,
      sentiment_score: 0.52,
      unlock_risk_score: 0.2,
      route_ready: true,
      live_tradable: true,
      scan_only: false,
      priority_fill: true,
      tags: ["coinbase", "momentum"],
    },
  ];
}

function listingDetail() {
  return {
    product: {
      product_id: "BTC-USD",
      base_currency: "BTC",
      quote_currency: "USD",
      status: "online",
      live_tradable: true,
      scan_only: false,
    },
    stage: "monitor",
    headline: "Liquidity stable and strategy aligned.",
    summary: "Listing Radar is aggregating venue state, strategy evidence, and route readiness.",
    composite_score: 0.79,
    liquidity_score: 0.66,
    sentiment_score: 0.52,
    unlock_risk_score: 0.2,
    route_ready: true,
    priority_fill: true,
    catalysts: ["Wallet momentum improving"],
    insights: [],
    routes: [],
    eligibility: {
      live_tradable: true,
      scan_only: false,
      eligible: true,
      reasons: [],
    },
    imports: [],
  };
}

function strategyCandidates(score: number) {
  return {
    product_id: "BTC-USD",
    source_report_path: "data/strategy_lab/optimize-fixture.json",
    cycle_summary_path: "data/strategy_lab/hourly_optimizer_runs/cycle-fixture.json",
    candidates: [
      {
        rank: 1,
        product_id: "BTC-USD",
        selected_market: "BTC-USD",
        variant: "microstructure_v2",
        params: { short_window: 5, long_window: 21 },
        score,
        objective_breakdown: {
          net_return_after_costs: 0.12,
          drawdown_penalty: 0.03,
          turnover_penalty: 0.01,
          stability_penalty: 0.005,
          final_score: score,
        },
        stability: {
          splits_requested: 3,
          score_stddev: 0.01,
          return_stddev: 0.02,
          penalty: 0.005,
          positive_windows: 3,
        },
        risk_gate: {
          status: "pass",
          failure_count: 0,
          reason_codes: [],
        },
        promotion_gate: {
          status: "eligible_for_manual_review",
          requires_replay_acceptance: true,
          replay_acceptance_status: "pass",
          promotion_status: "promoted",
          source_run_id: "cycle-fixture-1",
        },
        rejection_reasons: [],
        source_report_path: "data/strategy_lab/optimize-fixture.json",
        cycle_summary_path: "data/strategy_lab/hourly_optimizer_runs/cycle-fixture.json",
      },
    ],
  };
}

type FrontendBenchmarkScenario = {
  first_render_ms: number;
  product_review_ms: number;
  strategy_switch_ms: number;
  agent_switch_ms: number;
  candle_update_latency_ms: number;
  overlay_redraw_latency_ms: number;
};

type FrontendBenchmarkReport = {
  generated_at: string;
  harness: string;
  fixture_label: string;
  command_metadata: {
    command: string;
  };
  metrics: {
    one_product: FrontendBenchmarkScenario;
    eight_product: FrontendBenchmarkScenario;
    memory_footprint_mb: null;
  };
  notes: {
    memory_footprint: string;
    evidence_boundary: string;
  };
};

function riskOverview() {
  return {
    killswitch: "running",
    daily_pnl: 12.4,
    open_notional: 25,
    unhedged_delta: 0,
    blocked_markets: 0,
    live_eligible_markets: 1,
    queued_notional: 25,
    live_orders: 0,
    taker_orders: 0,
    policy_breaches: [],
  };
}

function agentConsole() {
  return {
    autonomy_tier: "recommend_only",
    live_arm: strategiesResponse().live_arm,
    next_action: "Review BTC-USD chart and import lineage.",
    blocked_markets: 0,
    imports_loaded: 1,
    recommended_products: ["BTC-USD"],
    approvals: [],
  };
}

function installFetchMock(opts: { productCount: number; currentPrice: { value: number }; candidateScore: { value: number } }) {
  const fetchMock = vi.fn((input: RequestInfo | URL) => {
    const url = String(input);
    if (url === "/api/v1/scanner") return Promise.resolve(jsonResponse(scannerRows(opts.productCount)));
    if (url === "/api/v1/strategies") return Promise.resolve(jsonResponse(strategiesResponse()));
    if (url === "/api/v1/orders") return Promise.resolve(jsonResponse([]));
    if (url === "/api/v1/listings") return Promise.resolve(jsonResponse(listingRows()));
    if (url === "/api/v1/risk/overview") return Promise.resolve(jsonResponse(riskOverview()));
    if (url === "/api/v1/agent/console") return Promise.resolve(jsonResponse(agentConsole()));
    if (url === "/api/v1/strategy-candidates" || url === "/api/v1/strategy-candidates?product_id=BTC-USD") {
      return Promise.resolve(jsonResponse(strategyCandidates(opts.candidateScore.value)));
    }
    if (url === "/api/v1/products/BTC-USD") return Promise.resolve(jsonResponse(productDetail(opts.currentPrice.value)));
    if (url === "/api/v1/listings/BTC-USD") return Promise.resolve(jsonResponse(listingDetail()));
    return Promise.reject(new Error(`unexpected fetch: ${url}`));
  });
  vi.stubGlobal("fetch", fetchMock);
  return fetchMock;
}

async function measureScenario(productCount: number) {
  const currentPrice = { value: 61002.5 };
  const candidateScore = { value: 0.55 };
  installFetchMock({ productCount, currentPrice, candidateScore });

  const renderStart = performance.now();
  render(<App />);
  await screen.findByRole("heading", { name: "Selected Market" });
  const firstRenderMs = performance.now() - renderStart;

  const reviewStart = performance.now();
  await screen.findByRole("heading", { name: "Selected product review surface" });
  const productReviewMs = performance.now() - reviewStart;

  const strategyStart = performance.now();
  fireEvent.click(screen.getByRole("button", { name: /Validate Strategy Lab/i }));
  await screen.findByRole("heading", { name: "Candidate Review" });
  const strategySwitchMs = performance.now() - strategyStart;

  const agentStart = performance.now();
  fireEvent.click(screen.getByRole("button", { name: /Agent Console/i }));
  await screen.findByRole("heading", { name: "Approval Queue" });
  const agentSwitchMs = performance.now() - agentStart;

  cleanup();
  vi.unstubAllGlobals();

  currentPrice.value = 62002.5;
  installFetchMock({ productCount, currentPrice, candidateScore });
  const candleStart = performance.now();
  render(<App />);
  await screen.findByRole("heading", { name: "Selected Market" });
  await waitFor(() => expect(screen.getByText(/62,003 USD/i)).toBeInTheDocument());
  const candleUpdateMs = performance.now() - candleStart;

  cleanup();
  vi.unstubAllGlobals();

  candidateScore.value = 0.61;
  installFetchMock({ productCount, currentPrice, candidateScore });
  const overlayStart = performance.now();
  render(<App />);
  await screen.findByRole("heading", { name: "Selected Market" });
  fireEvent.click(screen.getByRole("button", { name: /Validate Strategy Lab/i }));
  await screen.findByRole("heading", { name: "Candidate Review" });
  await waitFor(() => expect(screen.getByText(/Score 0.610 · promotion promoted/i)).toBeInTheDocument());
  const overlayRedrawMs = performance.now() - overlayStart;

  cleanup();
  vi.unstubAllGlobals();

  return {
    first_render_ms: firstRenderMs,
    product_review_ms: productReviewMs,
    strategy_switch_ms: strategySwitchMs,
    agent_switch_ms: agentSwitchMs,
    candle_update_latency_ms: candleUpdateMs,
    overlay_redraw_latency_ms: overlayRedrawMs,
  };
}

function markdownReport(report: FrontendBenchmarkReport) {
  return [
    "# Frontend Benchmark",
    "",
    `- Generated at: ${report.generated_at}`,
    `- Harness: ${report.harness}`,
    `- Fixture label: ${report.fixture_label}`,
    "",
    "## Metrics",
    "",
    `- One-product first render: ${report.metrics.one_product.first_render_ms.toFixed(2)} ms`,
    `- Eight-product first render: ${report.metrics.eight_product.first_render_ms.toFixed(2)} ms`,
    `- Candle update latency: ${report.metrics.one_product.candle_update_latency_ms.toFixed(2)} ms`,
    `- Overlay redraw latency: ${report.metrics.one_product.overlay_redraw_latency_ms.toFixed(2)} ms`,
    `- Strategy tab switch: ${report.metrics.one_product.strategy_switch_ms.toFixed(2)} ms`,
    `- Agent tab switch: ${report.metrics.one_product.agent_switch_ms.toFixed(2)} ms`,
    "",
    "This is a local fixture-backed benchmark harness. It is not replay or paper evidence.",
    report.notes.memory_footprint,
  ].join("\n");
}

const shouldRun = !!process.env.PT_FRONTEND_BENCH_OUT;

afterEach(() => {
  cleanup();
  vi.useRealTimers();
  vi.unstubAllGlobals();
});

describe.skipIf(!shouldRun)("frontend benchmark harness", () => {
  it("captures a repeatable fixture-backed benchmark report", async () => {
    const oneProduct = await measureScenario(1);
    const eightProduct = await measureScenario(8);

    const report: FrontendBenchmarkReport = {
      generated_at: new Date().toISOString(),
      harness: "vitest_jsdom_headless_fixture",
      fixture_label: "dashboard_frontend_fixture_v1",
      command_metadata: {
        command: "pnpm exec vitest run src/benchmark.harness.test.tsx",
      },
      metrics: {
        one_product: oneProduct,
        eight_product: eightProduct,
        memory_footprint_mb: null,
      },
      notes: {
        memory_footprint: "Memory footprint unavailable under jsdom harness; report null instead of guessing.",
        evidence_boundary: "Fixture/local benchmark only. Not replay or paper evidence.",
      },
    };

    const jsonOut = process.env.PT_FRONTEND_BENCH_OUT!;
    const mdOut = process.env.PT_FRONTEND_BENCH_MD_OUT!;
    await fs.mkdir(dirname(jsonOut), { recursive: true });
    await fs.writeFile(jsonOut, JSON.stringify(report, null, 2), "utf8");
    await fs.writeFile(mdOut, markdownReport(report), "utf8");

    expect(report.metrics.one_product.first_render_ms).toBeGreaterThan(0);
    expect(report.metrics.eight_product.first_render_ms).toBeGreaterThan(0);
  }, 20000);
});
