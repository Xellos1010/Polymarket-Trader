import { FormEvent, useEffect, useMemo, useState } from "react";
import { formatBps, scoreTone } from "./format";

type Tone = "buy" | "sell" | "flat";

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

const INITIAL_DETAIL: ProductDetail | null = null;

async function getJson<T>(path: string, init?: RequestInit): Promise<T> {
  const response = await fetch(path, init);
  if (!response.ok) {
    throw new Error(`${path} failed with ${response.status}`);
  }
  return response.json() as Promise<T>;
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

  return (
    <div className="app-shell">
      <section className="hero">
        <div>
          <p className="eyebrow">Coinbase-native local workstation</p>
          <h1>Scanner-first entry and exit control.</h1>
          <p className="lede">
            Rank the market, inspect the vector stack, arm live automation, and queue maker or
            taker orders from one surface.
          </p>
        </div>
        <div className={`signal-card tone-${tone}`}>
          <span>Top signal</span>
          <strong>{topRow?.product_id ?? "Waiting for scanner"}</strong>
          <small>{topRow ? `${formatBps(topRow.spread_bps)} spread` : "No market data yet"}</small>
        </div>
      </section>

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
            <button
              onClick={() => void postJson("/api/v1/strategy-lab/import", { path: importPath })}
            >
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
                  threshold {strategy.score_threshold.toFixed(2)} / ${strategy.quote_size_usd.toFixed(0)}
                </small>
              </div>
            ))}
          </div>
        </div>
      </section>
    </div>
  );
}
