import { FormEvent, useEffect, useMemo, useState } from "react";
import { formatBps, scoreTone } from "./format";

type Tone = "buy" | "sell" | "flat";
type WorkspaceId = "command" | "listing" | "risk" | "strategy" | "agent";

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
  imports: Array<{ path: string; best_variants: string[] }>;
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
  imports: Array<{ import_id: string; path: string; markets: string[]; best_variants: string[] }>;
};

type WorkspaceTab = {
  id: WorkspaceId;
  label: string;
  kicker: string;
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

export default function App() {
  const [scanner, setScanner] = useState<ScannerRow[]>([]);
  const [orders, setOrders] = useState<WorkstationOrder[]>([]);
  const [strategies, setStrategies] = useState<StrategiesResponse | null>(null);
  const [selectedProduct, setSelectedProduct] = useState<string>("");
  const [detail, setDetail] = useState<ProductDetail | null>(INITIAL_DETAIL);
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
        const [scannerRows, strategyState, orderRows] = await Promise.all([
          getJson<ScannerRow[]>("/api/v1/scanner"),
          getJson<StrategiesResponse>("/api/v1/strategies"),
          getJson<WorkstationOrder[]>("/api/v1/orders"),
        ]);
        if (cancelled) {
          return;
        }
        setScanner(scannerRows);
        setStrategies(strategyState);
        setOrders(orderRows);
        setModeDraft(strategyState.mode);
        if (!selectedProduct && scannerRows.length > 0) {
          setSelectedProduct(scannerRows[0].product_id);
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
      return;
    }
    let cancelled = false;
    getJson<ProductDetail>(`/api/v1/products/${encodeURIComponent(selectedProduct)}`)
      .then((payload) => {
        if (!cancelled) {
          setDetail(payload);
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

  const selection = useMemo(() => {
    return scanner.find((row) => row.product_id === selectedProduct) ?? null;
  }, [scanner, selectedProduct]);

  const blockedCount = useMemo(
    () => scanner.filter((row) => !row.current_risk_eligibility.eligible).length,
    [scanner],
  );
  const liveEligibleCount = useMemo(
    () =>
      scanner.filter(
        (row) => row.live_tradable && row.current_risk_eligibility.eligible && !row.scan_only,
      ).length,
    [scanner],
  );
  const takerOrders = useMemo(
    () => orders.filter((order) => order.route === "taker").length,
    [orders],
  );
  const liveOrders = useMemo(() => orders.filter((order) => order.live).length, [orders]);
  const totalQueuedNotional = useMemo(
    () => orders.reduce((sum, order) => sum + order.quote_notional, 0),
    [orders],
  );

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
              <h2>{detail?.product.product_id ?? topRow?.product_id ?? "Awaiting catalyst"}</h2>
              <p className="muted">
                The listing radar blends scanner score, route readiness, strategy imports, and
                policy eligibility into a single pre-launch research surface.
              </p>
            </div>
            <div className="spotlight-stats">
              <div className="vector-card">
                <span>Composite score</span>
                <strong>{detail?.strategy.composite_score.toFixed(3) ?? "--"}</strong>
              </div>
              <div className="vector-card">
                <span>Priority fill</span>
                <strong>{detail?.strategy.priority_fill ? "Yes" : "No"}</strong>
              </div>
              <div className="vector-card">
                <span>Imports</span>
                <strong>{detail?.imports.length ?? 0}</strong>
              </div>
            </div>
          </div>

          <div className="workspace-grid tertiary">
            <div className="panel">
              <div className="panel-title">
                <h2>Listing Readiness Board</h2>
                <span>{scanner.length} products watched</span>
              </div>
              <div className="candidate-grid">
                {scanner.slice(0, 6).map((row) => (
                  <article key={row.product_id} className="candidate-card">
                    <header>
                      <strong>{row.product_id}</strong>
                      <span className={`score-pill ${scoreTone(row.score)}`}>
                        {row.score.toFixed(3)}
                      </span>
                    </header>
                    <p>
                      {row.scan_only
                        ? "Scan-only until venue trading state and policy gates advance."
                        : "Tradable surface available once strategy and risk gates agree."}
                    </p>
                    <div className="candidate-metrics">
                      <span>{formatBps(row.spread_bps)} spread</span>
                      <span>{formatPercent(row.fill_rate_estimate)} fill</span>
                      <span>imb {row.imbalance.toFixed(2)}</span>
                    </div>
                  </article>
                ))}
              </div>
            </div>

            <div className="panel">
              <div className="panel-title">
                <h2>Provider Lanes</h2>
                <span>Best-in-class targets</span>
              </div>
              <div className="provider-grid">
                {[
                  ["Coinbase/CDP", "listing states, custody-ready execution, wallet and agent primitives"],
                  ["Dune + DeFiLlama", "onchain demand, TVL, fee and wallet growth context"],
                  ["0x + Jupiter", "pre-listing route discovery and DEX execution simulation"],
                  ["TradingView", "chart overlays, Pine alerts, operator-facing context"],
                ].map(([name, detailLine]) => (
                  <div key={name} className="provider-card">
                    <strong>{name}</strong>
                    <p>{detailLine}</p>
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
                <span>Always-on</span>
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
                <h2>Policy Reasons</h2>
                <span>{selection?.product_id ?? "Scanner-driven"}</span>
              </div>
              <div className="reason-stack">
                {(detail?.eligibility.reasons.length ?? 0) > 0 ? (
                  detail?.eligibility.reasons.map((reason) => (
                    <div key={reason} className="reason-card">
                      <strong>Policy hold</strong>
                      <p>{reason}</p>
                    </div>
                  ))
                ) : (
                  <div className="reason-card good">
                    <strong>Eligible</strong>
                    <p>Current selection is clear for automated routing under the present policy.</p>
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
                <h2>Strategy Deployment Map</h2>
                <span>{strategies?.strategies.length ?? 0}</span>
              </div>
              <div className="strategy-list dense">
                {(strategies?.strategies ?? []).map((strategy) => (
                  <div key={strategy.product_id} className="strategy-card">
                    <strong>{strategy.product_id}</strong>
                    <span>{strategy.strategy_name}</span>
                    <small>
                      threshold {strategy.score_threshold.toFixed(2)} / $
                      {strategy.quote_size_usd.toFixed(0)}
                    </small>
                  </div>
                ))}
              </div>
            </div>
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
              <span>{strategies?.mode ?? "paper"}</span>
            </div>
            <div className="agent-grid">
              <div className="agent-card">
                <span>Autonomy</span>
                <strong>{liveArm?.armed ? "Bounded execute" : "Recommend only"}</strong>
                <p>
                  Agent authority should remain capped by live arm state, strategy thresholds, and
                  explicit route policy.
                </p>
              </div>
              <div className="agent-card">
                <span>Next best action</span>
                <strong>
                  {blockedCount > 0 ? "Review blocked markets" : "Advance replay candidates"}
                </strong>
                <p>
                  Use the scanner and strategy imports to choose what deserves replay promotion
                  before any live escalation.
                </p>
              </div>
              <div className="agent-card">
                <span>Evidence set</span>
                <strong>{strategies?.imports.length ?? 0} strategy imports</strong>
                <p>Every agent recommendation should point back to replay, policy, or order evidence.</p>
              </div>
            </div>
          </div>

          <div className="panel">
            <div className="panel-title">
              <h2>Approval Queue</h2>
              <span>Human in the loop</span>
            </div>
            <div className="reason-stack">
              {[
                "Enable taker routing for a specific strategy only after replay and paper evidence agree.",
                "Approve new listing-radar provider adapters before they can influence live risk budgets.",
                "Require human review before any capital moves onto a new chain or venue route.",
              ].map((item) => (
                <div key={item} className="reason-card">
                  <strong>Approval required</strong>
                  <p>{item}</p>
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

                <div className="vector-grid">
                  <div className="vector-card">
                    <span>Microstructure</span>
                    <strong>{detail.strategy.microstructure_score.toFixed(3)}</strong>
                  </div>
                  <div className="vector-card">
                    <span>Momentum</span>
                    <strong>{detail.strategy.momentum_score.toFixed(3)}</strong>
                  </div>
                  <div className="vector-card">
                    <span>Volatility</span>
                    <strong>{detail.strategy.volatility_score.toFixed(3)}</strong>
                  </div>
                  <div className="vector-card">
                    <span>Plugin</span>
                    <strong>{detail.strategy.plugin_score.toFixed(3)}</strong>
                  </div>
                </div>

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

                <div className="eligibility">
                  <h3>Eligibility</h3>
                  <ul>
                    {detail.eligibility.reasons.length === 0 ? (
                      <li>Eligible for automated routing.</li>
                    ) : (
                      detail.eligibility.reasons.map((reason) => <li key={reason}>{reason}</li>)
                    )}
                  </ul>
                </div>
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
                    threshold {strategy.score_threshold.toFixed(2)} / $
                    {strategy.quote_size_usd.toFixed(0)}
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
          strategy validation, and agent governance while continuing to run on the current API
          surface.
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
            <button onClick={() => void postJson("/api/v1/mode", { mode: modeDraft })}>
              Apply Mode
            </button>
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
            <button onClick={() => void postJson("/api/v1/strategy-lab/import", { path: importPath })}>
              Import
            </button>
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
