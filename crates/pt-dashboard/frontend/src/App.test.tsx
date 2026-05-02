import { afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import App from "./App";

function jsonResponse(body: unknown): Response {
  return new Response(JSON.stringify(body), {
    status: 200,
    headers: { "Content-Type": "application/json" },
  });
}

const scannerRows = [
  {
    product_id: "BTC-USD",
    instrument: "spot",
    live_tradable: true,
    scan_only: false,
    spread_bps: 8.4,
    imbalance: 0.28,
    tape_direction: 0.61,
    realized_volatility: 0.12,
    fill_rate_estimate: 0.73,
    active_strategy: "microstructure",
    score: 0.78,
    current_risk_eligibility: {
      live_tradable: true,
      scan_only: false,
      eligible: false,
      reasons: ["daily loss limit near threshold"],
    },
    best_bid: 61000,
    best_ask: 61005,
    mid_price: 61002.5,
    action: "buy",
    priority_fill: true,
    one_way_persistence: 3,
    ts: "2026-04-26T08:00:00Z",
  },
];

const strategiesResponse = {
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
      best_variants: ["microstructure_v2"],
    },
  ],
};

const ordersResponse = [
  {
    order_id: "ord-1",
    product_id: "BTC-USD",
    side: "Buy",
    route: "maker",
    status: "queued",
    live: false,
    post_only: true,
    limit_price: 61000,
    base_size: 0.001,
    quote_notional: 25,
    expected_net_bps: 12.4,
    reason: "spread capture",
    updated_at: "2026-04-26T08:00:00Z",
  },
];

const productDetail = {
  product: {
    product_id: "BTC-USD",
    instrument: "spot",
    base_currency: "BTC",
    quote_currency: "USD",
    status: "online",
    price: 61002.5,
    volume_24h: 123456.78,
    live_tradable: true,
    scan_only: false,
  },
  microstructure: {
    best_bid: 61000,
    best_ask: 61005,
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
    eligible: false,
    reasons: ["daily loss limit near threshold"],
  },
  orders: ordersResponse,
  imports: [{ path: "data/strategy_lab/dashboard-btc.json", best_variants: ["microstructure_v2"] }],
};

