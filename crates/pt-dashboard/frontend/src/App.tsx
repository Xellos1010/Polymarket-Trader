import { FormEvent, useEffect, useMemo, useState } from "react";
import { formatBps, scoreTone } from "./format";

type Tone = "buy" | "sell" | "flat";
type WorkspaceId = "command" | "listing" | "risk" | "strategy" | "agent";
type ContractMode = "current-api" | "fixture-backed";

type LiveArmState = {
  armed: boolean;
  mode?: string | null;
  reason?: string | null;
  auto_disarm_reason?: string | null;
  armed_at?: string | null;
  updated_at?: string | null;
};

type TradingEligibility = {
  live_tradable: boolean;
  scan_only: boolean;
  eligible: boolean;
  reasons: string[];
};

type ScannerRow = {
  product_id: string;
  instrument?: "spot" | "derivative" | null;
  live_tradable: boolean;
  scan_only: boolean;
  spread_bps: number;
  imbalance: number;
  tape_direction: number;
  realized_volatility: number;
  fill_rate_estimate: number;
  active_strategy: string;
  score: number;
  current_risk_eligibility: TradingEligibility;
  best_bid: number;
  best_ask: number;
  mid_price: number;
  action?: "buy" | "sell" | "hold" | null;
  priority_fill: boolean;
  one_way_persistence: number;
  ts?: string | null;
};

type WorkstationOrder = {
  order_id: string;
  product_id: string;
  side?: "Buy" | "Sell" | null;
  route?: "maker" | "taker" | "scan_only" | null;
  status?: string | null;
  live: boolean;
  post_only: boolean;
  limit_price?: number | null;
  base_size: number;
  quote_notional: number;
  expected_net_bps: number;
  reason?: string | null;
  updated_at?: string | null;
};

type ProductImport = {
  import_id?: string;
  path: string;
  best_variants: string[];
  artifact_id?: string | null;
  source_run_id?: string | null;
  promotion_status?: string;
  replay_acceptance_status?: string | null;
  objective_score?: number | null;
  confidence?: number | null;
  timeframe?: string | null;
};

type ProductDetail = {
  product: {
    product_id: string;
    instrument?: "spot" | "derivative" | null;
    base_currency: string;
    quote_currency: string;
    status: string;
    price: number;
    volume_24h: number;
    live_tradable: boolean;
    scan_only: boolean;
  };
  microstructure: {
    best_bid: number;
    best_ask: number;
    spread_bps: number;
    imbalance: number;
    tape_direction: number;
    realized_volatility: number;
    fill_rate_estimate: number;
    one_way_persistence: number;
  };
  strategy: {
    strategy_name: string;
    microstructure_score: number;
    momentum_score: number;
    volatility_score: number;
    plugin_score: number;
    composite_score: number;
    priority_fill: boolean;
  };
  eligibility: TradingEligibility;
  orders: WorkstationOrder[];
  imports: ProductImport[];
};

type StrategiesResponse = {
  mode: string;
  live_arm: LiveArmState;
  strategies: Array<{
    product_id: string;
    strategy_name: string;
    enabled: boolean;
    live_enabled: boolean;
    score_threshold: number;
    quote_size_usd: number;
    plugin_signal: number;
  }>;
  imports: Array<{
    import_id: string;
    path: string;
    markets: string[];
    best_variants: string[];
    artifact_id?: string | null;
    source_run_id?: string | null;
    promotion_status?: string;
    replay_acceptance_status?: string | null;
    objective_score?: number | null;
    confidence?: number | null;
    timeframe?: string | null;
  }>;
};

type ListingRadarRow = {
  product_id: string;
  asset_symbol: string;
  base_currency: string;
  quote_currency: string;
  stage: string;
  headline: string;
  composite_score: number;
  liquidity_score: number;
  sentiment_score: number;
  unlock_risk_score: number;
  route_ready: boolean;
  live_tradable: boolean;
  scan_only: boolean;
  priority_fill: boolean;
  tags: string[];
};

type ProviderInsight = {
  provider: string;
  category: string;
  summary: string;
  freshness_label: string;
  status: string;
};

type ListingVenueRoute = {
  venue: string;
  route_type: string;
  readiness: string;
  tradable: boolean;
  notes: string;
};

type ListingRadarDetail = {
  product: {
    product_id: string;
    base_currency: string;
    quote_currency: string;
    status: string;
    live_tradable: boolean;
    scan_only: boolean;
  };
  stage: string;
  headline: string;
  summary: string;
  composite_score: number;
  liquidity_score: number;
  sentiment_score: number;
  unlock_risk_score: number;
  route_ready: boolean;
  priority_fill: boolean;
  catalysts: string[];
  insights: ProviderInsight[];
  routes: ListingVenueRoute[];
  eligibility: TradingEligibility;
  imports: ProductImport[];
};

type RiskOverview = {
  killswitch: string;
  daily_pnl: number;
  open_notional: number;
  unhedged_delta: number;
  blocked_markets: number;
  live_eligible_markets: number;
  queued_notional: number;
  live_orders: number;
  taker_orders: number;
  policy_breaches: string[];
};

type AgentApprovalItem = {
  id: string;
  title: string;
  description: string;
  severity: string;
  status: string;
  product_id?: string | null;
};

type AgentConsoleView = {
  autonomy_tier: string;
  live_arm: LiveArmState;
  next_action: string;
  blocked_markets: number;
  imports_loaded: number;
  recommended_products: string[];
  approvals: AgentApprovalItem[];
};

