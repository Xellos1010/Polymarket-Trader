use axum::{
    extract::{Query, State},
    routing::{get, post},
    Json, Router,
};
use parking_lot::RwLock;
use pt_core::{
    Asset, ExecutionReport, KillSwitchState, MarketHistoryPoint, MarketSnapshot, MetricsRegistry,
    RiskState,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
pub struct DashboardState {
    pub metrics: Arc<MetricsRegistry>,
    pub risk_state: Arc<RwLock<RiskState>>,
    pub kill_switch: Arc<RwLock<KillSwitchState>>,
    pub latest_books: Arc<RwLock<HashMap<String, MarketSnapshot>>>,
    pub market_history: Arc<RwLock<HashMap<String, Vec<MarketHistoryPoint>>>>,
    pub recent_executions: Arc<RwLock<Vec<ExecutionReport>>>,
    pub fused_bias: Arc<RwLock<HashMap<Asset, f64>>>,
    pub inventory_usd: Arc<RwLock<f64>>,
}

impl DashboardState {
    pub fn new(
        metrics: Arc<MetricsRegistry>,
        risk_state: Arc<RwLock<RiskState>>,
        kill_switch: Arc<RwLock<KillSwitchState>>,
        latest_books: Arc<RwLock<HashMap<String, MarketSnapshot>>>,
        market_history: Arc<RwLock<HashMap<String, Vec<MarketHistoryPoint>>>>,
        recent_executions: Arc<RwLock<Vec<ExecutionReport>>>,
        fused_bias: Arc<RwLock<HashMap<Asset, f64>>>,
        inventory_usd: Arc<RwLock<f64>>,
    ) -> Self {
        Self {
            metrics,
            risk_state,
            kill_switch,
            latest_books,
            market_history,
            recent_executions,
            fused_bias,
            inventory_usd,
        }
    }
}

#[derive(Debug, Serialize)]
struct Health {
    status: &'static str,
    kill_switch: String,
}

#[derive(Debug, Clone, Serialize)]
struct BiasView {
    asset: String,
    bias: f64,
}

#[derive(Debug, Clone, Serialize)]
struct InventoryView {
    inventory_usd: f64,
}

#[derive(Debug, Clone, Serialize)]
struct MarketView {
    market_id: String,
    token_id: String,
    bid: f64,
    ask: f64,
    spread: f64,
    mid: f64,
    ts: String,
}

#[derive(Debug, Clone, Serialize)]
struct MarketHistoryResponse {
    market_id: Option<String>,
    points: Vec<MarketHistoryPoint>,
}

#[derive(Debug, Clone, Deserialize)]
struct HistoryQuery {
    market_id: Option<String>,
    limit: Option<usize>,
}

pub fn router(state: DashboardState) -> Router {
    Router::new()
        .route("/", get(get_dashboard))
        .route("/health", get(get_health))
        .route("/healthz", get(get_health))
        .route("/ready", get(get_health))
        .route("/metrics", get(get_metrics))
        .route("/state/risk", get(get_risk_state))
        .route("/state/books", get(get_books))
        .route("/state/markets", get(get_markets))
        .route("/state/history", get(get_market_history))
        .route("/state/executions", get(get_executions))
        .route("/state/bias", get(get_bias))
        .route("/state/inventory", get(get_inventory))
        .route("/ops/halt", post(post_halt))
        .route("/ops/resume", post(post_resume))
        .route("/ops/flatten", post(post_flatten))
        .with_state(state)
}

async fn get_dashboard() -> &'static str {
    DASHBOARD_HTML
}

async fn get_health(State(state): State<DashboardState>) -> Json<Health> {
    let k = format!("{:?}", *state.kill_switch.read());
    Json(Health {
        status: "ok",
        kill_switch: k,
    })
}

async fn get_metrics(State(state): State<DashboardState>) -> String {
    state.metrics.render_prometheus()
}

async fn get_risk_state(State(state): State<DashboardState>) -> Json<RiskState> {
    Json(state.risk_state.read().clone())
}

async fn get_books(State(state): State<DashboardState>) -> Json<Vec<MarketSnapshot>> {
    let mut books: Vec<MarketSnapshot> = state.latest_books.read().values().cloned().collect();
    books.sort_by(|a, b| b.ts.cmp(&a.ts));
    Json(books)
}

async fn get_markets(State(state): State<DashboardState>) -> Json<Vec<MarketView>> {
    let mut rows: Vec<MarketView> = state
        .latest_books
        .read()
        .values()
        .map(|b| MarketView {
            market_id: b.market_id.clone(),
            token_id: b.token_id.clone(),
            bid: b.bid,
            ask: b.ask,
            spread: b.spread,
            mid: (b.bid + b.ask) / 2.0,
            ts: b.ts.to_rfc3339(),
        })
        .collect();

    rows.sort_by(|a, b| b.ts.cmp(&a.ts));
    Json(rows)
}

