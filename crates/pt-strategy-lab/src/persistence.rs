use crate::types::{PaperEndpointReport, StrategyProfile, StrategyRunReport};
use pt_core::{PtError, PtResult};
use rusqlite::{params, Connection};

/// Write a JSON manifest for the run to `data/backtest/{run_id}.json`.
/// Non-fatal: silently skips on any I/O error.
pub fn save_run_manifest(report: &StrategyRunReport) {
    let dir = std::path::Path::new("data/backtest");
    if std::fs::create_dir_all(dir).is_err() {
        return;
    }
    let path = dir.join(format!("{}.json", report.run_id));
    if let Ok(json) = serde_json::to_string_pretty(report) {
        let _ = std::fs::write(path, json);
    }
}

fn open(db_path: &str) -> PtResult<Connection> {
    if let Some(parent) = std::path::Path::new(db_path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|e| PtError::Io(e.to_string()))?;
        }
    }
    let conn = Connection::open(db_path).map_err(|e| PtError::Io(e.to_string()))?;
    conn.pragma_update(None, "journal_mode", "WAL")
        .map_err(|e| PtError::Io(e.to_string()))?;
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS strategy_profiles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            profile_id TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            latest_version INTEGER NOT NULL,
            updated_ts_ms INTEGER NOT NULL
        );
        CREATE TABLE IF NOT EXISTS strategy_profile_versions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            profile_id TEXT NOT NULL,
            version INTEGER NOT NULL,
            created_ts_ms INTEGER NOT NULL,
            note TEXT,
            payload TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS strategy_runs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            run_id TEXT NOT NULL UNIQUE,
            profile_id TEXT NOT NULL,
            product_id TEXT NOT NULL,
            ts_ms INTEGER NOT NULL,
            total_return_pct REAL NOT NULL,
            max_drawdown_pct REAL NOT NULL,
            trades INTEGER NOT NULL,
            win_rate REAL NOT NULL,
            pnl REAL NOT NULL,
            payload TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS indicator_series (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            run_id TEXT NOT NULL,
            ts_ms INTEGER NOT NULL,
            indicator_name TEXT NOT NULL,
            bias REAL NOT NULL,
            confidence REAL NOT NULL,
            regime TEXT NOT NULL,
            payload TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS signal_series (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            run_id TEXT NOT NULL,
            ts_ms INTEGER NOT NULL,
            score REAL NOT NULL,
            action TEXT NOT NULL,
            confluence INTEGER NOT NULL,
            regime TEXT NOT NULL,
            payload TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS regime_series (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            run_id TEXT NOT NULL,
            ts_ms INTEGER NOT NULL,
            regime TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS paper_endpoint_reports (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ts_ms INTEGER NOT NULL,
            profile_id TEXT NOT NULL,
            simulated_orders INTEGER NOT NULL,
            simulated_edits INTEGER NOT NULL,
            simulated_cancel_replace INTEGER NOT NULL,
            estimated_reject_rate REAL NOT NULL,
            notes TEXT NOT NULL,
            payload TEXT NOT NULL
        );
        ",
    )
    .map_err(|e| PtError::Io(e.to_string()))?;
    Ok(conn)
}

pub fn save_profile(db_path: &str, profile: &StrategyProfile, note: Option<&str>) -> PtResult<()> {
    let conn = open(db_path)?;
    let now_ms = chrono::Utc::now().timestamp_millis();
    let payload = serde_json::to_string(profile).map_err(|e| PtError::Serde(e.to_string()))?;

    conn.execute(
        "INSERT INTO strategy_profiles (profile_id, name, latest_version, updated_ts_ms)
         VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(profile_id) DO UPDATE SET
         name=excluded.name,
         latest_version=excluded.latest_version,
         updated_ts_ms=excluded.updated_ts_ms",
        params![profile.profile_id, profile.name, profile.version, now_ms],
    )
    .map_err(|e| PtError::Io(e.to_string()))?;

    conn.execute(
        "INSERT INTO strategy_profile_versions (profile_id, version, created_ts_ms, note, payload)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            profile.profile_id,
            profile.version,
            now_ms,
            note.unwrap_or(""),
            payload
        ],
    )
    .map_err(|e| PtError::Io(e.to_string()))?;

    Ok(())
}

