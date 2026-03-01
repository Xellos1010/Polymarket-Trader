use clap::{Parser, Subcommand};
use pt_coinbase::{
    CoinbaseAuthManager, CoinbaseWalletClient, CoinbaseWsEvent, CoinbaseWsRunConfig,
};
use pt_core::{ensure_rustls_crypto_provider, AppConfig, EngineMode, ReplayAcceptanceReport};
use pt_engine::TradingEngine;
use pt_strategy_lab::{
    fetch_coinbase_candles, load_profile as load_strategy_profile, optimize_random_walk_forward,
    run_backtest, save_profile, save_run as save_strategy_run, serve as serve_strategy_lab,
    StrategyLabState, StrategyProfile,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    cmp::Ordering,
    collections::HashMap,
    fs,
    net::TcpListener,
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};
use tracing::{error, info};

#[derive(Debug, Parser)]
#[command(name = "pt-cli")]
#[command(about = "Polymarket trader engine CLI")]
struct Cli {
    #[arg(short, long, global = true, default_value = "config/config.toml")]
    config: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Run,
    Status {
        #[arg(short, long, default_value = "http://127.0.0.1:8080/health")]
        url: String,
    },
    WalletStatus {
        #[arg(
            short,
            long,
            default_value = "http://127.0.0.1:8080/state/coinbase/wallet"
        )]
        url: String,
    },
    WalletPlan {
        #[arg(
            short,
            long,
            default_value = "http://127.0.0.1:8080/state/coinbase/rebalance-plan"
        )]
        url: String,
    },
    CoinbaseWsStatus {
        #[arg(
            short,
            long,
            default_value = "http://127.0.0.1:8080/state/coinbase/orderbook"
        )]
        url: String,
    },
    CoinbaseAuthStatus {
        #[arg(
            short,
            long,
            default_value = "http://127.0.0.1:8080/state/coinbase/auth"
        )]
        url: String,
    },
    CoinbaseAuthReload {
        #[arg(
            short,
            long,
            default_value = "http://127.0.0.1:8080/ops/coinbase/auth/reload"
        )]
        url: String,
    },
    CoinbaseAuthSwitch {
        #[arg(long)]
        profile: String,
        #[arg(
            short,
            long,
            default_value = "http://127.0.0.1:8080/ops/coinbase/auth/switch-profile"
        )]
        url: String,
    },
    WalletApprove {
        #[arg(long)]
        token_id: String,
        #[arg(
            short,
            long,
            default_value = "http://127.0.0.1:8080/ops/coinbase/rebalance/approve"
        )]
        url: String,
    },
    ExecutionStatus {
        #[arg(
            short,
            long,
            default_value = "http://127.0.0.1:8080/state/execution/orders"
        )]
        url: String,
    },
    OrderManagerStatus {
        #[arg(
            short,
            long,
            default_value = "http://127.0.0.1:8080/state/execution/orders"
        )]
        url: String,
    },
    RoutesStatus {
        #[arg(
            short,
            long,
            default_value = "http://127.0.0.1:8080/state/routes/opportunities"
        )]
        url: String,
    },
    SetEdgeProfile {
        #[arg(long)]
        strategy: String,
        #[arg(long)]
        min_bps: f64,
        #[arg(long)]
        asset: Option<String>,
    },
    PilotStart {
        #[arg(long, default_value_t = 20.0)]
        capital: f64,
        #[arg(long, default_value = "ultra-tight")]
        profile: String,
        #[arg(long, default_value_t = 3000)]
        timeout_ms: u64,
    },
    VerifyPromoted {
        #[arg(long)]
        artifact: String,
        #[arg(long, default_value = "data/output/replay_acceptance_report.json")]
        out: String,
    },
    ReportVariants {
        #[arg(long, default_value = "data/strategy_lab/trade_journal.sqlite")]
        journal: String,
        #[arg(long, default_value = "data/output/variant_report.csv")]
        out_csv: String,
        #[arg(long, default_value = "data/output/variant_report.md")]
        out_md: String,
    },
    PreflightLive {
        #[arg(long, default_value_t = 3000)]
        timeout_ms: u64,
    },
    CoinbaseSmoke {
        #[arg(long, default_value_t = 3000)]
        timeout_ms: u64,
        #[arg(long, default_value_t = false)]
        write_test: bool,
        #[arg(long, default_value = "")]
        confirm: String,
        #[arg(long, default_value = "data/output/coinbase_smoke_report.json")]
        out: String,
    },
    StrategyLabServe {
        #[arg(long, default_value = "127.0.0.1:9090")]
        bind: String,
        #[arg(long, default_value = "data/strategy_lab/strategy_lab.sqlite")]
        db: String,
        #[arg(long)]
        profile_id: Option<String>,
    },
    StrategyBacktest {
        #[arg(long, default_value = "BTC-USD")]
        product: String,
        #[arg(long, default_value_t = 300)]
        granularity_sec: u32,
        #[arg(long, default_value_t = 600)]
        limit: usize,
        #[arg(long, default_value = "data/strategy_lab/strategy_lab.sqlite")]
        db: String,
        #[arg(long)]
        profile_id: Option<String>,
        #[arg(long, default_value = "data/output/strategy_backtest_report.json")]
        out: String,
    },
    StrategyOptimize {
        #[arg(long, default_value = "BTC-USD")]
        product: String,
        #[arg(long, default_value_t = 300)]
        granularity_sec: u32,
        #[arg(long, default_value_t = 600)]
        limit: usize,
        #[arg(long, default_value_t = 200)]
        iterations: usize,
        #[arg(long, default_value_t = 4)]
        walk_forward_splits: usize,
        #[arg(long, default_value_t = 42)]
        seed: u64,
        #[arg(long, default_value = "data/strategy_lab/strategy_lab.sqlite")]
        db: String,
        #[arg(long)]
        profile_id: Option<String>,
        #[arg(long, default_value = "data/output/strategy_optimize_report.json")]
        out: String,
    },
    StrategyProfileSave {
        #[arg(long)]
        path: String,
        #[arg(long, default_value = "data/strategy_lab/strategy_lab.sqlite")]
        db: String,
        #[arg(long)]
        note: Option<String>,
    },
    StrategyProfileLoad {
        #[arg(long)]
        profile_id: String,
        #[arg(long, default_value = "data/strategy_lab/strategy_lab.sqlite")]
        db: String,
        #[arg(long)]
        out: Option<String>,
    },
    PineParams {
        #[arg(long)]
        path: String,
        #[arg(long, default_value = "data/tuning/pine_params.json")]
        out: String,
    },
    TunePine {
        #[arg(long)]
        path: String,
        #[arg(long, default_value_t = 100)]
        iterations: usize,
        #[arg(long, default_value_t = 10)]
        top_k: usize,
        #[arg(long)]
        evaluate_cmd: Option<String>,
        #[arg(long)]
        seed: Option<u64>,
        #[arg(long, default_value = "data/tuning/pine_tuning_results.json")]
        out: String,
    },
    SaveContext {
        #[arg(long, default_value = "docs/SESSION_CONTEXT.md")]
        out: String,
        #[arg(long)]
        note: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PineInputKind {
    Int,
    Float,
    Bool,
    String,
    Source,
    Generic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PineParam {
    name: String,
    line: usize,
    kind: PineInputKind,
    title: Option<String>,
    default_raw: Option<String>,
    default_num: Option<f64>,
    default_bool: Option<bool>,
    default_string: Option<String>,
    min: Option<f64>,
    max: Option<f64>,
    options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PineTuneCandidate {
    iteration: usize,
    params: HashMap<String, Value>,
    score: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PineTuneReport {
    script_path: String,
    iterations: usize,
    evaluate_cmd: Option<String>,
    scored: bool,
    top_k: usize,
    best: Option<PineTuneCandidate>,
    top_candidates: Vec<PineTuneCandidate>,
}

#[tokio::main]
async fn main() {
    init_tracing();

    let cli = Cli::parse();
    let config_path = resolve_config_path(&cli.config);

    match cli.command {
        Commands::Run => {
            if let Err(e) = run(&config_path).await {
                error!(%e, "engine failed");
                std::process::exit(1);
            }
        }
        Commands::Status { url } => {
            if let Err(e) = status(&url).await {
                error!(%e, "status check failed");
                std::process::exit(1);
            }
        }
        Commands::WalletStatus { url } => {
            if let Err(e) = status(&url).await {
                error!(%e, "wallet status failed");
                std::process::exit(1);
            }
        }
        Commands::WalletPlan { url } => {
            if let Err(e) = status(&url).await {
                error!(%e, "wallet plan failed");
                std::process::exit(1);
            }
        }
        Commands::CoinbaseWsStatus { url } => {
            if let Err(e) = status(&url).await {
                error!(%e, "coinbase ws status failed");
                std::process::exit(1);
            }
        }
        Commands::CoinbaseAuthStatus { url } => {
            if let Err(e) = status(&url).await {
                error!(%e, "coinbase auth status failed");
                std::process::exit(1);
            }
        }
        Commands::CoinbaseAuthReload { url } => {
            if let Err(e) = post_no_body(&url).await {
                error!(%e, "coinbase auth reload failed");
                std::process::exit(1);
            }
        }
        Commands::CoinbaseAuthSwitch { profile, url } => {
            if let Err(e) = coinbase_auth_switch(&url, &profile).await {
                error!(%e, "coinbase auth switch failed");
                std::process::exit(1);
            }
        }
        Commands::WalletApprove { token_id, url } => {
            if let Err(e) = wallet_approve(&url, &token_id).await {
                error!(%e, "wallet approve failed");
                std::process::exit(1);
            }
        }
        Commands::ExecutionStatus { url } => {
            if let Err(e) = status(&url).await {
                error!(%e, "execution status failed");
                std::process::exit(1);
            }
        }
        Commands::OrderManagerStatus { url } => {
            if let Err(e) = status(&url).await {
                error!(%e, "order manager status failed");
                std::process::exit(1);
            }
        }
        Commands::RoutesStatus { url } => {
            if let Err(e) = status(&url).await {
                error!(%e, "routes status failed");
                std::process::exit(1);
            }
        }
        Commands::SetEdgeProfile {
            strategy,
            min_bps,
            asset,
        } => {
            if let Err(e) = set_edge_profile(&config_path, &strategy, min_bps, asset.as_deref()) {
                error!(%e, "set edge profile failed");
                std::process::exit(1);
            }
        }
        Commands::PilotStart {
            capital,
            profile,
            timeout_ms,
        } => {
            if let Err(e) = pilot_start(&config_path, capital, &profile, timeout_ms).await {
                error!(%e, "pilot start failed");
                std::process::exit(1);
            }
        }
        Commands::VerifyPromoted { artifact, out } => {
            if let Err(e) = verify_promoted(&config_path, &artifact, &out).await {
                error!(%e, "verify promoted failed");
                std::process::exit(1);
            }
        }
        Commands::ReportVariants {
            journal,
            out_csv,
            out_md,
        } => {
            if let Err(e) = report_variants(&journal, &out_csv, &out_md) {
                error!(%e, "report variants failed");
                std::process::exit(1);
            }
        }
        Commands::PreflightLive { timeout_ms } => {
            if let Err(e) = preflight_live(&config_path, timeout_ms).await {
                error!(%e, "preflight failed");
                std::process::exit(1);
            }
        }
        Commands::CoinbaseSmoke {
            timeout_ms,
            write_test,
            confirm,
            out,
        } => {
            if let Err(e) =
                coinbase_smoke(&config_path, timeout_ms, write_test, &confirm, &out).await
            {
                error!(%e, "coinbase smoke failed");
                std::process::exit(1);
            }
        }
        Commands::StrategyLabServe {
            bind,
            db,
            profile_id,
        } => {
            if let Err(e) = strategy_lab_serve(&bind, &db, profile_id.as_deref()).await {
                error!(%e, "strategy lab server failed");
                std::process::exit(1);
            }
        }
        Commands::StrategyBacktest {
            product,
            granularity_sec,
            limit,
            db,
            profile_id,
            out,
        } => {
            if let Err(e) = strategy_backtest(
                &product,
                granularity_sec,
                limit,
                &db,
                profile_id.as_deref(),
                &out,
            )
            .await
            {
                error!(%e, "strategy backtest failed");
                std::process::exit(1);
            }
        }
        Commands::StrategyOptimize {
            product,
            granularity_sec,
            limit,
            iterations,
            walk_forward_splits,
            seed,
            db,
            profile_id,
            out,
        } => {
            if let Err(e) = strategy_optimize(
                &product,
                granularity_sec,
                limit,
                iterations,
                walk_forward_splits,
                seed,
                &db,
                profile_id.as_deref(),
                &out,
            )
            .await
            {
                error!(%e, "strategy optimize failed");
                std::process::exit(1);
            }
        }
        Commands::StrategyProfileSave { path, db, note } => {
            if let Err(e) = strategy_profile_save(&path, &db, note.as_deref()) {
                error!(%e, "strategy profile save failed");
                std::process::exit(1);
            }
        }
        Commands::StrategyProfileLoad {
            profile_id,
            db,
            out,
        } => {
            if let Err(e) = strategy_profile_load(&profile_id, &db, out.as_deref()) {
                error!(%e, "strategy profile load failed");
                std::process::exit(1);
            }
        }
        Commands::PineParams { path, out } => {
            if let Err(e) = pine_params(&path, &out) {
                error!(%e, "pine params extraction failed");
                std::process::exit(1);
            }
        }
        Commands::TunePine {
            path,
            iterations,
            top_k,
            evaluate_cmd,
            seed,
            out,
        } => {
            if let Err(e) = tune_pine(
                &path,
                iterations,
                top_k,
                evaluate_cmd.as_deref(),
                seed,
                &out,
            ) {
                error!(%e, "pine tuning failed");
                std::process::exit(1);
            }
        }
        Commands::SaveContext { out, note } => {
            if let Err(e) = save_context(&out, note.as_deref(), &config_path) {
                error!(%e, "save context failed");
                std::process::exit(1);
            }
        }
    }
}

fn resolve_config_path(cli_path: &str) -> String {
    // If user did not explicitly pass --config, allow environment override.
    if cli_path == "config/config.toml" {
        if let Ok(env_path) = std::env::var("PT_CONFIG_PATH") {
            if !env_path.trim().is_empty() {
                return env_path;
            }
        }
    }
    cli_path.to_string()
}

fn init_tracing() {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .init();
}

async fn run(config_path: &str) -> Result<(), String> {
    info!(config_path, "loading config");
    let provider =
        ensure_rustls_crypto_provider().map_err(|e| format!("rustls provider init failed: {e}"))?;
    info!(provider = %provider, "rustls provider ready");
    let cfg = AppConfig::from_file(config_path).map_err(|e| e.to_string())?;
    let engine = TradingEngine::new(cfg).map_err(|e| e.to_string())?;
    engine.run().await.map_err(|e| e.to_string())
}

async fn status(url: &str) -> Result<(), String> {
    let body = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;
    println!("{}", body);
    Ok(())
}

async fn wallet_approve(url: &str, token_id: &str) -> Result<(), String> {
    let client = reqwest::Client::new();
    let resp = client
        .post(url)
        .json(&serde_json::json!({ "token_id": token_id }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = resp.status();
    let body = resp.text().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(format!(
            "approval request failed status={} body={body}",
            status
        ));
    }
    println!("{}", body);
    Ok(())
}

async fn post_no_body(url: &str) -> Result<(), String> {
    let client = reqwest::Client::new();
    let resp = client.post(url).send().await.map_err(|e| e.to_string())?;
    let status = resp.status();
    let body = resp.text().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(format!("request failed status={} body={body}", status));
    }
    println!("{}", body);
    Ok(())
}

async fn coinbase_auth_switch(url: &str, profile: &str) -> Result<(), String> {
    if profile.trim().is_empty() {
        return Err("profile must not be empty".to_string());
    }
    let client = reqwest::Client::new();
    let resp = client
        .post(url)
        .json(&serde_json::json!({ "profile_id": profile.trim() }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = resp.status();
    let body = resp.text().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(format!(
            "switch profile request failed status={} body={body}",
            status
        ));
    }
    println!("{}", body);
    Ok(())
}

fn set_edge_profile(
    config_path: &str,
    strategy: &str,
    min_bps: f64,
    asset: Option<&str>,
) -> Result<(), String> {
    if !min_bps.is_finite() || min_bps < 0.0 {
        return Err("min_bps must be a finite non-negative number".to_string());
    }
    let raw = fs::read_to_string(config_path).map_err(|e| e.to_string())?;
    let mut doc: toml::Value = toml::from_str(&raw).map_err(|e| e.to_string())?;
    let root = doc
        .as_table_mut()
        .ok_or_else(|| "config root is not a table".to_string())?;
    let exec = root
        .entry("execution")
        .or_insert_with(|| toml::Value::Table(toml::map::Map::new()))
        .as_table_mut()
        .ok_or_else(|| "execution is not a table".to_string())?;
    let edge = exec
        .entry("edge_profiles")
        .or_insert_with(|| toml::Value::Table(toml::map::Map::new()))
        .as_table_mut()
        .ok_or_else(|| "execution.edge_profiles is not a table".to_string())?;

    let key = match strategy.to_ascii_lowercase().as_str() {
        "maker_mm_spot" | "maker-mm-spot" | "maker" => "maker_mm_spot_min_bps",
        "conversion_cycle" | "conversion-cycle" | "conversion" => "conversion_cycle_min_bps",
        "position_reentry" | "position-reentry" | "reentry" => "position_reentry_min_bps",
        other => return Err(format!("unsupported strategy '{other}'")),
    };
    edge.insert(key.to_string(), toml::Value::from(min_bps));

    if let Some(asset) = asset {
        let normalized = asset.trim().to_ascii_uppercase();
        if normalized.is_empty() {
            return Err("asset override cannot be empty".to_string());
        }
        let overrides = edge
            .entry("per_asset_overrides_bps")
            .or_insert_with(|| toml::Value::Table(toml::map::Map::new()))
            .as_table_mut()
            .ok_or_else(|| {
                "execution.edge_profiles.per_asset_overrides_bps must be a table".to_string()
            })?;
        overrides.insert(normalized.clone(), toml::Value::from(min_bps));
        println!(
            "set execution.edge_profiles.{}={} and asset override {}={}",
            key, min_bps, normalized, min_bps
        );
    } else {
        println!("set execution.edge_profiles.{}={}", key, min_bps);
    }

    let out = toml::to_string_pretty(&doc).map_err(|e| e.to_string())?;
    fs::write(config_path, out).map_err(|e| e.to_string())?;
    Ok(())
}

async fn pilot_start(
    config_path: &str,
    capital: f64,
    profile: &str,
    timeout_ms: u64,
) -> Result<(), String> {
    if !profile.eq_ignore_ascii_case("ultra-tight") {
        return Err(format!(
            "unsupported profile '{}'; only 'ultra-tight' is supported",
            profile
        ));
    }
    if !capital.is_finite() || capital <= 0.0 || capital > 50.0 {
        return Err("capital must be in range (0, 50] for pilot-start".to_string());
    }

    preflight_live(config_path, timeout_ms).await?;
    let cfg = AppConfig::from_file(config_path).map_err(|e| e.to_string())?;
    if !matches!(cfg.engine.mode, EngineMode::Live) {
        return Err(
            "pilot-start requires engine.mode='live' after local paper/replay verification"
                .to_string(),
        );
    }
    if cfg.risk.daily_loss_limit_pct > 0.01
        || cfg.risk.max_notional_per_market > 2.5
        || cfg.risk.max_total_open_notional > 10.0
        || cfg.risk.max_markets_quoted_simultaneously > 1
    {
        return Err(
            "pilot-start blocked: risk caps exceed ultra-tight launch limits for $20 pilot"
                .to_string(),
        );
    }

    println!(
        "pilot start accepted: capital={} profile={} config={}",
        capital, profile, config_path
    );
    run(config_path).await
}

async fn verify_promoted(config_path: &str, artifact: &str, out: &str) -> Result<(), String> {
    let raw = fs::read_to_string(artifact).map_err(|e| e.to_string())?;
    let artifact_json: Value = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    let replay_path = artifact_json["replay"]["path"]
        .as_str()
        .ok_or_else(|| "artifact missing replay.path".to_string())?
        .to_string();

    if !Path::new(&replay_path).exists() {
        return Err(format!("replay path does not exist: {}", replay_path));
    }

    let mut cfg = AppConfig::from_file(config_path).map_err(|e| e.to_string())?;
    cfg.engine.mode = EngineMode::Replay;
    cfg.engine.replay_path = Some(replay_path.clone());

    let start_ms = now_unix() * 1000;
    let sqlite_path = cfg.storage.sqlite_path.clone();

    let engine = TradingEngine::new(cfg.clone()).map_err(|e| e.to_string())?;
    engine.run().await.map_err(|e| e.to_string())?;

    let conn = Connection::open(sqlite_path).map_err(|e| e.to_string())?;
    let mut total_reports = 0_usize;
    let mut reject_error = 0_usize;
    {
        let mut stmt = conn
            .prepare("SELECT status FROM execution_reports WHERE ts_ms >= ?1")
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([start_ms], |r| r.get::<_, String>(0))
            .map_err(|e| e.to_string())?;
        for row in rows {
            let status = row.map_err(|e| e.to_string())?;
            total_reports += 1;
            let s = status.to_ascii_lowercase();
            if s.contains("rejected") || s.contains("error") {
                reject_error += 1;
            }
        }
    }

    let mut max_unhedged_delta = 0.0_f64;
    let mut latest_killswitch = "unknown".to_string();
    {
        let mut stmt = conn
            .prepare("SELECT payload FROM risk_events WHERE ts_ms >= ?1")
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([start_ms], |r| r.get::<_, String>(0))
            .map_err(|e| e.to_string())?;
        for row in rows {
            let payload = row.map_err(|e| e.to_string())?;
            if let Ok(v) = serde_json::from_str::<Value>(&payload) {
                if let Some(delta) = v["unhedged_delta"].as_f64() {
                    max_unhedged_delta = max_unhedged_delta.max(delta.abs());
                }
                if let Some(k) = v["killswitch"].as_str() {
                    latest_killswitch = k.to_string();
                }
            }
        }
    }

    let effective_fee_bps_avg = {
        let mut stmt = conn
            .prepare(
                "SELECT AVG(ABS(fee_bps - rebate_bps_est)) FROM execution_costs WHERE ts_ms >= ?1",
            )
            .map_err(|e| e.to_string())?;
        let avg: Option<f64> = stmt
            .query_row([start_ms], |r| r.get(0))
            .map_err(|e| e.to_string())?;
        avg.unwrap_or(0.0)
    };

    let reject_error_rate = if total_reports == 0 {
        0.0
    } else {
        reject_error as f64 / total_reports as f64
    };

    let mut fail_reasons = Vec::new();
    if reject_error_rate > cfg.acceptance.replay.max_reject_error_rate {
        fail_reasons.push(format!(
            "reject/error rate {} > {}",
            reject_error_rate, cfg.acceptance.replay.max_reject_error_rate
        ));
    }
    if max_unhedged_delta > cfg.acceptance.replay.max_unhedged_delta {
        fail_reasons.push(format!(
            "max_unhedged_delta {} > {}",
            max_unhedged_delta, cfg.acceptance.replay.max_unhedged_delta
        ));
    }
    if effective_fee_bps_avg > cfg.acceptance.replay.max_effective_fee_bps_avg {
        fail_reasons.push(format!(
            "effective_fee_bps_avg {} > {}",
            effective_fee_bps_avg, cfg.acceptance.replay.max_effective_fee_bps_avg
        ));
    }
    if cfg.acceptance.replay.require_killswitch_running
        && !latest_killswitch.eq_ignore_ascii_case("running")
    {
        fail_reasons.push(format!("killswitch is {}", latest_killswitch));
    }

    let report = ReplayAcceptanceReport {
        artifact_path: artifact.to_string(),
        passed: fail_reasons.is_empty(),
        fail_reasons,
        total_reports,
        reject_error_rate,
        max_unhedged_delta,
        killswitch: latest_killswitch,
        effective_fee_bps_avg,
        created_ts: chrono::Utc::now(),
    };

    write_json_file(out, &report)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
    );
    if !report.passed {
        return Err("replay acceptance failed".to_string());
    }
    Ok(())
}

fn report_variants(journal_path: &str, out_csv: &str, out_md: &str) -> Result<(), String> {
    let conn = Connection::open(journal_path).map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "
            SELECT
              market,
              variant,
              COUNT(*) as runs,
              AVG(total_return) as avg_total_return,
              AVG(max_drawdown) as avg_max_drawdown,
              AVG(pnl_abs) as avg_pnl_abs,
              AVG(trades) as avg_trades
            FROM market_results
            GROUP BY market, variant
            ORDER BY avg_total_return DESC
            ",
        )
        .map_err(|e| e.to_string())?;

    let mut rows = Vec::new();
    let mapped = stmt
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, i64>(2)?,
                r.get::<_, f64>(3)?,
                r.get::<_, f64>(4)?,
                r.get::<_, f64>(5)?,
                r.get::<_, f64>(6)?,
            ))
        })
        .map_err(|e| e.to_string())?;

    for row in mapped {
        rows.push(row.map_err(|e| e.to_string())?);
    }

    let mut csv_out = String::from(
        "market,variant,runs,avg_total_return,avg_max_drawdown,avg_pnl_abs,avg_trades\n",
    );
    let mut md_out = String::from(
        "| market | variant | runs | avg_total_return | avg_max_drawdown | avg_pnl_abs | avg_trades |\n|---|---:|---:|---:|---:|---:|---:|\n",
    );

    for (market, variant, runs, avg_ret, avg_dd, avg_pnl, avg_trades) in rows {
        csv_out.push_str(&format!(
            "{},{},{},{:.8},{:.8},{:.8},{:.4}\n",
            market, variant, runs, avg_ret, avg_dd, avg_pnl, avg_trades
        ));
        md_out.push_str(&format!(
            "| {} | {} | {} | {:.6} | {:.6} | {:.4} | {:.2} |\n",
            market, variant, runs, avg_ret, avg_dd, avg_pnl, avg_trades
        ));
    }

    write_text_file(out_csv, &csv_out)?;
    write_text_file(out_md, &md_out)?;
    println!("{}\n{}", out_csv, out_md);
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum PreflightLevel {
    Pass,
    Warn,
    Fail,
}