type StrategyCandidateReview = {
  rank: number;
  product_id?: string | null;
  selected_market?: string | null;
  variant: string;
  params: Record<string, number | string | boolean | null>;
  score: number;
  objective_breakdown: {
    net_return_after_costs: number;
    drawdown_penalty: number;
    turnover_penalty: number;
    stability_penalty: number;
    final_score: number;
  };
  stability: {
    splits_requested: number;
    score_stddev: number;
    return_stddev: number;
    penalty: number;
    positive_windows: number;
  };
  risk_gate: {
    status: string;
    failure_count: number;
    reason_codes: string[];
  };
  promotion_gate: {
    status: string;
    requires_replay_acceptance: boolean;
    replay_acceptance_status?: string | null;
    promotion_status?: string | null;
    source_run_id?: string | null;
  };
  rejection_reasons: string[];
  source_report_path?: string | null;
  cycle_summary_path?: string | null;
};

type StrategyCandidatesResponse = {
  product_id?: string | null;
  source_report_path?: string | null;
  cycle_summary_path?: string | null;
  candidates: StrategyCandidateReview[];
};

type WorkspaceTab = {
  id: WorkspaceId;
  label: string;
  kicker: string;
};

type SyntheticBar = {
  label: string;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
  spread: number;
  marker?: string;
};

const INITIAL_DETAIL: ProductDetail | null = null;
const WORKSPACES: WorkspaceTab[] = [
  { id: "command", label: "Command Center", kicker: "Operate" },
  { id: "listing", label: "Listing Radar", kicker: "Research" },
  { id: "risk", label: "Risk Cockpit", kicker: "Protect" },
  { id: "strategy", label: "Strategy Lab", kicker: "Validate" },
  { id: "agent", label: "Agent Console", kicker: "Supervise" },
];

async function getJson<T>(path: string, init?: RequestInit): Promise<T> {
  const response = await fetch(path, init);
  if (!response.ok) {
    throw new Error(`${path} failed with ${response.status}`);
  }
  return response.json() as Promise<T>;
}

function formatPercent(value: number): string {
  return `${(value * 100).toFixed(1)}%`;
}

function formatCompactPrice(value: number): string {
  if (value >= 1000) {
    return value.toLocaleString(undefined, { maximumFractionDigits: 0 });
  }
  if (value >= 1) {
    return value.toFixed(2);
  }
  return value.toFixed(4);
}

function contractLabel(mode: ContractMode): string {
  return mode === "current-api" ? "current API" : "fixture backed";
}

function buildSyntheticBars(detail: ProductDetail, selection: ScannerRow | null): SyntheticBar[] {
  const base = detail.product.price || selection?.mid_price || detail.microstructure.best_bid || 1;
  const spreadPx = Math.max(base * (detail.microstructure.spread_bps / 10000), base * 0.00018);
  const directionalBias = (detail.strategy.composite_score - 0.5) * 2.4;
  const rhythm = detail.microstructure.tape_direction * 0.7;

  return Array.from({ length: 18 }, (_, index) => {
    const wave = Math.sin(index * 0.55 + rhythm) * spreadPx * 4.6;
    const drift = directionalBias * spreadPx * index * 0.4;
    const anchor = base + drift + wave;
    const open = anchor - spreadPx * (0.8 + Math.cos(index * 0.33));
    const close = anchor + spreadPx * (0.55 + Math.sin(index * 0.49));
    const high = Math.max(open, close) + spreadPx * (1.5 + (index % 3) * 0.3);
    const low = Math.min(open, close) - spreadPx * (1.1 + (index % 4) * 0.24);
    const volume = detail.product.volume_24h / 48 + index * 320 + Math.abs(detail.microstructure.imbalance) * 1000;

    let marker: string | undefined;
    if (index === 5) {
      marker = "Bias inflection";
    } else if (index === 11 && detail.strategy.priority_fill) {
      marker = "Priority fill";
    } else if (index === 16 && detail.imports[0]?.best_variants[0]) {
      marker = detail.imports[0].best_variants[0];
    }

    return {
      label: `${index + 1}`,
      open,
      high,
      low,
      close,
      volume,
      spread: detail.microstructure.spread_bps,
      marker,
    };
  });
}