pub fn load_profile(db_path: &str, profile_id: &str) -> PtResult<StrategyProfile> {
    let conn = open(db_path)?;
    let payload: String = conn
        .query_row(
            "SELECT payload FROM strategy_profile_versions WHERE profile_id=?1 ORDER BY version DESC, created_ts_ms DESC LIMIT 1",
            params![profile_id],
            |row| row.get(0),
        )
        .map_err(|e| PtError::Io(e.to_string()))?;

    serde_json::from_str::<StrategyProfile>(&payload).map_err(|e| PtError::Serde(e.to_string()))
}

pub fn save_run(db_path: &str, report: &StrategyRunReport) -> PtResult<()> {
    let conn = open(db_path)?;
    let payload = serde_json::to_string(report).map_err(|e| PtError::Serde(e.to_string()))?;

    conn.execute(
        "INSERT OR REPLACE INTO strategy_runs (run_id, profile_id, product_id, ts_ms, total_return_pct, max_drawdown_pct, trades, win_rate, pnl, payload)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            report.run_id,
            report.profile_id,
            report.product_id,
            report.started_ts_ms,
            report.total_return_pct,
            report.max_drawdown_pct,
            report.trades as i64,
            report.win_rate,
            report.pnl,
            payload
        ],
    )
    .map_err(|e| PtError::Io(e.to_string()))?;

    for decision in &report.decisions {
        let payload = serde_json::to_string(decision).map_err(|e| PtError::Serde(e.to_string()))?;
        conn.execute(
            "INSERT INTO signal_series (run_id, ts_ms, score, action, confluence, regime, payload)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                report.run_id,
                decision.ts_ms,
                decision.score,
                format!("{:?}", decision.action).to_lowercase(),
                decision.confluence as i64,
                format!("{:?}", decision.regime).to_lowercase(),
                payload
            ],
        )
        .map_err(|e| PtError::Io(e.to_string()))?;

        conn.execute(
            "INSERT INTO regime_series (run_id, ts_ms, regime) VALUES (?1, ?2, ?3)",
            params![
                report.run_id,
                decision.ts_ms,
                format!("{:?}", decision.regime).to_lowercase(),
            ],
        )
        .map_err(|e| PtError::Io(e.to_string()))?;

        for indicator in &decision.indicators {
            let ipayload =
                serde_json::to_string(indicator).map_err(|e| PtError::Serde(e.to_string()))?;
            conn.execute(
                "INSERT INTO indicator_series (run_id, ts_ms, indicator_name, bias, confidence, regime, payload)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    report.run_id,
                    decision.ts_ms,
                    indicator.name,
                    indicator.bias,
                    indicator.confidence,
                    format!("{:?}", indicator.regime_vote).to_lowercase(),
                    ipayload,
                ],
            )
            .map_err(|e| PtError::Io(e.to_string()))?;
        }
    }

    Ok(())
}

pub fn list_runs(db_path: &str, limit: usize) -> PtResult<Vec<StrategyRunReport>> {
    let conn = open(db_path)?;
    let mut stmt = conn
        .prepare("SELECT payload FROM strategy_runs ORDER BY ts_ms DESC LIMIT ?1")
        .map_err(|e| PtError::Io(e.to_string()))?;

    let mut rows = stmt
        .query(params![limit as i64])
        .map_err(|e| PtError::Io(e.to_string()))?;

    let mut out = Vec::new();
    while let Some(row) = rows.next().map_err(|e| PtError::Io(e.to_string()))? {
        let payload: String = row.get(0).map_err(|e| PtError::Io(e.to_string()))?;
        if let Ok(report) = serde_json::from_str::<StrategyRunReport>(&payload) {
            out.push(report);
        }
    }
    Ok(out)
}

pub fn save_paper_endpoint_report(db_path: &str, report: &PaperEndpointReport) -> PtResult<()> {
    let conn = open(db_path)?;
    let payload = serde_json::to_string(report).map_err(|e| PtError::Serde(e.to_string()))?;
    conn.execute(
        "INSERT INTO paper_endpoint_reports (ts_ms, profile_id, simulated_orders, simulated_edits, simulated_cancel_replace, estimated_reject_rate, notes, payload)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            report.created_ts_ms,
            report.profile_id,
            report.simulated_orders as i64,
            report.simulated_edits as i64,
            report.simulated_cancel_replace as i64,
            report.estimated_reject_rate,
            report.notes,
            payload,
        ],
    )
    .map_err(|e| PtError::Io(e.to_string()))?;
    Ok(())
}