async fn get_market_history(
    State(state): State<DashboardState>,
    Query(query): Query<HistoryQuery>,
) -> Json<MarketHistoryResponse> {
    let limit = query.limit.unwrap_or(240).clamp(10, 1200);

    let selected_market = query
        .market_id
        .filter(|m| !m.trim().is_empty())
        .or_else(|| {
            let mut books: Vec<MarketSnapshot> =
                state.latest_books.read().values().cloned().collect();
            books.sort_by(|a, b| b.ts.cmp(&a.ts));
            books.first().map(|b| b.market_id.clone())
        });

    let mut points = selected_market
        .as_ref()
        .and_then(|market_id| state.market_history.read().get(market_id).cloned())
        .unwrap_or_default();

    if points.len() > limit {
        let keep_from = points.len() - limit;
        points = points.split_off(keep_from);
    }

    Json(MarketHistoryResponse {
        market_id: selected_market,
        points,
    })
}

async fn get_executions(State(state): State<DashboardState>) -> Json<Vec<ExecutionReport>> {
    Json(state.recent_executions.read().clone())
}

async fn get_bias(State(state): State<DashboardState>) -> Json<Vec<BiasView>> {
    let mut rows: Vec<BiasView> = state
        .fused_bias
        .read()
        .iter()
        .map(|(asset, bias)| BiasView {
            asset: asset.as_str().to_string(),
            bias: *bias,
        })
        .collect();
    rows.sort_by(|a, b| a.asset.cmp(&b.asset));
    Json(rows)
}

async fn get_inventory(State(state): State<DashboardState>) -> Json<InventoryView> {
    Json(InventoryView {
        inventory_usd: *state.inventory_usd.read(),
    })
}

async fn post_halt(State(state): State<DashboardState>) -> Json<Health> {
    *state.kill_switch.write() = KillSwitchState::ManualHalt;
    Json(Health {
        status: "ok",
        kill_switch: "ManualHalt".to_string(),
    })
}

async fn post_resume(State(state): State<DashboardState>) -> Json<Health> {
    *state.kill_switch.write() = KillSwitchState::Running;
    Json(Health {
        status: "ok",
        kill_switch: "Running".to_string(),
    })
}

async fn post_flatten(State(state): State<DashboardState>) -> Json<Health> {
    *state.kill_switch.write() = KillSwitchState::SafeMode;
    Json(Health {
        status: "ok",
        kill_switch: "SafeMode".to_string(),
    })
}