function VisualWorkstation({ detail, selection }: { detail: ProductDetail; selection: ScannerRow | null }) {
  const bars = useMemo(() => buildSyntheticBars(detail, selection), [detail, selection]);
  const highMax = Math.max(...bars.map((bar) => bar.high));
  const lowMin = Math.min(...bars.map((bar) => bar.low));
  const volumeMax = Math.max(...bars.map((bar) => bar.volume));
  const priceRange = Math.max(highMax - lowMin, detail.product.price * 0.001);

  return (
    <div className="workstation-grid">
      <section className="chart-stage">
        <div className="chart-stage__header">
          <div>
            <p className="eyebrow">Visual workstation</p>
            <h3>Selected product review surface</h3>
          </div>
          <div className="contract-stack">
            <span className="contract-chip contract-chip--current">{contractLabel("current-api")}</span>
            <span className="contract-chip contract-chip--fixture">{contractLabel("fixture-backed")}</span>
          </div>
        </div>

        <div className="chart-stage__meta">
          <div>
            <span>Source contract</span>
            <strong>`/api/v1/products/{detail.product.product_id}`</strong>
          </div>
          <div>
            <span>History lane</span>
            <strong>derived bars until product-history wiring lands</strong>
          </div>
          <div>
            <span>Overlay source</span>
            <strong>strategy, spread, import lineage</strong>
          </div>
        </div>

        <div className="chart-shell">
          <div className="price-pane">
            <div className="pane-header">
              <strong>{detail.product.product_id}</strong>
              <span>{formatCompactPrice(detail.product.price)} {detail.product.quote_currency}</span>
            </div>
            <svg viewBox="0 0 760 310" className="price-chart" role="img" aria-label="Selected product price chart">
              <defs>
                <linearGradient id="chartGlow" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="0%" stopColor="rgba(119, 230, 255, 0.2)" />
                  <stop offset="100%" stopColor="rgba(119, 230, 255, 0)" />
                </linearGradient>
              </defs>
              <rect x="0" y="0" width="760" height="310" rx="18" className="chart-backdrop" />
              {Array.from({ length: 4 }).map((_, index) => {
                const y = 42 + index * 58;
                return <line key={y} x1="28" y1={y} x2="732" y2={y} className="grid-line" />;
              })}
              {bars.map((bar, index) => {
                const step = 700 / bars.length;
                const x = 44 + index * step;
                const yHigh = 245 - ((bar.high - lowMin) / priceRange) * 180;
                const yLow = 245 - ((bar.low - lowMin) / priceRange) * 180;
                const yOpen = 245 - ((bar.open - lowMin) / priceRange) * 180;
                const yClose = 245 - ((bar.close - lowMin) / priceRange) * 180;
                const up = bar.close >= bar.open;
                const bodyY = Math.min(yOpen, yClose);
                const bodyHeight = Math.max(Math.abs(yClose - yOpen), 3);
                return (
                  <g key={`${bar.label}-${index}`}>
                    <line x1={x} y1={yHigh} x2={x} y2={yLow} className="wick-line" />
                    <rect
                      x={x - 9}
                      y={bodyY}
                      width="18"
                      height={bodyHeight}
                      rx="4"
                      className={up ? "candle candle--up" : "candle candle--down"}
                    />
                    {bar.marker ? (
                      <g>
                        <circle cx={x} cy={yHigh - 14} r="4" className="marker-dot" />
                        <text x={x} y={yHigh - 22} textAnchor="middle" className="marker-label">
                          {bar.marker}
                        </text>
                      </g>
                    ) : null}
                  </g>
                );
              })}
              <path
                d={`M 44 ${245 - ((bars[0].close - lowMin) / priceRange) * 180} ${bars
                  .map((bar, index) => {
                    const step = 700 / bars.length;
                    const x = 44 + index * step;
                    const y = 245 - ((bar.close - lowMin) / priceRange) * 180;
                    return `L ${x} ${y}`;
                  })
                  .join(" ")}`}
                className="close-line"
              />
            </svg>
          </div>

          <div className="volume-pane">
            <div className="pane-header">
              <strong>Activity lane</strong>
              <span>{formatBps(detail.microstructure.spread_bps)} spread</span>
            </div>
            <div className="volume-bars" aria-label="Selected product activity chart">
              {bars.map((bar, index) => (
                <div key={`${bar.label}-volume-${index}`} className="volume-bar-wrap">
                  <div
                    className="volume-bar"
                    style={{ height: `${Math.max((bar.volume / volumeMax) * 100, 8)}%` }}
                  />
                </div>
              ))}
            </div>
            <div className="volume-caption">
              <span>Volume is derived from current product detail for layout and benchmarking.</span>
            </div>
          </div>
        </div>
      </section>

      <aside className="strategy-rail">
        <div className="strategy-rail__header">
          <p className="eyebrow">Strategy review rail</p>
          <h3>Paper-state lineage and guardrails</h3>
        </div>

        <div className="rail-stack">
          <div className="rail-card">
            <span>Active strategy</span>
            <strong>{detail.strategy.strategy_name}</strong>
            <p>
              Composite score {detail.strategy.composite_score.toFixed(3)} with
              {detail.strategy.priority_fill ? " priority fill enabled." : " standard execution posture."}
            </p>
          </div>

          <div className="rail-card">
            <span>Imported lineage</span>
            <strong>{detail.imports[0]?.best_variants[0] ?? "No imported variant active"}</strong>
            <p>{detail.imports[0]?.path ?? "This product is currently running without imported strategy-lab lineage."}</p>
            {detail.imports[0] ? (
              <p className="muted">
                Artifact <code>{detail.imports[0].artifact_id ?? detail.imports[0].import_id ?? "—"}</code>
                {detail.imports[0].source_run_id ? (
                  <>
                    {" "}
                    · run <code>{detail.imports[0].source_run_id}</code>
                  </>
                ) : null}
                {" · "}
                {(detail.imports[0].promotion_status ?? "imported_only").replace(/_/g, " ")}
                {detail.imports[0].objective_score != null && detail.imports[0].objective_score !== undefined
                  ? ` · lab preview ${detail.imports[0].objective_score.toFixed(3)}`
                  : null}
              </p>
            ) : null}
          </div>

          <div className="rail-metrics">
            <div className="rail-metric">
              <span>Micro</span>
              <strong>{detail.strategy.microstructure_score.toFixed(3)}</strong>
            </div>
            <div className="rail-metric">
              <span>Momentum</span>
              <strong>{detail.strategy.momentum_score.toFixed(3)}</strong>
            </div>
            <div className="rail-metric">
              <span>Volatility</span>
              <strong>{detail.strategy.volatility_score.toFixed(3)}</strong>
            </div>
            <div className="rail-metric">
              <span>Plugin</span>
              <strong>{detail.strategy.plugin_score.toFixed(3)}</strong>
            </div>
          </div>

          <div className="rail-card rail-card--guardrail">
            <span>Routing guardrails</span>
            <strong>
              {detail.eligibility.eligible ? "Eligible for paper routing" : detail.eligibility.scan_only ? "Scan only" : "Policy hold active"}
            </strong>
            <ul>
              {detail.eligibility.reasons.length === 0 ? <li>No active policy holds.</li> : detail.eligibility.reasons.map((reason) => <li key={reason}>{reason}</li>)}
            </ul>
          </div>

          <div className="rail-card rail-card--contract">
            <span>Contract stance</span>
            <strong>Current API for strategy state, fixture-backed for history bars</strong>
            <p>
              This first visual slice is safe for operator review and benchmarking, but it is not replay evidence.
            </p>
          </div>
        </div>
      </aside>
    </div>
  );
}