#[derive(Debug, Clone)]
struct PreflightCheck {
    name: String,
    level: PreflightLevel,
    detail: String,
}

impl PreflightCheck {
    fn pass(name: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            level: PreflightLevel::Pass,
            detail: detail.into(),
        }
    }

    fn warn(name: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            level: PreflightLevel::Warn,
            detail: detail.into(),
        }
    }

    fn fail(name: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            level: PreflightLevel::Fail,
            detail: detail.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CoinbaseSmokeCheck {
    name: String,
    ok: bool,
    detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CoinbaseSmokeReport {
    created_ts: String,
    config_path: String,
    mode: String,
    write_test: bool,
    profile_id: Option<String>,
    passed: bool,
    checks: Vec<CoinbaseSmokeCheck>,
}

async fn preflight_live(config_path: &str, timeout_ms: u64) -> Result<(), String> {
    let cfg = AppConfig::from_file(config_path).map_err(|e| e.to_string())?;
    let mut checks = Vec::new();

    checks.push(match cfg.engine.mode {
        EngineMode::Live => PreflightCheck::pass("engine.mode", "live"),
        _ => PreflightCheck::warn(
            "engine.mode",
            format!(
                "configured as {:?}; preflight still validates live prerequisites",
                cfg.engine.mode
            ),
        ),
    });

    match ensure_rustls_crypto_provider() {
        Ok(provider) => checks.push(PreflightCheck::pass(
            "runtime.rustls_provider",
            format!("installed provider={provider}"),
        )),
        Err(e) => {
            if matches!(cfg.engine.mode, EngineMode::Live) {
                checks.push(PreflightCheck::fail(
                    "runtime.rustls_provider",
                    format!("provider install failed: {e}"),
                ));
            } else {
                checks.push(PreflightCheck::warn(
                    "runtime.rustls_provider",
                    format!("provider install failed: {e}"),
                ));
            }
        }
    }

    checks.push(require_nonempty(
        "venues.polymarket.private_key",
        cfg.venues.polymarket.private_key.as_deref(),
    ));
    let legacy_coinbase_auth = cfg
        .venues
        .coinbase
        .api_key
        .as_deref()
        .map(|v| !v.trim().is_empty())
        .unwrap_or(false)
        && cfg
            .venues
            .coinbase
            .api_secret
            .as_deref()
            .map(|v| !v.trim().is_empty())
            .unwrap_or(false);
    let active_profile = cfg
        .venues
        .coinbase
        .auth
        .active_profile
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty());
    let active_profile_cfg =
        active_profile.and_then(|profile_id| cfg.venues.coinbase.auth.profiles.get(profile_id));
    let profile_has_source = active_profile_cfg
        .map(|profile| {
            profile
                .cdp_key_file
                .as_deref()
                .map(|v| !v.trim().is_empty())
                .unwrap_or(false)
                || profile
                    .cdp_secret_id
                    .as_deref()
                    .map(|v| !v.trim().is_empty())
                    .unwrap_or(false)
        })
        .unwrap_or(false);

    if legacy_coinbase_auth {
        checks.push(PreflightCheck::pass(
            "coinbase.auth",
            "legacy venues.coinbase.api_key/api_secret present",
        ));
    } else if profile_has_source {
        let mut detail = format!(
            "profile '{}' has cdp_key_file/cdp_secret_id",
            active_profile.unwrap_or("unknown")
        );
        if let Some(profile) = active_profile_cfg {
            if let Some(path) = profile.cdp_key_file.as_deref() {
                let trimmed = path.trim();
                if !trimmed.is_empty() {
                    if Path::new(trimmed).exists() {
                        detail.push_str("; cdp_key_file exists");
                    } else {
                        checks.push(PreflightCheck::warn(
                            "coinbase.auth.cdp_key_file",
                            format!("configured file does not exist yet: {trimmed}"),
                        ));
                    }
                }
            }
        }
        checks.push(PreflightCheck::pass("coinbase.auth", detail));
    } else {
        checks.push(PreflightCheck::fail(
            "coinbase.auth",
            "missing auth: provide legacy api_key/api_secret or active profile with cdp_key_file/cdp_secret_id",
        ));
    }

    if legacy_coinbase_auth || profile_has_source {
        let wallet_client = if let Some(profile) = active_profile_cfg {
            let coinbase_cfg = cfg.venues.coinbase.clone();
            if profile
                .cdp_key_file
                .as_deref()
                .map(|v| !v.trim().is_empty())
                .unwrap_or(false)
                || profile
                    .cdp_secret_id
                    .as_deref()
                    .map(|v| !v.trim().is_empty())
                    .unwrap_or(false)
            {
                match CoinbaseAuthManager::new(coinbase_cfg.clone(), cfg.engine.mode.clone()) {
                    Ok(manager) => Some(CoinbaseWalletClient::new_with_auth_manager(
                        coinbase_cfg.api_base.clone(),
                        std::sync::Arc::new(manager),
                        coinbase_cfg.passphrase.clone(),
                    )),
                    Err(e) => {
                        checks.push(if matches!(cfg.engine.mode, EngineMode::Live) {
                            PreflightCheck::fail(
                                "coinbase.auth.profile_init",
                                format!("failed to initialize auth profile: {e}"),
                            )
                        } else {
                            PreflightCheck::warn(
                                "coinbase.auth.profile_init",
                                format!("failed to initialize auth profile: {e}"),
                            )
                        });
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        };

        let wallet_client = wallet_client.unwrap_or_else(|| {
            CoinbaseWalletClient::new(
                cfg.venues.coinbase.api_base.clone(),
                cfg.venues.coinbase.api_key.clone(),
                cfg.venues.coinbase.api_secret.clone(),
                cfg.venues.coinbase.passphrase.clone(),
            )
        });

        match wallet_client.auth_token_self_test() {
            Ok((rest, ws)) => checks.push(PreflightCheck::pass(
                "coinbase.auth.jwt",
                format!("rest_jwt_len={} ws_jwt_len={}", rest.len(), ws.len()),
            )),
            Err(e) => checks.push(if matches!(cfg.engine.mode, EngineMode::Live) {
                PreflightCheck::fail("coinbase.auth.jwt", format!("jwt self-test failed: {e}"))
            } else {
                PreflightCheck::warn("coinbase.auth.jwt", format!("jwt self-test failed: {e}"))
            }),
        }

        match tokio::time::timeout(
            Duration::from_millis(timeout_ms.max(500)),
            wallet_client.probe_authenticated_accounts(),
        )
        .await
        {
            Ok(Ok(accounts)) => checks.push(PreflightCheck::pass(
                "coinbase.auth.probe_accounts",
                format!("authenticated probe ok (accounts={accounts})"),
            )),
            Ok(Err(e)) => checks.push(if matches!(cfg.engine.mode, EngineMode::Live) {
                PreflightCheck::fail(
                    "coinbase.auth.probe_accounts",
                    format!("authenticated probe failed: {e}"),
                )
            } else {
                PreflightCheck::warn(
                    "coinbase.auth.probe_accounts",
                    format!("authenticated probe failed: {e}"),
                )
            }),
            Err(_) => checks.push(if matches!(cfg.engine.mode, EngineMode::Live) {
                PreflightCheck::fail(
                    "coinbase.auth.probe_accounts",
                    "authenticated probe timed out".to_string(),
                )
            } else {
                PreflightCheck::warn(
                    "coinbase.auth.probe_accounts",
                    "authenticated probe timed out".to_string(),
                )
            }),
        }
    }

    checks.push(check_bind(
        "ops.dashboard_bind",
        &cfg.ops.dashboard_bind,
        "dashboard",
    ));

    if cfg.signals.tradingview.enabled {
        checks.push(check_bind(
            "signals.tradingview.bind_addr",
            &cfg.signals.tradingview.bind_addr,
            "tradingview listener",
        ));
    } else {
        checks.push(PreflightCheck::warn(
            "signals.tradingview.bind_addr",
            "tradingview listener disabled",
        ));
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(timeout_ms.max(500)))
        .build()
        .map_err(|e| format!("failed to build http client: {e}"))?;

    let gamma_probe = format!(
        "{}/markets?limit=1",
        trim_trailing_slash(&cfg.venues.polymarket.gamma_api)
    );
    checks.push(probe_http(&client, "polymarket.gamma_api", &gamma_probe).await);

    let clob_probe = format!(
        "{}/time",
        trim_trailing_slash(&cfg.venues.polymarket.clob_api)
    );
    checks.push(probe_http(&client, "polymarket.clob_api", &clob_probe).await);

    let coinbase_probe = format!(
        "{}/time",
        trim_trailing_slash(&cfg.venues.coinbase.api_base)
    );
    checks.push(probe_http(&client, "coinbase.api_base", &coinbase_probe).await);

    println!("live preflight report ({})", config_path);
    for c in &checks {
        print_preflight_check(c);
    }

    let fail_count = checks
        .iter()
        .filter(|c| matches!(c.level, PreflightLevel::Fail))
        .count();
    let warn_count = checks
        .iter()
        .filter(|c| matches!(c.level, PreflightLevel::Warn))
        .count();
    println!("summary: {} failed, {} warnings", fail_count, warn_count);

    if fail_count > 0 {
        return Err(format!(
            "preflight blocked by {} failing checks",
            fail_count
        ));
    }

    Ok(())
}

fn print_preflight_check(check: &PreflightCheck) {
    let label = match check.level {
        PreflightLevel::Pass => "PASS",
        PreflightLevel::Warn => "WARN",
        PreflightLevel::Fail => "FAIL",
    };
    println!("[{}] {}: {}", label, check.name, check.detail);
}

async fn coinbase_smoke(
    config_path: &str,
    timeout_ms: u64,
    write_test: bool,
    confirm: &str,
    out: &str,
) -> Result<(), String> {
    const WRITE_CONFIRM_TOKEN: &str = "I_UNDERSTAND_POST_ONLY_TEST_ORDERS";

    let cfg = AppConfig::from_file(config_path).map_err(|e| e.to_string())?;
    let mut checks: Vec<CoinbaseSmokeCheck> = Vec::new();
    let mut passed = true;

    let mut push_check = |name: &str, ok: bool, detail: String| {
        if !ok {
            passed = false;
        }
        checks.push(CoinbaseSmokeCheck {
            name: name.to_string(),
            ok,
            detail,
        });
    };

    match ensure_rustls_crypto_provider() {
        Ok(provider) => push_check(
            "runtime.rustls_provider",
            true,
            format!("installed provider={provider}"),
        ),
        Err(e) => push_check(
            "runtime.rustls_provider",
            false,
            format!("provider install failed: {e}"),
        ),
    }

    let active_profile = cfg
        .venues
        .coinbase
        .auth
        .active_profile
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(str::to_string);

    let wallet_client = if let Some(profile_id) = active_profile.as_deref() {
        if cfg
            .venues
            .coinbase
            .auth
            .profiles
            .get(profile_id)
            .map(|p| {
                p.cdp_key_file
                    .as_deref()
                    .map(|v| !v.trim().is_empty())
                    .unwrap_or(false)
                    || p.cdp_secret_id
                        .as_deref()
                        .map(|v| !v.trim().is_empty())
                        .unwrap_or(false)
            })
            .unwrap_or(false)
        {
            match CoinbaseAuthManager::new(cfg.venues.coinbase.clone(), cfg.engine.mode.clone()) {
                Ok(manager) => {
                    push_check(
                        "coinbase.auth.profile",
                        true,
                        format!("profile initialized ({profile_id})"),
                    );
                    CoinbaseWalletClient::new_with_auth_manager(
                        cfg.venues.coinbase.api_base.clone(),
                        std::sync::Arc::new(manager),
                        cfg.venues.coinbase.passphrase.clone(),
                    )
                }
                Err(e) => {
                    push_check(
                        "coinbase.auth.profile",
                        false,
                        format!("profile init failed: {e}"),
                    );
                    CoinbaseWalletClient::new(
                        cfg.venues.coinbase.api_base.clone(),
                        cfg.venues.coinbase.api_key.clone(),
                        cfg.venues.coinbase.api_secret.clone(),
                        cfg.venues.coinbase.passphrase.clone(),
                    )
                }
            }
        } else {
            CoinbaseWalletClient::new(
                cfg.venues.coinbase.api_base.clone(),
                cfg.venues.coinbase.api_key.clone(),
                cfg.venues.coinbase.api_secret.clone(),
                cfg.venues.coinbase.passphrase.clone(),
            )
        }
    } else {
        CoinbaseWalletClient::new(
            cfg.venues.coinbase.api_base.clone(),
            cfg.venues.coinbase.api_key.clone(),
            cfg.venues.coinbase.api_secret.clone(),
            cfg.venues.coinbase.passphrase.clone(),
        )
    };

    match wallet_client.auth_token_self_test() {
        Ok((rest, ws)) => push_check(
            "coinbase.auth.jwt",
            true,
            format!("rest_jwt_len={} ws_jwt_len={}", rest.len(), ws.len()),
        ),
        Err(e) => push_check(
            "coinbase.auth.jwt",
            false,
            format!("jwt self-test failed: {e}"),
        ),
    }

    match tokio::time::timeout(
        Duration::from_millis(timeout_ms.max(500)),
        wallet_client.probe_authenticated_accounts(),
    )
    .await
    {
        Ok(Ok(accounts)) => push_check(
            "coinbase.auth.accounts_probe",
            true,
            format!("authenticated accounts probe ok count={accounts}"),
        ),
        Ok(Err(e)) => push_check(
            "coinbase.auth.accounts_probe",
            false,
            format!("accounts probe failed: {e}"),
        ),
        Err(_) => push_check(
            "coinbase.auth.accounts_probe",
            false,
            "accounts probe timed out".to_string(),
        ),
    }

    let products = cfg.venues.coinbase.products.clone();
    if products.is_empty() {
        push_check(
            "coinbase.products",
            false,
            "venues.coinbase.products is empty".to_string(),
        );
    } else {
        match wallet_client.fetch_wallet_balances(&products).await {
            Ok(balances) => push_check(
                "coinbase.accounts",
                true,
                format!("wallet balances fetched rows={}", balances.len()),
            ),
            Err(e) => push_check(
                "coinbase.accounts",
                false,
                format!("fetch balances failed: {e}"),
            ),
        }
        match wallet_client.fetch_open_orders().await {
            Ok(orders) => push_check(
                "coinbase.open_orders",
                true,
                format!("open orders fetched rows={}", orders.len()),
            ),
            Err(e) => push_check(
                "coinbase.open_orders",
                false,
                format!("open orders failed: {e}"),
            ),
        }

        let first_product = products[0].clone();
        match wallet_client.get_product(&first_product).await {
            Ok(p) => push_check(
                "coinbase.product",
                true,
                format!(
                    "product {} status={} base_increment={}",
                    p.product_id, p.status, p.base_increment
                ),
            ),
            Err(e) => push_check(
                "coinbase.product",
                false,
                format!("get product failed: {e}"),
            ),
        }
        match wallet_client.get_product_book(&first_product, 50).await {
            Ok(book) => push_check(
                "coinbase.book",
                true,
                format!(
                    "book {} bids={} asks={}",
                    book.product_id,
                    book.bids.len(),
                    book.asks.len()
                ),
            ),
            Err(e) => push_check("coinbase.book", false, format!("get book failed: {e}")),
        }
        match wallet_client.get_best_bid_ask(&products).await {
            Ok(books) => push_check(
                "coinbase.best_bid_ask",
                true,
                format!("best bid/ask rows={}", books.len()),
            ),
            Err(e) => push_check(
                "coinbase.best_bid_ask",
                false,
                format!("best bid/ask failed: {e}"),
            ),
        }
        match wallet_client.get_transaction_summary().await {
            Ok(summary) => push_check(
                "coinbase.transaction_summary",
                true,
                format!(
                    "maker_fee_rate={:?} taker_fee_rate={:?}",
                    summary.maker_fee_rate, summary.taker_fee_rate
                ),
            ),
            Err(e) => push_check(
                "coinbase.transaction_summary",
                false,
                format!("transaction summary failed: {e}"),
            ),
        }

        let ws_cfg = CoinbaseWsRunConfig {
            ws_url: cfg.venues.coinbase.ws.url.clone(),
            channels: vec![
                "heartbeats".to_string(),
                "level2".to_string(),
                "user".to_string(),
            ],
            product_ids: vec![first_product.clone()],
            heartbeat_timeout_ms: cfg.venues.coinbase.ws.heartbeat_timeout_ms,
            resync_on_gap: cfg.venues.coinbase.ws.resync_on_gap,
        };

        match wallet_client.spawn_ws_event_loop(ws_cfg) {
            Ok(mut rx) => {
                let ws_result = tokio::time::timeout(Duration::from_secs(8), async {
                    let mut got_heartbeat = false;
                    let mut got_l2 = false;
                    let mut user_subscribed = false;
                    let mut user_update_seen = false;
                    let mut user_skipped = false;
                    for _ in 0..40 {
                        if let Some(event) = rx.recv().await {
                            match event {
                                CoinbaseWsEvent::Heartbeat { .. } => got_heartbeat = true,
                                CoinbaseWsEvent::L2 { .. } => got_l2 = true,
                                CoinbaseWsEvent::User { .. } => user_update_seen = true,
                                CoinbaseWsEvent::Subscribed { channel } => {
                                    if channel.eq_ignore_ascii_case("user")
                                        || channel.eq_ignore_ascii_case("subscriptions")
                                    {
                                        user_subscribed = true;
                                    }
                                }
                                CoinbaseWsEvent::Error { message } => {
                                    if message.contains("user channel skipped") {
                                        user_skipped = true;
                                    }
                                }
                                CoinbaseWsEvent::Reconnected | CoinbaseWsEvent::Gap { .. } => {}
                            }
                            if got_heartbeat
                                && got_l2
                                && (user_update_seen || user_subscribed || user_skipped)
                            {
                                return Ok::<(), String>(());
                            }
                        } else {
                            break;
                        }
                    }
                    Err("did not receive expected ws events within sample window".to_string())
                })
                .await;

                match ws_result {
                    Ok(Ok(())) => push_check(
                        "coinbase.ws",
                        true,
                        "ws channel subscriptions and sample events observed".to_string(),
                    ),
                    Ok(Err(e)) => push_check("coinbase.ws", false, e),
                    Err(_) => push_check("coinbase.ws", false, "ws sample timed out".to_string()),
                }
            }
            Err(e) => push_check("coinbase.ws", false, format!("spawn ws loop failed: {e}")),
        }

        if write_test {
            if confirm.trim() != WRITE_CONFIRM_TOKEN {
                push_check(
                    "coinbase.write_test.guard",
                    false,
                    format!("write test denied: pass --confirm {}", WRITE_CONFIRM_TOKEN),
                );
            } else {
                let top = wallet_client.fetch_top_of_book(&first_product).await;
                match top {
                    Ok(top) => {
                        let buy_price = (top.best_bid * 0.90).max(0.00000001);
                        let mut size = 0.001_f64;
                        if let Ok(product) = wallet_client.get_product(&first_product).await {
                            if let Ok(incr) = product.base_increment.parse::<f64>() {
                                if incr.is_finite() && incr > 0.0 {
                                    size = (incr * 10.0).max(incr);
                                }
                            }
                        }
                        let preview = wallet_client
                            .preview_order_post_only(
                                &first_product,
                                pt_core::Side::Buy,
                                size,
                                buy_price,
                            )
                            .await;
                        match preview {
                            Ok(p) if p.success => {
                                push_check(
                                    "coinbase.write_test.preview",
                                    true,
                                    "preview accepted".to_string(),
                                );
                                match wallet_client
                                    .create_order_post_only(
                                        &first_product,
                                        pt_core::Side::Buy,
                                        size,
                                        buy_price,
                                    )
                                    .await
                                {
                                    Ok(create_report) => {
                                        push_check(
                                            "coinbase.write_test.create",
                                            true,
                                            format!("created order_id={}", create_report.order_id),
                                        );
                                        let edit_px = (buy_price * 0.99).max(0.00000001);
                                        match wallet_client
                                            .edit_order(&create_report.order_id, edit_px, size)
                                            .await
                                        {
                                            Ok(_) => push_check(
                                                "coinbase.write_test.edit",
                                                true,
                                                "edit succeeded".to_string(),
                                            ),
                                            Err(e) => push_check(
                                                "coinbase.write_test.edit",
                                                false,
                                                format!("edit failed: {e}"),
                                            ),
                                        }
                                        match wallet_client
                                            .cancel_orders_batch(&[create_report.order_id.clone()])
                                            .await
                                        {
                                            Ok(results) => push_check(
                                                "coinbase.write_test.cancel",
                                                true,
                                                format!("cancel results={}", results.len()),
                                            ),
                                            Err(e) => push_check(
                                                "coinbase.write_test.cancel",
                                                false,
                                                format!("cancel failed: {e}"),
                                            ),
                                        }
                                    }
                                    Err(e) => push_check(
                                        "coinbase.write_test.create",
                                        false,
                                        format!("create failed: {e}"),
                                    ),
                                }
                            }
                            Ok(p) => push_check(
                                "coinbase.write_test.preview",
                                false,
                                format!(
                                    "preview rejected: {}",
                                    p.failure_reason.unwrap_or_else(|| "unknown".to_string())
                                ),
                            ),
                            Err(e) => push_check(
                                "coinbase.write_test.preview",
                                false,
                                format!("preview failed: {e}"),
                            ),
                        }
                    }
                    Err(e) => push_check(
                        "coinbase.write_test.book",
                        false,
                        format!("failed to fetch top of book: {e}"),
                    ),
                }
            }
        }
    }

    let report = CoinbaseSmokeReport {
        created_ts: chrono::Utc::now().to_rfc3339(),
        config_path: config_path.to_string(),
        mode: format!("{:?}", cfg.engine.mode),
        write_test,
        profile_id: active_profile,
        passed,
        checks,
    };
    write_json_file(out, &report)?;

    println!("coinbase smoke report: {}", out);
    for c in &report.checks {
        let label = if c.ok { "PASS" } else { "FAIL" };
        println!("[{}] {}: {}", label, c.name, c.detail);
    }

    if !report.passed {
        return Err("one or more Coinbase smoke checks failed".to_string());
    }
    Ok(())
}

fn resolve_strategy_profile(db: &str, profile_id: Option<&str>) -> Result<StrategyProfile, String> {
    if let Some(profile_id) = profile_id {
        return load_strategy_profile(db, profile_id).map_err(|e| e.to_string());
    }
    Ok(StrategyProfile::default())
}

async fn strategy_lab_serve(bind: &str, db: &str, profile_id: Option<&str>) -> Result<(), String> {
    let profile = resolve_strategy_profile(db, profile_id)?;
    save_profile(db, &profile, Some("strategy-lab-serve bootstrap")).map_err(|e| e.to_string())?;
    let state = StrategyLabState::new(db.to_string(), profile);
    println!("strategy lab serving on http://{}", bind);
    serve_strategy_lab(state, bind).await
}

async fn strategy_backtest(
    product: &str,
    granularity_sec: u32,
    limit: usize,
    db: &str,
    profile_id: Option<&str>,
    out: &str,
) -> Result<(), String> {
    let mut profile = resolve_strategy_profile(db, profile_id)?;
    profile.product_id = product.to_string();
    profile.granularity_sec = granularity_sec;
    profile.candle_limit = limit;
    let candles = fetch_coinbase_candles(product, granularity_sec, limit)
        .await
        .map_err(|e| e.to_string())?;
    if candles.len() < 50 {
        return Err("insufficient candles: need at least 50".to_string());
    }
    let report = run_backtest(&profile, &candles);
    save_strategy_run(db, &report).map_err(|e| e.to_string())?;
    write_json_file(out, &report)?;
    println!(
        "backtest saved {} (return={:.2}% dd={:.2}% trades={})",
        out,
        report.total_return_pct * 100.0,
        report.max_drawdown_pct * 100.0,
        report.trades
    );
    Ok(())
}

async fn strategy_optimize(
    product: &str,
    granularity_sec: u32,
    limit: usize,
    iterations: usize,
    walk_forward_splits: usize,
    seed: u64,
    db: &str,
    profile_id: Option<&str>,
    out: &str,
) -> Result<(), String> {
    let mut profile = resolve_strategy_profile(db, profile_id)?;
    profile.product_id = product.to_string();
    profile.granularity_sec = granularity_sec;
    profile.candle_limit = limit;
    let candles = fetch_coinbase_candles(product, granularity_sec, limit)
        .await
        .map_err(|e| e.to_string())?;
    if candles.len() < 120 {
        return Err("insufficient candles: need at least 120 for optimization".to_string());
    }
    let report =
        optimize_random_walk_forward(&profile, &candles, iterations, walk_forward_splits, seed);
    write_json_file(out, &report)?;

    if let Some(best) = report.top.first() {
        save_profile(
            db,
            &best.profile,
            Some("saved from strategy-optimize best candidate"),
        )
        .map_err(|e| e.to_string())?;
    }

    println!("optimize saved {} (candidates={})", out, report.top.len());
    Ok(())
}

fn strategy_profile_save(path: &str, db: &str, note: Option<&str>) -> Result<(), String> {
    let payload = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut profile: StrategyProfile =
        serde_json::from_str(&payload).map_err(|e| format!("invalid profile json: {}", e))?;
    profile.version = profile.version.saturating_add(1);
    save_profile(db, &profile, note).map_err(|e| e.to_string())?;
    println!(
        "saved profile {} v{} into {}",
        profile.profile_id, profile.version, db
    );
    Ok(())
}

fn strategy_profile_load(profile_id: &str, db: &str, out: Option<&str>) -> Result<(), String> {
    let profile = load_strategy_profile(db, profile_id).map_err(|e| e.to_string())?;
    let json = serde_json::to_string_pretty(&profile).map_err(|e| e.to_string())?;
    if let Some(out) = out {
        write_text_file(out, &json)?;
        println!("{}", out);
    } else {
        println!("{}", json);
    }
    Ok(())
}

fn require_nonempty(name: &str, value: Option<&str>) -> PreflightCheck {
    match value {
        Some(v) if !v.trim().is_empty() => PreflightCheck::pass(name, "set"),
        _ => PreflightCheck::fail(name, "missing or empty"),
    }
}

fn check_bind(name: &str, bind_addr: &str, label: &str) -> PreflightCheck {
    match TcpListener::bind(bind_addr) {
        Ok(listener) => {
            drop(listener);
            PreflightCheck::pass(name, format!("{label} bind address is available"))
        }
        Err(e) => PreflightCheck::fail(name, format!("cannot bind {bind_addr}: {e}")),
    }
}

async fn probe_http(client: &reqwest::Client, name: &str, url: &str) -> PreflightCheck {
    match client.get(url).send().await {
        Ok(resp) => PreflightCheck::pass(name, format!("reachable (status {})", resp.status())),
        Err(e) => PreflightCheck::fail(name, format!("request failed: {e}")),
    }
}

fn trim_trailing_slash(input: &str) -> &str {
    input.trim_end_matches('/')
}

fn pine_params(path: &str, out: &str) -> Result<(), String> {
    let params = extract_pine_params(path)?;
    if params.is_empty() {
        return Err("no pine inputs were found in the script".to_string());
    }
    write_json_file(out, &params)?;
    println!("extracted {} input params -> {}", params.len(), out);
    Ok(())
}

fn tune_pine(
    path: &str,
    iterations: usize,
    top_k: usize,
    evaluate_cmd: Option<&str>,
    seed: Option<u64>,
    out: &str,
) -> Result<(), String> {
    if iterations == 0 {
        return Err("iterations must be > 0".to_string());
    }
    if top_k == 0 {
        return Err("top_k must be > 0".to_string());
    }

    let params = extract_pine_params(path)?;
    if params.is_empty() {
        return Err("no tunable params found in pine script".to_string());
    }

    let mut rng = make_rng(seed);
    let mut candidates = Vec::with_capacity(iterations);

    for i in 0..iterations {
        let mut candidate_params = HashMap::new();
        for param in &params {
            candidate_params.insert(param.name.clone(), sample_param_value(param, &mut rng));
        }

        let score = if let Some(cmd) = evaluate_cmd {
            Some(run_evaluator(cmd, path, &candidate_params)?)
        } else {
            None
        };

        candidates.push(PineTuneCandidate {
            iteration: i + 1,
            params: candidate_params,
            score,
        });
    }

    let mut scored: Vec<PineTuneCandidate> = candidates
        .iter()
        .filter(|c| c.score.is_some())
        .cloned()
        .collect();
    scored.sort_by(|a, b| {
        b.score
            .unwrap_or(f64::NEG_INFINITY)
            .partial_cmp(&a.score.unwrap_or(f64::NEG_INFINITY))
            .unwrap_or(Ordering::Equal)
    });

    let best = scored.first().cloned();
    let top_candidates = if !scored.is_empty() {
        scored.into_iter().take(top_k).collect()
    } else {
        candidates.into_iter().take(top_k).collect()
    };

    let report = PineTuneReport {
        script_path: path.to_string(),
        iterations,
        evaluate_cmd: evaluate_cmd.map(str::to_string),
        scored: evaluate_cmd.is_some(),
        top_k,
        best,
        top_candidates,
    };

    write_json_file(out, &report)?;
    println!("tuning report written -> {}", out);
    if evaluate_cmd.is_none() {
        println!(
            "note: no --evaluate-cmd was provided, so candidates are generated but not score-ranked"
        );
    }
    Ok(())
}

fn save_context(out: &str, note: Option<&str>, config_path: &str) -> Result<(), String> {
    let mut text = String::new();
    text.push_str("# Session Context\n\n");
    text.push_str(&format!(
        "Generated at UNIX epoch seconds: `{}`\n\n",
        now_unix()
    ));
    if let Some(note) = note {
        text.push_str("## Note\n");
        text.push_str(note);
        text.push_str("\n\n");
    }
    text.push_str("## Runtime\n");
    text.push_str(&format!(
        "`rustc`: `{}`\n",
        cmd_out("rustc", &["--version"])
    ));
    text.push_str(&format!(
        "`cargo`: `{}`\n\n",
        cmd_out("cargo", &["--version"])
    ));
    text.push_str("## Core Commands\n");
    text.push_str(&format!(
        "- Run engine: `cargo run -p pt-cli -- run --config {}`\n",
        config_path
    ));
    text.push_str(&format!(
        "- Live preflight: `cargo run -p pt-cli -- preflight-live --config {} --timeout-ms 3000`\n",
        config_path
    ));
    text.push_str("- Dashboard: `http://127.0.0.1:8080/`\n");
    text.push_str("- Health: `cargo run -p pt-cli -- status --url http://127.0.0.1:8080/health`\n");
    text.push_str("- Wallet status: `cargo run -p pt-cli -- wallet-status`\n");
    text.push_str("- Wallet plan: `cargo run -p pt-cli -- wallet-plan`\n");
    text.push_str(
        "- Wallet approve: `cargo run -p pt-cli -- wallet-approve --token-id <token_id>`\n",
    );
    text.push_str("- Execution status: `cargo run -p pt-cli -- execution-status`\n");
    text.push_str("- Coinbase WS status: `cargo run -p pt-cli -- coinbase-ws-status`\n");
    text.push_str("- Coinbase auth status: `cargo run -p pt-cli -- coinbase-auth-status`\n");
    text.push_str("- Coinbase auth reload: `cargo run -p pt-cli -- coinbase-auth-reload`\n");
    text.push_str(
        "- Coinbase auth switch: `cargo run -p pt-cli -- coinbase-auth-switch --profile primary`\n",
    );
    text.push_str("- Order manager status: `cargo run -p pt-cli -- order-manager-status`\n");
    text.push_str("- Routes status: `cargo run -p pt-cli -- routes-status`\n");
    text.push_str("- Set edge profile: `cargo run -p pt-cli -- set-edge-profile --strategy maker_mm_spot --min-bps 8`\n");
    text.push_str("- Pilot start: `cargo run -p pt-cli -- pilot-start --capital 20 --profile ultra-tight --timeout-ms 3000`\n");
    text.push_str("- Market list: `curl -s http://127.0.0.1:8080/state/markets | jq '.[0:5]'`\n");
    text.push_str(
        "- Market history: `curl -s \"http://127.0.0.1:8080/state/history?limit=120\" | jq`\n",
    );
    text.push_str("- Coinbase orderbook: `curl -s http://127.0.0.1:8080/state/coinbase/orderbook | jq '.[0:5]'`\n");
    text.push_str("- Route opportunities: `curl -s http://127.0.0.1:8080/state/routes/opportunities | jq '.[0:5]'`\n");
    text.push_str("- Extract pine params: `cargo run -p pt-cli -- pine-params --path pine-scripts/<script> --out data/tuning/pine_params.json`\n");
    text.push_str("- Tune pine params: `cargo run -p pt-cli -- tune-pine --path pine-scripts/<script> --iterations 100 --evaluate-cmd \"python3 tools/evaluate_candidate.py\"`\n\n");
    text.push_str("- Promote tuning candidate: `./scripts/promote_candidate.sh data/tuning/pine_tuning_results.json data/tuning/promoted_candidate.json BTC 15m`\n");
    text.push_str("- Verify promoted artifact: `cargo run -p pt-cli -- verify-promoted --artifact data/tuning/promoted_candidate.json --out data/output/replay_acceptance_report.json`\n");
    text.push_str("- Report variants: `cargo run -p pt-cli -- report-variants --journal data/strategy_lab/trade_journal.sqlite --out-csv data/output/variant_report.csv --out-md data/output/variant_report.md`\n");
    text.push_str("- Paper soak: `./scripts/paper_soak.sh 3600 30 config/config.toml`\n");
    text.push_str(
        "- Tiny live pilot checks: `./scripts/tiny_live_pilot.sh config/config.toml 3000`\n",
    );
    text.push_str("- Install git hooks: `./scripts/install_git_hooks.sh`\n\n");
    text.push_str("## Live Prerequisites\n");
    text.push_str("- Set `engine.mode = \"live\"` in config.\n");
    text.push_str("- Set `venues.polymarket.private_key` or `POLYMARKET_PRIVATE_KEY`.\n");
    text.push_str("- Coinbase auth: legacy (`venues.coinbase.api_key/api_secret`) OR profile (`venues.coinbase.auth.active_profile` + `cdp_key_file|cdp_secret_id`).\n");
    text.push_str("- Env overrides: `COINBASE_AUTH_PROFILE`, `COINBASE_CDP_KEY_FILE`, `COINBASE_CDP_SECRET_ID`, `COINBASE_EXPECTED_KEY_ID`.\n");
    text.push_str("- Keep hard risk caps enabled for tiny-live rollout.\n");

    let out_path = PathBuf::from(out);
    ensure_parent_dir(&out_path)?;
    fs::write(&out_path, text).map_err(|e| e.to_string())?;
    println!("saved context instructions -> {}", out);
    Ok(())
}

fn extract_pine_params(path: &str) -> Result<Vec<PineParam>, String> {
    let script = fs::read_to_string(path).map_err(|e| format!("unable to read {}: {e}", path))?;
    let mut params = Vec::new();

    for (idx, line) in script.lines().enumerate() {
        if let Some(param) = parse_pine_input_line(line, idx + 1) {
            params.push(param);
        }
    }

    Ok(params)
}

fn parse_pine_input_line(line: &str, line_no: usize) -> Option<PineParam> {
    let cleaned = strip_inline_comment(line);
    let cleaned = cleaned.trim();
    if cleaned.is_empty() || cleaned.starts_with("//") {
        return None;
    }

    let (lhs, rhs) = cleaned.split_once('=')?;
    let name = lhs.trim();
    if name.is_empty() {
        return None;
    }

    let input_expr_pos = rhs.find("input")?;
    let input_expr = rhs[input_expr_pos..].trim();
    let (kind, args) = parse_input_expr(input_expr)?;
    let parts = split_top_level_csv(&args);

    let mut keyed: HashMap<String, String> = HashMap::new();
    let mut default_raw: Option<String> = None;

    for part in parts {
        let item = part.trim();
        if item.is_empty() {
            continue;
        }
        if let Some((k, v)) = item.split_once('=') {
            keyed.insert(k.trim().to_ascii_lowercase(), v.trim().to_string());
        } else if default_raw.is_none() {
            default_raw = Some(item.to_string());
        }
    }

    let fallback_default = keyed.get("defval").cloned();
    let default_raw = default_raw.or(fallback_default);
    let default_num = default_raw.as_deref().and_then(parse_f64);
    let default_bool = default_raw.as_deref().and_then(parse_bool);
    let default_string = default_raw.as_deref().and_then(|v| {
        let out = unquote(v);
        if out.is_empty() {
            None
        } else {
            Some(out)
        }
    });

    let options = keyed
        .get("options")
        .map(|v| parse_options(v))
        .unwrap_or_default();

    Some(PineParam {
        name: name.to_string(),
        line: line_no,
        kind,
        title: keyed.get("title").map(|v| unquote(v)),
        default_raw,
        default_num,
        default_bool,
        default_string,
        min: keyed.get("minval").and_then(|v| parse_f64(v)),
        max: keyed.get("maxval").and_then(|v| parse_f64(v)),
        options,
    })
}

fn parse_input_expr(expr: &str) -> Option<(PineInputKind, String)> {
    if !expr.starts_with("input") {
        return None;
    }
    let mut kind = PineInputKind::Generic;
    if expr.starts_with("input.") {
        let suffix = &expr["input.".len()..];
        let open = suffix.find('(')?;
        let label = suffix[..open].trim().to_ascii_lowercase();
        kind = match label.as_str() {
            "int" => PineInputKind::Int,
            "float" => PineInputKind::Float,
            "bool" => PineInputKind::Bool,
            "string" => PineInputKind::String,
            "source" => PineInputKind::Source,
            _ => PineInputKind::Generic,
        };
    }

    let open = expr.find('(')?;
    let args = extract_parenthesized(expr, open)?;
    Some((kind, args))
}

fn extract_parenthesized(s: &str, open_idx: usize) -> Option<String> {
    let mut depth = 0_i32;
    let mut start = None;
    for (i, ch) in s.char_indices().skip(open_idx) {
        if ch == '(' {
            depth += 1;
            if start.is_none() {
                start = Some(i + 1);
            }
        } else if ch == ')' {
            depth -= 1;
            if depth == 0 {
                let st = start?;
                return Some(s[st..i].to_string());
            }
        }
    }
    None
}

fn split_top_level_csv(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut paren = 0_i32;
    let mut bracket = 0_i32;
    let mut brace = 0_i32;
    let mut in_single = false;
    let mut in_double = false;
    let mut prev = '\0';

    for ch in s.chars() {
        match ch {
            '\'' if !in_double && prev != '\\' => in_single = !in_single,
            '"' if !in_single && prev != '\\' => in_double = !in_double,
            '(' if !in_single && !in_double => paren += 1,
            ')' if !in_single && !in_double => paren -= 1,
            '[' if !in_single && !in_double => bracket += 1,
            ']' if !in_single && !in_double => bracket -= 1,
            '{' if !in_single && !in_double => brace += 1,
            '}' if !in_single && !in_double => brace -= 1,
            ',' if !in_single && !in_double && paren == 0 && bracket == 0 && brace == 0 => {
                out.push(current.trim().to_string());
                current.clear();
                prev = ch;
                continue;
            }
            _ => {}
        }
        current.push(ch);
        prev = ch;
    }

    if !current.trim().is_empty() {
        out.push(current.trim().to_string());
    }
    out
}

fn strip_inline_comment(line: &str) -> String {
    let mut out = String::new();
    let mut in_single = false;
    let mut in_double = false;
    let mut prev = '\0';
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];
        if ch == '\'' && !in_double && prev != '\\' {
            in_single = !in_single;
        } else if ch == '"' && !in_single && prev != '\\' {
            in_double = !in_double;
        }
        if !in_single && !in_double && ch == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
            break;
        }
        out.push(ch);
        prev = ch;
        i += 1;
    }
    out
}

fn parse_options(raw: &str) -> Vec<String> {
    let trimmed = raw.trim();
    let inner = trimmed
        .strip_prefix('[')
        .and_then(|v| v.strip_suffix(']'))
        .unwrap_or(trimmed);
    split_top_level_csv(inner)
        .into_iter()
        .map(|v| unquote(&v))
        .filter(|v| !v.is_empty())
        .collect()
}

fn parse_f64(raw: &str) -> Option<f64> {
    let txt = unquote(raw).replace('_', "");
    txt.parse::<f64>().ok()
}

fn parse_bool(raw: &str) -> Option<bool> {
    let t = raw.trim().to_ascii_lowercase();
    match t.as_str() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn unquote(raw: &str) -> String {
    let t = raw.trim();
    if (t.starts_with('"') && t.ends_with('"')) || (t.starts_with('\'') && t.ends_with('\'')) {
        return t[1..t.len() - 1].to_string();
    }
    t.to_string()
}

fn sample_param_value(param: &PineParam, rng: &mut StdRng) -> Value {
    if !param.options.is_empty() {
        let idx = rng.random_range(0..param.options.len());
        return Value::String(param.options[idx].clone());
    }

    match param.kind {
        PineInputKind::Int => {
            let def = param.default_num.unwrap_or(10.0);
            let min = param.min.unwrap_or((def * 0.5).floor().max(1.0));
            let max = param.max.unwrap_or((def * 1.5).ceil().max(min + 1.0));
            let span = (max - min).abs().max(1.0);
            let sampled = (def + rng.random_range(-0.35..=0.35) * span)
                .round()
                .clamp(min, max) as i64;
            Value::from(sampled)
        }
        PineInputKind::Float => {
            let def = param.default_num.unwrap_or(1.0);
            let min = param.min.unwrap_or((def * 0.5).min(def - 0.0001));
            let max = param.max.unwrap_or((def * 1.5).max(def + 0.0001));
            let span = (max - min).abs().max(0.0001);
            let sampled = (def + rng.random_range(-0.35..=0.35) * span).clamp(min, max);
            Value::from((sampled * 1_000_000.0).round() / 1_000_000.0)
        }
        PineInputKind::Bool => {
            let base = param.default_bool.unwrap_or(true);
            let flip = rng.random_range(0.0..1.0) < 0.25;
            Value::Bool(if flip { !base } else { base })
        }
        PineInputKind::String | PineInputKind::Source | PineInputKind::Generic => Value::String(
            param
                .default_string
                .clone()
                .or_else(|| param.default_raw.clone())
                .unwrap_or_default(),
        ),
    }
}

fn run_evaluator(
    cmd: &str,
    script_path: &str,
    params: &HashMap<String, Value>,
) -> Result<f64, String> {
    let payload = serde_json::to_string(params).map_err(|e| e.to_string())?;
    let output = Command::new("zsh")
        .arg("-lc")
        .arg(cmd)
        .env("PT_PINE_SCRIPT", script_path)
        .env("PT_PINE_CANDIDATE_JSON", payload)
        .output()
        .map_err(|e| format!("failed to start evaluator: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "evaluator failed with status {}: {}",
            output.status, stderr
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let score_line = stdout
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .ok_or_else(|| "evaluator produced no output score".to_string())?;
    score_line
        .trim()
        .parse::<f64>()
        .map_err(|e| format!("invalid evaluator score '{}': {e}", score_line.trim()))
}

fn make_rng(seed: Option<u64>) -> StdRng {
    if let Some(seed) = seed {
        StdRng::seed_from_u64(seed)
    } else {
        let mut trng = rand::rng();
        StdRng::from_rng(&mut trng)
    }
}

fn write_json_file<T: Serialize>(path: &str, value: &T) -> Result<(), String> {
    let out_path = PathBuf::from(path);
    ensure_parent_dir(&out_path)?;
    let body = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    fs::write(&out_path, body).map_err(|e| e.to_string())
}

fn write_text_file(path: &str, body: &str) -> Result<(), String> {
    let out_path = PathBuf::from(path);
    ensure_parent_dir(&out_path)?;
    fs::write(&out_path, body).map_err(|e| e.to_string())
}

fn ensure_parent_dir(path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn cmd_out(cmd: &str, args: &[&str]) -> String {
    match Command::new(cmd).args(args).output() {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).trim().to_string(),
        Ok(out) => format!("command failed: status {}", out.status),
        Err(e) => format!("unavailable ({e})"),
    }
}

fn now_unix() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::resolve_config_path;

    #[test]
    fn resolve_config_uses_env_when_default_is_passed() {
        std::env::set_var("PT_CONFIG_PATH", "config/alt.toml");
        let path = resolve_config_path("config/config.toml");
        assert_eq!(path, "config/alt.toml");
        std::env::remove_var("PT_CONFIG_PATH");
    }

    #[test]
    fn resolve_config_keeps_explicit_cli_path() {
        std::env::set_var("PT_CONFIG_PATH", "config/alt.toml");
        let path = resolve_config_path("config/custom.toml");
        assert_eq!(path, "config/custom.toml");
        std::env::remove_var("PT_CONFIG_PATH");
    }
}