const listingRows = [
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

const listingDetail = {
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
  insights: [
    {
      provider: "signal-fusion",
      category: "microstructure",
      summary: "Order-flow tilt remains constructive.",
      freshness_label: "fresh",
      status: "ok",
    },
  ],
  routes: [
    {
      venue: "coinbase",
      route_type: "maker",
      readiness: "ready",
      tradable: true,
      notes: "Spread capture lane available.",
    },
  ],
  eligibility: productDetail.eligibility,
  imports: productDetail.imports,
};

const riskOverview = {
  killswitch: "running",
  daily_pnl: 12.4,
  open_notional: 25,
  unhedged_delta: 0,
  blocked_markets: 1,
  live_eligible_markets: 0,
  queued_notional: 25,
  live_orders: 0,
  taker_orders: 0,
  policy_breaches: ["daily loss limit near threshold"],
};

const agentConsole = {
  autonomy_tier: "recommend_only",
  live_arm: strategiesResponse.live_arm,
  next_action: "Review BTC-USD chart and import lineage.",
  blocked_markets: 1,
  imports_loaded: 1,
  recommended_products: ["BTC-USD"],
  approvals: [
    {
      id: "approval-1",
      title: "Hold for policy review",
      description: "BTC-USD remains paper only while the daily loss threshold is close.",
      severity: "medium",
      status: "open",
      product_id: "BTC-USD",
    },
  ],
};

function installFetchMock(overrides?: Record<string, Response>) {
  const fetchMock = vi.fn((input: RequestInfo | URL, init?: RequestInit) => {
    const url = String(input);
    if (init?.method === "POST" && url === "/api/v1/orders") {
      return Promise.resolve(
        jsonResponse({
          ...ordersResponse[0],
          order_id: "ord-2",
          reason: "manual",
        }),
      );
    }
    if (init?.method === "POST" && url === "/api/v1/mode") {
      return Promise.resolve(jsonResponse({ mode: "replay", live_arm: strategiesResponse.live_arm }));
    }
    if (overrides && overrides[url]) {
      return Promise.resolve(overrides[url].clone());
    }
    if (url === "/api/v1/scanner") {
      return Promise.resolve(jsonResponse(scannerRows));
    }
    if (url === "/api/v1/strategies") {
      return Promise.resolve(jsonResponse(strategiesResponse));
    }
    if (url === "/api/v1/orders") {
      return Promise.resolve(jsonResponse(ordersResponse));
    }
    if (url === "/api/v1/listings") {
      return Promise.resolve(jsonResponse(listingRows));
    }
    if (url === "/api/v1/risk/overview") {
      return Promise.resolve(jsonResponse(riskOverview));
    }
    if (url === "/api/v1/agent/console") {
      return Promise.resolve(jsonResponse(agentConsole));
    }
    if (url === "/api/v1/products/BTC-USD") {
      return Promise.resolve(jsonResponse(productDetail));
    }
    if (url === "/api/v1/listings/BTC-USD") {
      return Promise.resolve(jsonResponse(listingDetail));
    }
    return Promise.reject(new Error(`unexpected fetch: ${url}`));
  });

  vi.stubGlobal("fetch", fetchMock);
  return fetchMock;
}

afterEach(() => {
  cleanup();
  vi.unstubAllGlobals();
});

describe("App", () => {
  it("renders the chart-first workstation surface from fixture data", async () => {
    installFetchMock();

    render(<App />);

    expect(await screen.findByRole("heading", { name: "Selected Market" })).toBeInTheDocument();
    expect(await screen.findByRole("heading", { name: "Selected product review surface" })).toBeInTheDocument();
    expect(await screen.findByRole("heading", { name: "Paper-state lineage and guardrails" })).toBeInTheDocument();
    expect(screen.getByText("daily loss limit near threshold")).toBeInTheDocument();
    expect(screen.getByText("data/strategy_lab/dashboard-btc.json")).toBeInTheDocument();
    expect(screen.getByLabelText("Selected product price chart")).toBeInTheDocument();
  });

  it("submits a manual order against the current API route", async () => {
    const fetchMock = installFetchMock();

    render(<App />);

    await screen.findByRole("heading", { name: "Selected Market" });
    fireEvent.change(screen.getByPlaceholderText("Quote notional"), {
      target: { value: "40" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Queue Order" }));

    await waitFor(() => {
      expect(fetchMock).toHaveBeenCalledWith(
        "/api/v1/orders",
        expect.objectContaining({
          method: "POST",
          headers: { "content-type": "application/json" },
          body: JSON.stringify({
            product_id: "BTC-USD",
            side: "buy",
            route: "maker",
            quote_notional: 40,
            strategy_name: "microstructure",
            priority_fill: false,
          }),
        }),
      );
    });
  });

  it("submits a mode change against the current API route", async () => {
    const fetchMock = installFetchMock();

    render(<App />);

    await screen.findByRole("heading", { name: "Selected Market" });
    fireEvent.change(screen.getAllByRole("combobox")[0], {
      target: { value: "replay" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Apply Mode" }));

    await waitFor(() => {
      expect(fetchMock).toHaveBeenCalledWith(
        "/api/v1/mode",
        expect.objectContaining({
          method: "POST",
          body: JSON.stringify({ mode: "replay" }),
        }),
      );
    });
  });

  it("shows an error banner when an API request fails", async () => {
    installFetchMock({
      "/api/v1/strategies": new Response("boom", { status: 500 }),
    });

    render(<App />);

    await waitFor(() => {
      expect(screen.getByText("/api/v1/strategies failed with 500")).toBeInTheDocument();
    });
  });
});