export default function App() {
  const [scanner, setScanner] = useState<ScannerRow[]>([]);
  const [orders, setOrders] = useState<WorkstationOrder[]>([]);
  const [strategies, setStrategies] = useState<StrategiesResponse | null>(null);
  const [listingRows, setListingRows] = useState<ListingRadarRow[]>([]);
  const [riskOverview, setRiskOverview] = useState<RiskOverview | null>(null);
  const [agentConsole, setAgentConsole] = useState<AgentConsoleView | null>(null);
  const [strategyCandidates, setStrategyCandidates] = useState<StrategyCandidatesResponse | null>(null);
  const [selectedProduct, setSelectedProduct] = useState<string>("");
  const [detail, setDetail] = useState<ProductDetail | null>(INITIAL_DETAIL);
  const [listingDetail, setListingDetail] = useState<ListingRadarDetail | null>(null);
  const [modeDraft, setModeDraft] = useState("paper");
  const [importPath, setImportPath] = useState("");
  const [orderNotional, setOrderNotional] = useState("25");
  const [route, setRoute] = useState("maker");
  const [side, setSide] = useState("buy");
  const [busy, setBusy] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [activeWorkspace, setActiveWorkspace] = useState<WorkspaceId>("command");

  useEffect(() => {
    let cancelled = false;

    async function tick() {
      try {
        const [scannerRows, strategyState, orderRows, listingRadarRows, nextRiskOverview, nextAgentConsole, nextStrategyCandidates] =
          await Promise.all([
            getJson<ScannerRow[]>("/api/v1/scanner"),
            getJson<StrategiesResponse>("/api/v1/strategies"),
            getJson<WorkstationOrder[]>("/api/v1/orders"),
            getJson<ListingRadarRow[]>("/api/v1/listings"),
            getJson<RiskOverview>("/api/v1/risk/overview"),
            getJson<AgentConsoleView>("/api/v1/agent/console"),
            getJson<StrategyCandidatesResponse>(
              `/api/v1/strategy-candidates${selectedProduct ? `?product_id=${encodeURIComponent(selectedProduct)}` : ""}`,
            ),
          ]);
        if (cancelled) {
          return;
        }
        setScanner(scannerRows);
        setStrategies(strategyState);
        setOrders(orderRows);
        setListingRows(listingRadarRows);
        setRiskOverview(nextRiskOverview);
        setAgentConsole(nextAgentConsole);
        setStrategyCandidates(nextStrategyCandidates);
        setModeDraft(strategyState.mode);
        if (!selectedProduct) {
          const fallbackProduct = scannerRows[0]?.product_id ?? listingRadarRows[0]?.product_id;
          if (fallbackProduct) {
            setSelectedProduct(fallbackProduct);
          }
        }
      } catch (nextError) {
        if (!cancelled) {
          setError(nextError instanceof Error ? nextError.message : String(nextError));
        }
      }
    }

    tick();
    const timer = window.setInterval(tick, 2000);
    return () => {
      cancelled = true;
      window.clearInterval(timer);
    };
  }, [selectedProduct]);

  useEffect(() => {
    if (!selectedProduct) {
      setDetail(null);
      setListingDetail(null);
      return;
    }
    let cancelled = false;
    Promise.all([
      getJson<ProductDetail>(`/api/v1/products/${encodeURIComponent(selectedProduct)}`),
      getJson<ListingRadarDetail>(`/api/v1/listings/${encodeURIComponent(selectedProduct)}`),
    ])
      .then(([productDetail, nextListingDetail]) => {
        if (!cancelled) {
          setDetail(productDetail);
          setListingDetail(nextListingDetail);
        }
      })
      .catch((nextError) => {
        if (!cancelled) {
          setError(nextError instanceof Error ? nextError.message : String(nextError));
        }
      });
    return () => {
      cancelled = true;
    };
  }, [selectedProduct, orders]);

  const topRow = scanner[0];
  const tone: Tone = topRow ? scoreTone(topRow.score) : "flat";
  const liveArm = strategies?.live_arm;
  const primaryCandidate = strategyCandidates?.candidates[0] ?? null;

  const selection = useMemo(() => {
    return scanner.find((row) => row.product_id === selectedProduct) ?? null;
  }, [scanner, selectedProduct]);

  const blockedCount = riskOverview?.blocked_markets ?? scanner.filter((row) => !row.current_risk_eligibility.eligible).length;
  const liveEligibleCount =
    riskOverview?.live_eligible_markets ??
    scanner.filter((row) => row.live_tradable && row.current_risk_eligibility.eligible && !row.scan_only).length;
  const takerOrders = riskOverview?.taker_orders ?? orders.filter((order) => order.route === "taker").length;
  const liveOrders = riskOverview?.live_orders ?? orders.filter((order) => order.live).length;
  const totalQueuedNotional =
    riskOverview?.queued_notional ?? orders.reduce((sum, order) => sum + order.quote_notional, 0);

  async function postJson(path: string, body: object) {
    setBusy(path);
    setError(null);
    try {
      await getJson(path, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify(body),
      });
    } catch (nextError) {
      setError(nextError instanceof Error ? nextError.message : String(nextError));
    } finally {
      setBusy(null);
    }
  }

  function submitManualOrder(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!selectedProduct) {
      return;
    }
    void postJson("/api/v1/orders", {
      product_id: selectedProduct,
      side,
      route,
      quote_notional: Number(orderNotional),
      strategy_name: selection?.active_strategy ?? "manual",
      priority_fill: route === "taker",
    });
  }

  const workspaceContent = (() => {
    if (activeWorkspace === "listing") {
      return (
        <section className="workspace-stack">
          <div className="spotlight-band">
            <div>
              <p className="eyebrow">Primary catalyst</p>
              <h2>{listingDetail?.product.product_id ?? listingRows[0]?.product_id ?? "Awaiting catalyst"}</h2>
              <p className="muted">{listingDetail?.summary ?? "Listing Radar is aggregating venue state, strategy evidence, and route readiness."}</p>
            </div>
            <div className="spotlight-stats">
              <div className="vector-card">
                <span>Composite score</span>
                <strong>{listingDetail ? listingDetail.composite_score.toFixed(3) : "--"}</strong>
              </div>
              <div className="vector-card">
                <span>Liquidity score</span>
                <strong>{listingDetail ? listingDetail.liquidity_score.toFixed(3) : "--"}</strong>
              </div>
              <div className="vector-card">
                <span>Stage</span>
                <strong>{listingDetail?.stage ?? "research"}</strong>
              </div>
              <div className="vector-card">
                <span>Route ready</span>
                <strong>{listingDetail?.route_ready ? "Yes" : "No"}</strong>
              </div>
            </div>
          </div>

          <div className="workspace-grid tertiary">
            <div className="panel">
              <div className="panel-title">
                <h2>Listing Readiness Board</h2>
                <span>{listingRows.length} products watched</span>
              </div>
              <div className="candidate-grid">
                {listingRows.slice(0, 6).map((row) => (
                  <article key={row.product_id} className="candidate-card">
                    <header>
                      <strong>{row.product_id}</strong>
                      <span className={`score-pill ${scoreTone(row.composite_score)}`}>
                        {row.composite_score.toFixed(3)}
                      </span>
                    </header>
                    <p>{row.headline}</p>
                    <div className="candidate-metrics">
                      <span>{row.stage}</span>
                      <span>{formatPercent(row.liquidity_score)} liquidity</span>
                      <span>{row.route_ready ? "route ready" : "research route"}</span>
                    </div>
                  </article>
                ))}
              </div>
            </div>

            <div className="panel">
              <div className="panel-title">
                <h2>Provider Lanes</h2>
                <span>{listingDetail?.insights.length ?? 0} active</span>
              </div>
              <div className="provider-grid">
                {(listingDetail?.insights ?? []).map((insight) => (
                  <div key={`${insight.provider}-${insight.category}`} className="provider-card">
                    <strong>{insight.provider}</strong>
                    <p>{insight.summary}</p>
                  </div>
                ))}
              </div>
            </div>
          </div>

          <div className="top-grid strategy-grid">
            <div className="panel">
              <div className="panel-title">
                <h2>Catalysts</h2>
                <span>{listingDetail?.catalysts.length ?? 0}</span>
              </div>
              <div className="reason-stack">
                {(listingDetail?.catalysts ?? []).map((catalyst) => (
                  <div key={catalyst} className="reason-card">
                    <strong>Signal</strong>
                    <p>{catalyst}</p>
                  </div>
                ))}
              </div>
            </div>

            <div className="panel">
              <div className="panel-title">
                <h2>Route Map</h2>
                <span>{listingDetail?.routes.length ?? 0}</span>
              </div>
              <div className="reason-stack">
                {(listingDetail?.routes ?? []).map((routeItem) => (
                  <div key={`${routeItem.venue}-${routeItem.route_type}`} className="reason-card">
                    <strong>{routeItem.venue}</strong>
                    <p>{routeItem.notes}</p>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </section>
      );
    }

    if (activeWorkspace === "risk") {
      return (
        <section className="workspace-stack">
          <div className="top-grid risk-grid">
            <div className="panel">
              <div className="panel-title">
                <h2>Guardrail Summary</h2>
                <span>{riskOverview?.killswitch ?? "Always-on"}</span>
              </div>
              <div className="metric-strip tall">
                <div>
                  <span>Live-eligible</span>
                  <strong>{liveEligibleCount}</strong>
                </div>
                <div>
                  <span>Blocked</span>
                  <strong>{blockedCount}</strong>
                </div>
                <div>
                  <span>Live orders</span>
                  <strong>{liveOrders}</strong>
                </div>
                <div>
                  <span>Taker orders</span>
                  <strong>{takerOrders}</strong>
                </div>
              </div>
            </div>

            <div className="panel">
              <div className="panel-title">
                <h2>Policy Breaches</h2>
                <span>{selectedProduct || "Scanner-driven"}</span>
              </div>
              <div className="reason-stack">
                {(riskOverview?.policy_breaches ?? detail?.eligibility.reasons ?? []).length > 0 ? (
                  (riskOverview?.policy_breaches ?? detail?.eligibility.reasons ?? []).map((reason) => (
                    <div key={reason} className="reason-card">
                      <strong>Policy hold</strong>
                      <p>{reason}</p>
                    </div>
                  ))
                ) : (
                  <div className="reason-card good">
                    <strong>Eligible</strong>
                    <p>Current risk posture is clear for automated routing under the present policy.</p>
                  </div>
                )}
              </div>
            </div>
          </div>

          <div className="panel">
            <div className="panel-title">
              <h2>Queued Exposure</h2>
              <span>${totalQueuedNotional.toFixed(2)} notional</span>
            </div>
            <div className="table-shell">
              <table>
                <thead>
                  <tr>
                    <th>Product</th>
                    <th>Route</th>
                    <th>Live</th>
                    <th>Expected Net</th>
                    <th>Reason</th>
                  </tr>
                </thead>
                <tbody>
                  {orders.slice(0, 10).map((order) => (
                    <tr key={order.order_id}>
                      <td>{order.product_id}</td>
                      <td>{order.route ?? "-"}</td>
                      <td>{order.live ? "yes" : "no"}</td>
                      <td>{formatBps(order.expected_net_bps)}</td>
                      <td>{order.reason ?? "-"}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        </section>
      );
    }

    if (activeWorkspace === "strategy") {
      return (
        <section className="workspace-stack">
          <div className="top-grid strategy-grid">
            <div className="panel">
              <div className="panel-title">
                <h2>Imported Research</h2>
                <span>{strategies?.imports.length ?? 0}</span>
              </div>
              <div className="import-timeline">
                {(strategies?.imports ?? []).length > 0 ? (
                  strategies?.imports.map((item) => (
                    <div key={item.import_id} className="timeline-card">
                      <strong>{item.path}</strong>
                      <p>{item.markets.join(", ") || "No markets recorded"}</p>
                      <small>{item.best_variants.join(", ") || "No best variants yet"}</small>
                    </div>
                  ))
                ) : (
                  <div className="timeline-card">
                    <strong>No imports loaded</strong>
                    <p>Bring in strategy-lab outputs to seed replay-ready ideas.</p>
                  </div>
                )}
              </div>
            </div>

            <div className="panel">
              <div className="panel-title">
                <h2>Candidate Review</h2>
                <span>{strategyCandidates?.candidates.length ?? 0}</span>
              </div>
              <div className="reason-stack">
                {primaryCandidate ? (
                  <>
                    <div className="reason-card">
                      <strong>
                        #{primaryCandidate.rank} {primaryCandidate.variant}
                      </strong>
                      <p>
                        Score {primaryCandidate.score.toFixed(3)} · promotion {primaryCandidate.promotion_gate.promotion_status ?? primaryCandidate.promotion_gate.status}
                        {primaryCandidate.promotion_gate.replay_acceptance_status
                          ? ` · replay ${primaryCandidate.promotion_gate.replay_acceptance_status}`
                          : ""}
                      </p>
                    </div>
                    <div className="reason-card">
                      <strong>Objective breakdown</strong>
                      <p>
                        net {primaryCandidate.objective_breakdown.net_return_after_costs.toFixed(3)} ·
                        dd {primaryCandidate.objective_breakdown.drawdown_penalty.toFixed(3)} ·
                        turn {primaryCandidate.objective_breakdown.turnover_penalty.toFixed(3)} ·
                        stability {primaryCandidate.objective_breakdown.stability_penalty.toFixed(3)}
                      </p>
                    </div>
                    <div className="reason-card">
                      <strong>Lineage</strong>
                      <p>
                        {(primaryCandidate.selected_market ?? primaryCandidate.product_id ?? "market pending")} ·
                        {primaryCandidate.promotion_gate.source_run_id ? ` run ${primaryCandidate.promotion_gate.source_run_id}` : " local candidate"}
                      </p>
                    </div>
                  </>
                ) : (
                  <div className="reason-card">
                    <strong>No candidate report loaded</strong>
                    <p>Run the local optimizer lane to populate ranking, rejection, and promotion evidence.</p>
                  </div>
                )}
              </div>
            </div>
          </div>

          <div className="panel">
            <div className="panel-title">
              <h2>Ranked Candidates</h2>
              <span>{selectedProduct || "all products"}</span>
            </div>
            <div className="table-shell">
              <table>
                <thead>
                  <tr>
                    <th>Rank</th>
                    <th>Variant</th>
                    <th>Params</th>
                    <th>Score</th>
                    <th>Risk</th>
                    <th>Promotion</th>
                    <th>Rejections</th>
                  </tr>
                </thead>
                <tbody>
                  {(strategyCandidates?.candidates ?? []).map((candidate) => (
                    <tr key={`${candidate.variant}-${candidate.rank}`}>
                      <td>{candidate.rank}</td>
                      <td>{candidate.variant}</td>
                      <td>
                        s{candidate.params.short_window ?? "-"} / l{candidate.params.long_window ?? "-"}
                      </td>
                      <td>{candidate.score.toFixed(3)}</td>
                      <td>{candidate.risk_gate.status}</td>
                      <td>{candidate.promotion_gate.promotion_status ?? candidate.promotion_gate.status}</td>
                      <td>{candidate.rejection_reasons.join(", ") || "-"}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
            <p className="muted">
              Candidate ranking is optimizer evidence only. Replay and paper evidence remain separate gates.
            </p>
          </div>
        </section>
      );
    }

    if (activeWorkspace === "agent") {
      return (
        <section className="workspace-stack">
          <div className="panel">
            <div className="panel-title">
              <h2>Agent Posture</h2>
              <span>{agentConsole?.autonomy_tier ?? strategies?.mode ?? "paper"}</span>
            </div>
            <div className="agent-grid">
              <div className="agent-card">
                <span>Autonomy</span>
                <strong>{agentConsole?.autonomy_tier ?? (liveArm?.armed ? "bounded_execute" : "recommend_only")}</strong>
                <p>
                  Agent authority remains bounded by live arm state, strategy thresholds, approval queue,
                  and explicit route policy.
                </p>
              </div>
              <div className="agent-card">
                <span>Next best action</span>
                <strong>{agentConsole?.next_action ?? "Waiting for operator context"}</strong>
                <p>
                  Recommended products: {(agentConsole?.recommended_products ?? []).join(", ") || "none yet"}.
                </p>
              </div>
              <div className="agent-card">
                <span>Evidence set</span>
                <strong>{agentConsole?.imports_loaded ?? strategies?.imports.length ?? 0} strategy imports</strong>
                <p>{agentConsole?.blocked_markets ?? blockedCount} blocked markets still require policy review.</p>
              </div>
            </div>
          </div>

          <div className="panel">
            <div className="panel-title">
              <h2>Approval Queue</h2>
              <span>{agentConsole?.approvals.length ?? 0}</span>
            </div>
            <div className="reason-stack">
              {(agentConsole?.approvals ?? []).map((item) => (
                <div key={item.id} className="reason-card">
                  <strong>{item.title}</strong>
                  <p>{item.description}</p>
                </div>
              ))}
            </div>
          </div>
        </section>
      );
    }

    return (
      <>
        <section className="workspace-grid">
          <div className="panel scanner-panel">
            <div className="panel-title">
              <h2>Scanner</h2>
              <span>{scanner.length} ranked</span>
            </div>
            <div className="scanner-list">
              {scanner.map((row) => {
                const rowTone = scoreTone(row.score);
                return (
                  <button
                    key={row.product_id}
                    className={`scanner-row ${selectedProduct === row.product_id ? "selected" : ""}`}
                    onClick={() => setSelectedProduct(row.product_id)}
                  >
                    <div>
                      <strong>{row.product_id}</strong>
                      <small>{row.active_strategy}</small>
                    </div>
                    <div className={`score-pill ${rowTone}`}>{row.score.toFixed(3)}</div>
                    <div className="scanner-meta">
                      <span>{formatBps(row.spread_bps)}</span>
                      <span>imb {row.imbalance.toFixed(2)}</span>
                      <span>fill {(row.fill_rate_estimate * 100).toFixed(0)}%</span>
                    </div>
                  </button>
                );
              })}
            </div>
          </div>

          <div className="panel detail-panel">
            <div className="panel-title">
              <h2>Selected Market</h2>
              <span>{detail?.product.instrument ?? "..."}</span>
            </div>
            {detail ? (
              <div className="detail-grid">
                <div className="metric-strip">
                  <div>
                    <span>Bid</span>
                    <strong>{detail.microstructure.best_bid.toFixed(4)}</strong>
                  </div>
                  <div>
                    <span>Ask</span>
                    <strong>{detail.microstructure.best_ask.toFixed(4)}</strong>
                  </div>
                  <div>
                    <span>Spread</span>
                    <strong>{formatBps(detail.microstructure.spread_bps)}</strong>
                  </div>
                  <div>
                    <span>Score</span>
                    <strong>{detail.strategy.composite_score.toFixed(3)}</strong>
                  </div>
                </div>

                <VisualWorkstation detail={detail} selection={selection} />

                <form className="trade-ticket" onSubmit={submitManualOrder}>
                  <div className="ticket-row">
                    <select value={side} onChange={(event) => setSide(event.target.value)}>
                      <option value="buy">Buy</option>
                      <option value="sell">Sell</option>
                    </select>
                    <select value={route} onChange={(event) => setRoute(event.target.value)}>
                      <option value="maker">Maker</option>
                      <option value="taker">Taker</option>
                      <option value="scan_only">Scan Only</option>
                    </select>
                    <input
                      value={orderNotional}
                      onChange={(event) => setOrderNotional(event.target.value)}
                      placeholder="Quote notional"
                    />
                    <button className="primary" type="submit">
                      Queue Order
                    </button>
                  </div>
                </form>
              </div>
            ) : (
              <p className="muted">Select a market from the scanner.</p>
            )}
          </div>
        </section>

        <section className="bottom-grid">
          <div className="panel">
            <div className="panel-title">
              <h2>Orders</h2>
              <span>{orders.length}</span>
            </div>
            <div className="table-shell">
              <table>
                <thead>
                  <tr>
                    <th>Product</th>
                    <th>Status</th>
                    <th>Route</th>
                    <th>Notional</th>
                    <th>Reason</th>
                  </tr>
                </thead>
                <tbody>
                  {orders.slice(0, 12).map((order) => (
                    <tr key={order.order_id}>
                      <td>{order.product_id}</td>
                      <td>{order.status ?? "pending"}</td>
                      <td>{order.route ?? "-"}</td>
                      <td>{order.quote_notional.toFixed(2)}</td>
                      <td>{order.reason ?? "-"}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>

          <div className="panel">
            <div className="panel-title">
              <h2>Strategy Map</h2>
              <span>{strategies?.strategies.length ?? 0}</span>
            </div>
            <div className="strategy-list">
              {(strategies?.strategies ?? []).slice(0, 8).map((strategy) => (
                <div key={strategy.product_id} className="strategy-card">
                  <strong>{strategy.product_id}</strong>
                  <span>{strategy.strategy_name}</span>
                  <small>
                    threshold {strategy.score_threshold.toFixed(2)} / ${strategy.quote_size_usd.toFixed(0)}
                  </small>
                </div>
              ))}
            </div>
          </div>
        </section>
      </>
    );
  })();

  return (
    <div className="app-shell">
      <header className="masthead">
        <div className="brand-lockup">
          <p className="eyebrow">Coinbase-first AI trading terminal</p>
          <h1>Deterministic execution, adaptive supervision.</h1>
        </div>
        <div className="hero-rail">
          <div className={`signal-card tone-${tone}`}>
            <span>Top signal</span>
            <strong>{topRow?.product_id ?? "Waiting for scanner"}</strong>
            <small>{topRow ? `${formatBps(topRow.spread_bps)} spread` : "No market data yet"}</small>
          </div>
          <div className="mini-stat">
            <span>Mode</span>
            <strong>{strategies?.mode ?? "paper"}</strong>
          </div>
          <div className="mini-stat">
            <span>Imports</span>
            <strong>{strategies?.imports.length ?? 0}</strong>
          </div>
        </div>
      </header>

      <section className="mission-band">
        <p className="lede">
          The workstation now separates operating views for execution, listing research, risk,
          strategy validation, and agent governance while continuing to run on the current API surface.
        </p>
        <div className="mission-stats">
          <div>
            <span>Markets ranked</span>
            <strong>{scanner.length}</strong>
          </div>
          <div>
            <span>Live-eligible</span>
            <strong>{liveEligibleCount}</strong>
          </div>
          <div>
            <span>Queued notional</span>
            <strong>${totalQueuedNotional.toFixed(0)}</strong>
          </div>
          <div>
            <span>Live arm</span>
            <strong>{liveArm?.armed ? "armed" : "disarmed"}</strong>
          </div>
        </div>
      </section>

      <nav className="workspace-nav" aria-label="Workspace navigation">
        {WORKSPACES.map((workspace) => (
          <button
            key={workspace.id}
            className={`workspace-tab ${activeWorkspace === workspace.id ? "active" : ""}`}
            onClick={() => setActiveWorkspace(workspace.id)}
            type="button"
          >
            <span>{workspace.kicker}</span>
            <strong>{workspace.label}</strong>
          </button>
        ))}
      </nav>

      <section className="top-grid">
        <div className="panel control-panel">
          <div className="panel-title">
            <h2>Mode & Arm</h2>
            <span>{liveArm?.armed ? "Armed" : "Disarmed"}</span>
          </div>
          <div className="mode-row">
            <select value={modeDraft} onChange={(event) => setModeDraft(event.target.value)}>
              <option value="replay">Replay</option>
              <option value="paper">Paper</option>
              <option value="live">Live</option>
            </select>
            <button onClick={() => void postJson("/api/v1/mode", { mode: modeDraft })}>Apply Mode</button>
          </div>
          <div className="button-row">
            <button
              className="primary"
              disabled={busy !== null}
              onClick={() => void postJson("/api/v1/live/arm", { reason: "operator arm" })}
            >
              Arm Live
            </button>
            <button
              className="ghost"
              disabled={busy !== null}
              onClick={() => void postJson("/api/v1/live/disarm", { reason: "operator disarm" })}
            >
              Disarm
            </button>
          </div>
          <p className="muted">{liveArm?.auto_disarm_reason ?? liveArm?.reason ?? "No arm event yet."}</p>
        </div>

        <div className="panel import-panel">
          <div className="panel-title">
            <h2>Strategy-Lab Import</h2>
            <span>{strategies?.imports.length ?? 0} loaded</span>
          </div>
          <div className="mode-row">
            <input
              value={importPath}
              onChange={(event) => setImportPath(event.target.value)}
              placeholder="data/strategy_lab/dashboard-....json"
            />
            <button onClick={() => void postJson("/api/v1/strategy-lab/import", { path: importPath })}>Import</button>
          </div>
          <div className="import-list">
            {(strategies?.imports ?? []).slice(0, 3).map((item) => (
              <div key={item.import_id} className="mini-chip">
                <strong>{item.markets.length}</strong>
                <span>{item.path}</span>
              </div>
            ))}
          </div>
        </div>
      </section>

      {error ? <p className="error-banner">{error}</p> : null}
      {workspaceContent}
    </div>
  );
}
