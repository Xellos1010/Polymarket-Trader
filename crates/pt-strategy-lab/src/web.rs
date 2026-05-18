use crate::backtest::run_backtest;
use crate::data::fetch_coinbase_candles;
use crate::persistence::{list_runs, load_profile, save_profile, save_run, save_run_manifest};
use crate::tuning::optimize_random_walk_forward;
use crate::types::{FusionDecision, StrategyProfile, StrategyRunReport, TuningReport};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct StrategyLabState {
    pub db_path: String,
    pub profile: Arc<RwLock<StrategyProfile>>,
    pub last_run: Arc<RwLock<Option<StrategyRunReport>>>,
    pub last_tuning: Arc<RwLock<Option<TuningReport>>>,
}

impl StrategyLabState {
    pub fn new(db_path: impl Into<String>, profile: StrategyProfile) -> Self {
        Self {
            db_path: db_path.into(),
            profile: Arc::new(RwLock::new(profile)),
            last_run: Arc::new(RwLock::new(None)),
            last_tuning: Arc::new(RwLock::new(None)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestRunRequest {
    pub product_id: Option<String>,
    pub granularity_sec: Option<u32>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizeRequest {
    pub iterations: Option<usize>,
    pub walk_forward_splits: Option<usize>,
    pub seed: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveProfileRequest {
    pub profile: StrategyProfile,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadProfileRequest {
    pub profile_id: String,
}

pub fn router(state: StrategyLabState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/lab/state/profile", get(get_profile))
        .route("/lab/profile/save", post(save_profile_handler))
        .route("/lab/profile/load", post(load_profile_handler))
        .route("/lab/backtest/run", post(run_backtest_handler))
        .route("/lab/optimize/run", post(run_optimize_handler))
        .route("/lab/state/indicators", get(get_indicators))
        .route("/lab/state/signals", get(get_signals))
        .route("/lab/state/regime", get(get_regime))
        .route("/lab/state/runs", get(get_runs))
        .with_state(state)
}

pub async fn serve(state: StrategyLabState, bind_addr: &str) -> Result<(), String> {
    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .map_err(|e| format!("failed to bind strategy lab {}: {}", bind_addr, e))?;
    axum::serve(listener, router(state))
        .await
        .map_err(|e| format!("strategy lab server error: {}", e))
}

async fn index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

async fn get_profile(State(state): State<StrategyLabState>) -> impl IntoResponse {
    Json(state.profile.read().clone())
}

async fn save_profile_handler(
    State(state): State<StrategyLabState>,
    Json(req): Json<SaveProfileRequest>,
) -> impl IntoResponse {
    let mut next = req.profile.clone();
    next.version = next.version.saturating_add(1);
    match save_profile(&state.db_path, &next, req.note.as_deref()) {
        Ok(_) => {
            *state.profile.write() = next.clone();
            Json(serde_json::json!({"ok": true, "profile": next}))
        }
        Err(e) => Json(serde_json::json!({"ok": false, "error": e.to_string()})),
    }
}

async fn load_profile_handler(
    State(state): State<StrategyLabState>,
    Json(req): Json<LoadProfileRequest>,
) -> impl IntoResponse {
    match load_profile(&state.db_path, &req.profile_id) {
        Ok(profile) => {
            *state.profile.write() = profile.clone();
            Json(serde_json::json!({"ok": true, "profile": profile}))
        }
        Err(e) => Json(serde_json::json!({"ok": false, "error": e.to_string()})),
    }
}

async fn run_backtest_handler(
    State(state): State<StrategyLabState>,
    Json(req): Json<BacktestRunRequest>,
) -> impl IntoResponse {
    let mut profile = state.profile.read().clone();
    if let Some(v) = req.product_id {
        profile.product_id = v;
    }
    if let Some(v) = req.granularity_sec {
        profile.granularity_sec = v;
    }
    if let Some(v) = req.limit {
        profile.candle_limit = v;
    }

    match fetch_coinbase_candles(
        &profile.product_id,
        profile.granularity_sec,
        profile.candle_limit,
    )
    .await
    {
        Ok(candles) => {
            if candles.len() < 50 {
                return Json(serde_json::json!({
                    "ok": false,
                    "error": "insufficient candles; need at least 50"
                }));
            }
            let report = run_backtest(&profile, &candles);
            if let Err(e) = save_run(&state.db_path, &report) {
                return Json(serde_json::json!({"ok": false, "error": e.to_string()}));
            }
            save_run_manifest(&report);
            *state.last_run.write() = Some(report.clone());
            Json(serde_json::json!({"ok": true, "report": report}))
        }
        Err(e) => Json(serde_json::json!({"ok": false, "error": e.to_string()})),
    }
}

async fn run_optimize_handler(
    State(state): State<StrategyLabState>,
    Json(req): Json<OptimizeRequest>,
) -> impl IntoResponse {
    let profile = state.profile.read().clone();
    match fetch_coinbase_candles(
        &profile.product_id,
        profile.granularity_sec,
        profile.candle_limit,
    )
    .await
    {
        Ok(candles) => {
            if candles.len() < 120 {
                return Json(serde_json::json!({
                    "ok": false,
                    "error": "insufficient candles; need at least 120 for optimization"
                }));
            }
            let tuning = optimize_random_walk_forward(
                &profile,
                &candles,
                req.iterations.unwrap_or(200),
                req.walk_forward_splits.unwrap_or(4),
                req.seed.unwrap_or(42),
            );
            *state.last_tuning.write() = Some(tuning.clone());
            Json(serde_json::json!({"ok": true, "tuning": tuning}))
        }
        Err(e) => Json(serde_json::json!({"ok": false, "error": e.to_string()})),
    }
}

async fn get_indicators(State(state): State<StrategyLabState>) -> impl IntoResponse {
    let indicators = state
        .last_run
        .read()
        .as_ref()
        .map(|run| {
            run.decisions
                .iter()
                .flat_map(|d| {
                    d.indicators.iter().map(move |i| {
                        serde_json::json!({
                            "ts_ms": d.ts_ms,
                            "name": i.name,
                            "bias": i.bias,
                            "confidence": i.confidence,
                            "regime_vote": format!("{:?}", i.regime_vote).to_lowercase(),
                        })
                    })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Json(serde_json::json!({"rows": indicators}))
}

async fn get_signals(State(state): State<StrategyLabState>) -> impl IntoResponse {
    let signals: Vec<FusionDecision> = state
        .last_run
        .read()
        .as_ref()
        .map(|run| run.decisions.clone())
        .unwrap_or_default();
    Json(serde_json::json!({"rows": signals}))
}

async fn get_regime(State(state): State<StrategyLabState>) -> impl IntoResponse {
    let rows = state
        .last_run
        .read()
        .as_ref()
        .map(|run| {
            run.decisions
                .iter()
                .map(|d| {
                    serde_json::json!({
                        "ts_ms": d.ts_ms,
                        "regime": format!("{:?}", d.regime).to_lowercase(),
                        "score": d.score,
                    })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Json(serde_json::json!({"rows": rows}))
}

async fn get_runs(State(state): State<StrategyLabState>) -> impl IntoResponse {
    match list_runs(&state.db_path, 20) {
        Ok(runs) => Json(serde_json::json!({"rows": runs})),
        Err(e) => Json(serde_json::json!({"rows": [], "error": e.to_string()})),
    }
}

const INDEX_HTML: &str = r#"<!doctype html>
<html lang=\"en\">
<head>
<meta charset=\"utf-8\" />
<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\" />
<title>Rust Strategy Lab</title>
<script src=\"https://cdn.jsdelivr.net/npm/chart.js@4.4.4/dist/chart.umd.min.js\"></script>
<style>
body { font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; background: #0b1220; color: #d9e4ff; margin: 0; }
header { padding: 16px 20px; border-bottom: 1px solid #24324f; display:flex; gap:12px; align-items:center; }
button,input { background:#111b2f; color:#d9e4ff; border:1px solid #24324f; padding:8px 10px; }
main { display:grid; grid-template-columns: 1fr; gap: 12px; padding: 12px; }
.panel { border:1px solid #24324f; background:#111b2f; padding:12px; }
canvas { width: 100%; height: 320px; }
.row { display:flex; gap:8px; align-items:center; flex-wrap:wrap; }
.small { font-size: 12px; color: #9cb3e6; }
</style>
</head>
<body>
<header>
  <strong>Rust Strategy Lab</strong>
  <div class=\"row\">
    <input id=\"product\" value=\"BTC-USD\" />
    <input id=\"limit\" value=\"600\" size=\"6\" />
    <button id=\"run\">Run Backtest</button>
    <button id=\"opt\">Optimize</button>
  </div>
</header>
<main>
  <div class=\"panel\">
    <div class=\"small\" id=\"stats\">No run loaded.</div>
  </div>
  <div class=\"panel\"><canvas id=\"priceChart\"></canvas></div>
  <div class=\"panel\"><canvas id=\"scoreChart\"></canvas></div>
  <div class=\"panel\"><canvas id=\"equityChart\"></canvas></div>
</main>
<script>
let priceChart, scoreChart, equityChart;
function mkChart(id, datasets) {
  const ctx = document.getElementById(id).getContext('2d');
  return new Chart(ctx, { type:'line', data:{ labels:[], datasets }, options:{ animation:false, responsive:true, maintainAspectRatio:false, plugins:{legend:{labels:{color:'#c6d8ff'}}}, scales:{x:{ticks:{color:'#9cb3e6'}}, y:{ticks:{color:'#9cb3e6'}}} } });
}
function ensureCharts() {
  if (!priceChart) priceChart = mkChart('priceChart', [
    { label:'Close', borderColor:'#6fb4ff', pointRadius:0, data:[] },
    { label:'Buy', borderColor:'#2ecc71', showLine:false, pointRadius:3, data:[] },
    { label:'Sell', borderColor:'#ff5a6f', showLine:false, pointRadius:3, data:[] },
  ]);
  if (!scoreChart) scoreChart = mkChart('scoreChart', [
    { label:'Score', borderColor:'#ffd166', pointRadius:0, data:[] },
    { label:'Buy Th', borderColor:'#2ecc71', borderDash:[4,4], pointRadius:0, data:[] },
    { label:'Sell Th', borderColor:'#ff5a6f', borderDash:[4,4], pointRadius:0, data:[] },
  ]);
  if (!equityChart) equityChart = mkChart('equityChart', [
    { label:'Equity', borderColor:'#c792ea', pointRadius:0, data:[] },
  ]);
}
async function runBacktest() {
  const product = document.getElementById('product').value.trim();
  const limit = Number(document.getElementById('limit').value || 600);
  const res = await fetch('/lab/backtest/run', { method:'POST', headers:{'content-type':'application/json'}, body: JSON.stringify({ product_id: product, limit })});
  const json = await res.json();
  if (!json.ok) {
    document.getElementById('stats').textContent = 'Backtest error: ' + json.error;
    return;
  }
  const report = json.report;
  ensureCharts();
  const labels = report.candles.map(c => new Date(c.ts_ms).toISOString().slice(11,19));
  const closes = report.candles.map(c => c.close);
  const buyMap = new Map(report.fills.filter(f => f.action === 'buy').map(f => [f.ts_ms, f.price]));
  const sellMap = new Map(report.fills.filter(f => f.action === 'sell').map(f => [f.ts_ms, f.price]));
  const buys = report.candles.map(c => buyMap.get(c.ts_ms) ?? null);
  const sells = report.candles.map(c => sellMap.get(c.ts_ms) ?? null);
  priceChart.data.labels = labels;
  priceChart.data.datasets[0].data = closes;
  priceChart.data.datasets[1].data = buys;
  priceChart.data.datasets[2].data = sells;
  priceChart.update();

  const scores = report.decisions.map(d => d.score);
  scoreChart.data.labels = labels;
  scoreChart.data.datasets[0].data = scores;
  scoreChart.data.datasets[1].data = scores.map(() => 0.60);
  scoreChart.data.datasets[2].data = scores.map(() => -0.60);
  scoreChart.update();

  equityChart.data.labels = report.equity_curve.map(p => new Date(p.ts_ms).toISOString().slice(11,19));
  equityChart.data.datasets[0].data = report.equity_curve.map(p => p.equity);
  equityChart.update();

  document.getElementById('stats').textContent = `Run ${report.run_id} | return ${(report.total_return_pct*100).toFixed(2)}% | dd ${(report.max_drawdown_pct*100).toFixed(2)}% | trades ${report.trades} | win ${(report.win_rate*100).toFixed(1)}% | pnl ${report.pnl.toFixed(2)}`;
}
async function runOptimize() {
  const res = await fetch('/lab/optimize/run', { method:'POST', headers:{'content-type':'application/json'}, body: JSON.stringify({ iterations: 200, walk_forward_splits: 4 })});
  const json = await res.json();
  if (!json.ok) {
    document.getElementById('stats').textContent = 'Optimize error: ' + json.error;
    return;
  }
  const best = (json.tuning.top && json.tuning.top[0]) || null;
  if (best) {
    document.getElementById('stats').textContent = `Optimize done. best score ${best.score.toFixed(4)} | return ${(best.report.total_return_pct*100).toFixed(2)}% | dd ${(best.report.max_drawdown_pct*100).toFixed(2)}%`;
  } else {
    document.getElementById('stats').textContent = 'Optimize done. No candidates.';
  }
}
document.getElementById('run').addEventListener('click', runBacktest);
document.getElementById('opt').addEventListener('click', runOptimize);
</script>
</body>
</html>"#;
