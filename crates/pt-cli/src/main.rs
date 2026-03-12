use chrono::Utc;
use clap::{Parser, Subcommand};
use pt_core::{AppConfig, EngineMode, MarketSnapshot};
use pt_engine::TradingEngine;
use pt_market_discovery::MarketDiscoveryClient;
use pt_polymarket::PolymarketClient;
use pt_quote::{build_quote_intent, expected_net, CostInputs, QuoteConfig};
use rand::{rngs::StdRng, Rng, SeedableRng};
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
    PreflightLive {
        #[arg(long, default_value_t = 3000)]
        timeout_ms: u64,
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
    ScanMarkets {
        #[arg(long, default_value_t = 40)]
        limit: usize,
        #[arg(long, default_value_t = 10)]
        top: usize,
        #[arg(long, default_value_t = 0.003)]
        adverse_sel_est: f64,
        #[arg(long, default_value_t = 0.001)]
        hedge_cost_est: f64,
        #[arg(long, default_value_t = 0.0005)]
        gas_amortized_est: f64,
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
        Commands::PreflightLive { timeout_ms } => {
            if let Err(e) = preflight_live(&config_path, timeout_ms).await {
                error!(%e, "preflight failed");
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
        Commands::ScanMarkets {
            limit,
            top,
            adverse_sel_est,
            hedge_cost_est,
            gas_amortized_est,
        } => {
            if let Err(e) = scan_markets(
                &config_path,
                limit,
                top,
                adverse_sel_est,
                hedge_cost_est,
                gas_amortized_est,
            )
            .await
            {
                error!(%e, "market scan failed");
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

    checks.push(require_nonempty(
        "venues.polymarket.private_key",
        cfg.venues.polymarket.private_key.as_deref(),
    ));
    checks.push(require_nonempty(
        "venues.coinbase.api_key",
        cfg.venues.coinbase.api_key.as_deref(),
    ));
    checks.push(require_nonempty(
        "venues.coinbase.api_secret",
        cfg.venues.coinbase.api_secret.as_deref(),
    ));

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
    text.push_str("- Market list: `curl -s http://127.0.0.1:8080/state/markets | jq '.[0:5]'`\n");
    text.push_str(
        "- Market history: `curl -s \"http://127.0.0.1:8080/state/history?limit=120\" | jq`\n",
    );
    text.push_str("- Extract pine params: `cargo run -p pt-cli -- pine-params --path pine-scripts/<script> --out data/tuning/pine_params.json`\n");
    text.push_str("- Tune pine params: `cargo run -p pt-cli -- tune-pine --path pine-scripts/<script> --iterations 100 --evaluate-cmd \"python3 tools/evaluate_candidate.py\"`\n\n");
    text.push_str("- Promote tuning candidate: `./scripts/promote_candidate.sh data/tuning/pine_tuning_results.json data/tuning/promoted_candidate.json BTC 15m`\n");
    text.push_str("- Paper soak: `./scripts/paper_soak.sh 3600 30 config/config.toml`\n");
    text.push_str(
        "- Tiny live pilot checks: `./scripts/tiny_live_pilot.sh config/config.toml 3000`\n",
    );
    text.push_str("- Install git hooks: `./scripts/install_git_hooks.sh`\n\n");
    text.push_str("## Live Prerequisites\n");
    text.push_str("- Set `engine.mode = \"live\"` in config.\n");
    text.push_str("- Set `venues.polymarket.private_key` or `POLYMARKET_PRIVATE_KEY`.\n");
    text.push_str(
        "- Set `venues.coinbase.api_key`/`api_secret` or `COINBASE_API_KEY`/`COINBASE_API_SECRET`.\n",
    );
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

#[derive(Debug, Clone)]
struct MarketScanRow {
    market_id: String,
    slug: String,
    question: String,
    bid: f64,
    ask: f64,
    spread: f64,
    expected_net: f64,
    quote_bid: f64,
    quote_ask: f64,
    volume24h: f64,
    liquidity: f64,
    tier: String,
}

async fn scan_markets(
    config_path: &str,
    limit: usize,
    top: usize,
    adverse_sel_est: f64,
    hedge_cost_est: f64,
    gas_amortized_est: f64,
) -> Result<(), String> {
    let cfg = AppConfig::from_file(config_path).map_err(|e| e.to_string())?;
    let discovery = MarketDiscoveryClient::new(
        cfg.venues.polymarket.gamma_api.clone(),
        cfg.venues.polymarket.filters.clone(),
    );
    let polymarket = PolymarketClient::new(
        cfg.venues.polymarket.clob_api.clone(),
        cfg.venues.polymarket.clob_ws.clone(),
    );

    let mut markets = discovery
        .fetch_all_markets()
        .await
        .map_err(|e| format!("fetch_all_markets failed: {e}"))?;
    markets.truncate(limit.max(1));

    let quote_cfg = QuoteConfig {
        min_expected_net: cfg.risk.min_expected_net,
        ..QuoteConfig::default()
    };

    let mut rows: Vec<MarketScanRow> = Vec::new();

    for market in markets {
        let best = match polymarket.get_best_book(&market.token_id_yes).await {
            Ok(v) => v,
            Err(_) => continue,
        };

        let snap = MarketSnapshot {
            market_id: market.market_id.clone(),
            token_id: market.token_id_yes.clone(),
            bid: best.best_bid,
            ask: best.best_ask,
            spread: best.spread,
            liquidity: market.liquidity,
            ts: Utc::now(),
        };

        let costs = CostInputs {
            rebate_est: if market.fees_enabled { 0.001 } else { 0.0 },
            adverse_sel_est,
            hedge_cost_est,
            gas_amortized_est,
        };

        let exp = expected_net(
            snap.ask - snap.bid,
            costs.rebate_est,
            costs.adverse_sel_est,
            costs.hedge_cost_est,
            costs.gas_amortized_est,
        );

        let (quote_bid, quote_ask) =
            match build_quote_intent(&market, &snap, 0.0, 0.0, &costs, &quote_cfg) {
                Some(q) => (q.bid_px, q.ask_px),
                None => (0.0, 0.0),
            };

        rows.push(MarketScanRow {
            market_id: market.market_id,
            slug: market.slug,
            question: market.question,
            bid: snap.bid,
            ask: snap.ask,
            spread: snap.spread,
            expected_net: exp,
            quote_bid,
            quote_ask,
            volume24h: market.volume24h,
            liquidity: market.liquidity,
            tier: format!("{:?}", market.tier),
        });
    }

    rows.sort_by(|a, b| {
        b.expected_net
            .partial_cmp(&a.expected_net)
            .unwrap_or(Ordering::Equal)
    });

    let take_n = top.max(1).min(rows.len());
    println!(
        "scan_markets: showing top {} / {} opportunities (maker-focused expected_net)",
        take_n,
        rows.len()
    );
    println!(
        "{: <12} {: <11} {: >7} {: >7} {: >8} {: >10} {: >10} {: >10} {: >10} {: >10}",
        "market_id", "tier", "bid", "ask", "spread", "exp_net", "q_bid", "q_ask", "vol24h", "liq"
    );

    for r in rows.iter().take(take_n) {
        println!(
            "{: <12} {: <11} {: >7.4} {: >7.4} {: >8.4} {: >10.4} {: >10.4} {: >10.4} {: >10.0} {: >10.0}",
            truncate_id(&r.market_id),
            r.tier,
            r.bid,
            r.ask,
            r.spread,
            r.expected_net,
            r.quote_bid,
            r.quote_ask,
            r.volume24h,
            r.liquidity,
        );
        println!("    slug={} | question={}", r.slug, r.question);
    }

    Ok(())
}

fn truncate_id(v: &str) -> String {
    if v.len() <= 12 {
        return v.to_string();
    }
    format!("{}…", &v[..11])
}