const DASHBOARD_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width,initial-scale=1" />
  <title>Polymarket Trader Dashboard</title>
  <style>
    :root {
      --bg: #0f172a;
      --panel: #111827;
      --panel2: #1f2937;
      --text: #e5e7eb;
      --muted: #94a3b8;
      --buy: #10b981;
      --sell: #ef4444;
      --warn: #f59e0b;
      --accent: #38bdf8;
    }
    * { box-sizing: border-box; }
    body {
      margin: 0;
      font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
      color: var(--text);
      background: radial-gradient(1200px 800px at 20% -10%, #1e293b, var(--bg));
    }
    .wrap { max-width: 1280px; margin: 0 auto; padding: 16px; }
    .title {
      display: flex; justify-content: space-between; align-items: center;
      gap: 12px; margin-bottom: 16px;
    }
    .title h1 { margin: 0; font-size: 20px; letter-spacing: 0.5px; color: var(--accent); }
    .status { color: var(--muted); font-size: 12px; }
    .grid {
      display: grid;
      grid-template-columns: repeat(12, 1fr);
      gap: 12px;
    }
    .card {
      background: linear-gradient(180deg, var(--panel), var(--panel2));
      border: 1px solid #243042;
      border-radius: 10px;
      padding: 12px;
      box-shadow: 0 8px 24px rgba(0,0,0,0.25);
    }
    .kpis { grid-column: span 12; display: grid; grid-template-columns: repeat(6, 1fr); gap: 8px; }
    .kpi { background: #0b1220; border: 1px solid #1e293b; border-radius: 8px; padding: 8px; }
    .kpi .label { color: var(--muted); font-size: 11px; }
    .kpi .value { font-size: 16px; margin-top: 4px; }
    .chart { grid-column: span 6; }
    .controls { grid-column: span 12; display: flex; gap: 8px; align-items: center; flex-wrap: wrap; }
    button {
      background: #0b1220; color: var(--text); border: 1px solid #334155;
      border-radius: 8px; padding: 8px 12px; cursor: pointer; font-size: 12px;
    }
    button:hover { border-color: var(--accent); }
    select {
      background: #0b1220;
      color: var(--text);
      border: 1px solid #334155;
      border-radius: 8px;
      padding: 8px 12px;
      font-size: 12px;
      min-width: 340px;
      max-width: 100%;
    }
    .table-card { grid-column: span 6; }
    table { width: 100%; border-collapse: collapse; font-size: 12px; }
    th, td { text-align: left; padding: 6px; border-bottom: 1px solid #1e293b; }
    th { color: var(--muted); position: sticky; top: 0; background: #0f172a; }
    .scroll { max-height: 360px; overflow: auto; }
    .buy { color: var(--buy); }
    .sell { color: var(--sell); }
    .warn { color: var(--warn); }
    .tiny { color: var(--muted); font-size: 11px; }
    canvas {
      width: 100%;
      height: 200px;
      background: #0b1220;
      border: 1px solid #1e293b;
      border-radius: 8px;
    }
    @media (max-width: 960px) {
      .kpis { grid-template-columns: repeat(2, 1fr); }
      .chart, .controls, .table-card { grid-column: span 12; }
      select { min-width: 100%; }
    }
  </style>
</head>
<body>
  <div class="wrap">
    <div class="title">
      <h1>Polymarket Trader</h1>
      <div class="status" id="status">Loading...</div>
    </div>
    <div class="grid">
      <div class="kpis">
        <div class="kpi"><div class="label">Kill Switch</div><div class="value" id="k_kill">-</div></div>
        <div class="kpi"><div class="label">Daily PnL</div><div class="value" id="k_pnl">-</div></div>
        <div class="kpi"><div class="label">Open Notional</div><div class="value" id="k_open">-</div></div>
        <div class="kpi"><div class="label">Unhedged Delta</div><div class="value" id="k_delta">-</div></div>
        <div class="kpi"><div class="label">Open Markets</div><div class="value" id="k_markets">-</div></div>
        <div class="kpi"><div class="label">Inventory USD</div><div class="value" id="k_inv">-</div></div>
      </div>

      <div class="card chart">
        <div class="tiny">Daily PnL (rolling)</div>
        <canvas id="pnlChart" width="640" height="220"></canvas>
      </div>

      <div class="card chart">
        <div class="tiny">Selected Market Mid-Price (rolling)</div>
        <canvas id="marketChart" width="640" height="220"></canvas>
        <div class="tiny" id="marketMeta">No market selected</div>
      </div>

      <div class="card controls">
        <button onclick="op('/ops/halt')">HALT</button>
        <button onclick="op('/ops/resume')">RESUME</button>
        <button onclick="op('/ops/flatten')">FLATTEN</button>
        <label class="tiny" for="marketSelect">Market</label>
        <select id="marketSelect"></select>
        <div class="tiny" id="opsResult"></div>
      </div>

      <div class="card table-card">
        <div class="tiny">Current Books</div>
        <div class="scroll">
          <table>
            <thead><tr><th>Market</th><th>Bid</th><th>Ask</th><th>Spread</th><th>TS</th></tr></thead>
            <tbody id="booksBody"></tbody>
          </table>
        </div>
      </div>

      <div class="card table-card">
        <div class="tiny">Recent Executions</div>
        <div class="scroll">
          <table>
            <thead><tr><th>TS</th><th>Venue</th><th>Status</th><th>Side</th><th>Qty</th><th>Px</th></tr></thead>
            <tbody id="execBody"></tbody>
          </table>
        </div>
      </div>

      <div class="card table-card" style="grid-column: span 12;">
        <div class="tiny">Asset Bias</div>
        <div class="scroll">
          <table>
            <thead><tr><th>Asset</th><th>Bias</th></tr></thead>
            <tbody id="biasBody"></tbody>
          </table>
        </div>
      </div>
    </div>
  </div>

  <script>
    const pnlSeries = [];
    let selectedMarketId = null;
    let marketSignature = '';

    function fmtNum(n) {
      const v = Number(n || 0);
      if (!Number.isFinite(v)) return '-';
      return v.toFixed(4);
    }

    function drawSeries(canvasId, series, color) {
      const canvas = document.getElementById(canvasId);
      const ctx = canvas.getContext('2d');
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      ctx.strokeStyle = '#334155';
      ctx.lineWidth = 1;
      for (let i = 0; i < 5; i++) {
        const y = 20 + i * 45;
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(canvas.width, y);
        ctx.stroke();
      }

      if (series.length < 2) return;

      const min = Math.min(...series);
      const max = Math.max(...series);
      const span = (max - min) || 1;

      ctx.strokeStyle = color;
      ctx.lineWidth = 2;
      ctx.beginPath();
      series.forEach((v, i) => {
        const x = (i / (series.length - 1)) * (canvas.width - 20) + 10;
        const y = canvas.height - 15 - ((v - min) / span) * (canvas.height - 30);
        if (i === 0) {
          ctx.moveTo(x, y);
        } else {
          ctx.lineTo(x, y);
        }
      });
      ctx.stroke();
    }

    function syncMarketSelect(markets) {
      const select = document.getElementById('marketSelect');
      const ids = markets.map(x => x.market_id);
      const signature = ids.join('|');

      if (!selectedMarketId || !ids.includes(selectedMarketId)) {
        selectedMarketId = ids.length > 0 ? ids[0] : null;
      }

      if (signature !== marketSignature) {
        marketSignature = signature;
        select.innerHTML = '';
        ids.forEach(id => {
          const opt = document.createElement('option');
          opt.value = id;
          opt.textContent = id;
          select.appendChild(opt);
        });
      }

      if (selectedMarketId) {
        select.value = selectedMarketId;
      }
    }

    function drawMarket(points) {
      const mids = points.map(p => Number(p.mid || 0));
      drawSeries('marketChart', mids, '#10b981');
      const meta = document.getElementById('marketMeta');
      if (points.length === 0) {
        meta.textContent = 'No market history points available yet';
        return;
      }

      const last = points[points.length - 1];
      const marketId = selectedMarketId || last.market_id;
      meta.textContent = `${marketId} mid=${fmtNum(last.mid)} spread=${fmtNum(last.spread)} ts=${new Date(last.ts).toLocaleTimeString()}`;
    }

    async function op(path) {
      try {
        const r = await fetch(path, { method: 'POST' });
        document.getElementById('opsResult').textContent = await r.text();
      } catch (e) {
        document.getElementById('opsResult').textContent = String(e);
      }
    }

    async function tick() {
      try {
        const [h, r, b, e, bias, inv, markets] = await Promise.all([
          fetch('/health').then(x => x.json()),
          fetch('/state/risk').then(x => x.json()),
          fetch('/state/books').then(x => x.json()),
          fetch('/state/executions').then(x => x.json()),
          fetch('/state/bias').then(x => x.json()),
          fetch('/state/inventory').then(x => x.json()),
          fetch('/state/markets').then(x => x.json()),
        ]);

        syncMarketSelect(markets);

        let historyUrl = '/state/history?limit=360';
        if (selectedMarketId) {
          historyUrl += `&market_id=${encodeURIComponent(selectedMarketId)}`;
        }
        const history = await fetch(historyUrl).then(x => x.json());
        if (history.market_id && history.market_id !== selectedMarketId) {
          selectedMarketId = history.market_id;
          syncMarketSelect(markets);
        }

        document.getElementById('status').textContent = `Updated ${new Date().toLocaleTimeString()}`;
        document.getElementById('k_kill').textContent = h.kill_switch;
        document.getElementById('k_pnl').textContent = fmtNum(r.daily_pnl);
        document.getElementById('k_open').textContent = fmtNum(r.open_notional);
        document.getElementById('k_delta').textContent = fmtNum(r.unhedged_delta);
        document.getElementById('k_markets').textContent = r.open_markets;
        document.getElementById('k_inv').textContent = fmtNum(inv.inventory_usd);

        pnlSeries.push(Number(r.daily_pnl || 0));
        if (pnlSeries.length > 240) pnlSeries.shift();
        drawSeries('pnlChart', pnlSeries, '#38bdf8');
        drawMarket(history.points || []);

        const booksBody = document.getElementById('booksBody');
        booksBody.innerHTML = b.slice(0, 120).map(x =>
          `<tr><td>${x.market_id}</td><td>${fmtNum(x.bid)}</td><td>${fmtNum(x.ask)}</td><td>${fmtNum(x.spread)}</td><td>${new Date(x.ts).toLocaleTimeString()}</td></tr>`
        ).join('');

        const execBody = document.getElementById('execBody');
        execBody.innerHTML = e.slice(0, 160).map(x => {
          const sideClass = x.side === 'Buy' ? 'buy' : 'sell';
          const statusClass = (x.status === 'Rejected' || x.status === 'Error') ? 'warn' : '';
          return `<tr><td>${new Date(x.ts).toLocaleTimeString()}</td><td>${x.venue}</td><td class="${statusClass}">${x.status}</td><td class="${sideClass}">${x.side}</td><td>${fmtNum(x.filled_qty)}</td><td>${fmtNum(x.avg_px)}</td></tr>`;
        }).join('');

        const biasBody = document.getElementById('biasBody');
        biasBody.innerHTML = bias.map(x => `<tr><td>${x.asset}</td><td>${fmtNum(x.bias)}</td></tr>`).join('');
      } catch (err) {
        document.getElementById('status').textContent = `Error: ${err}`;
      }
    }

    document.getElementById('marketSelect').addEventListener('change', (ev) => {
      selectedMarketId = ev.target.value || null;
    });

    tick();
    setInterval(tick, 1000);
  </script>
</body>
</html>
"#;
