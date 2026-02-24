use axum::{body::to_bytes, body::Body, http::Request, http::StatusCode, Router};
use chrono::Utc;
use parking_lot::RwLock;
use pt_core::{
    Asset, ExecutionReport, ExecutionStatus, KillSwitchState, MarketHistoryPoint, MarketSnapshot,
    MetricsRegistry, RiskState, Side, Venue,
};
use pt_dashboard::{router, DashboardState};
use serde_json::Value;
use std::{collections::HashMap, fs, sync::Arc};
use tower::util::ServiceExt;

fn fixture_state() -> DashboardState {
    let mut latest_books = HashMap::new();
    let now = Utc::now();
    let market_id = "mkt-1".to_string();
    latest_books.insert(
        market_id.clone(),
        MarketSnapshot {
            market_id: market_id.clone(),
            token_id: "token-yes-1".to_string(),
            bid: 0.49,
            ask: 0.50,
            spread: 0.01,
            liquidity: 10_000.0,
            ts: now,
        },
    );

    let mut history = HashMap::new();
    history.insert(
        market_id.clone(),
        vec![MarketHistoryPoint {
            market_id: market_id.clone(),
            mid: 0.495,
            spread: 0.01,
            bid: 0.49,
            ask: 0.50,
            ts: now,
        }],
    );

    let executions = vec![ExecutionReport {
        venue: Venue::Sim,
        order_id: "sim-1".to_string(),
        market_id: Some(market_id.clone()),
        status: ExecutionStatus::Filled,
        side: Side::Buy,
        filled_qty: 1.0,
        avg_px: 0.495,
        ts: now,
        details: Some("fixture".to_string()),
    }];

    let mut bias = HashMap::new();
    bias.insert(Asset::Btc, 0.2);

    DashboardState::new(
        Arc::new(MetricsRegistry::default()),
        Arc::new(RwLock::new(RiskState {
            killswitch: "Running".to_string(),
            daily_pnl: 1.25,
            max_daily_loss: 1.0,
            open_notional: 2.5,
            unhedged_delta: 0.5,
            open_markets: 1,
            stale_books: 0,
            last_update_ms: now.timestamp_millis(),
        })),
        Arc::new(RwLock::new(KillSwitchState::Running)),
        Arc::new(RwLock::new(latest_books)),
        Arc::new(RwLock::new(history)),
        Arc::new(RwLock::new(executions)),
        Arc::new(RwLock::new(bias)),
        Arc::new(RwLock::new(5.0)),
    )
}

fn load_openapi_doc() -> Value {
    let path = format!(
        "{}/../../docs/api/dashboard-openapi.yaml",
        env!("CARGO_MANIFEST_DIR")
    );
    let raw = fs::read_to_string(path).expect("read openapi file");
    serde_yaml::from_str::<Value>(&raw).expect("parse openapi yaml")
}

fn schema_from_path<'a>(doc: &'a Value, path: &str, method: &str) -> &'a Value {
    let entry = &doc["paths"][path][method]["responses"]["200"]["content"];
    let schema = if !entry["application/json"].is_null() {
        &entry["application/json"]["schema"]
    } else {
        panic!("{} {} is not application/json", method, path);
    };

    assert!(!schema.is_null(), "{} {} missing schema", method, path);
    schema
}

fn resolve_schema_ref<'a>(doc: &'a Value, schema: &'a Value) -> &'a Value {
    if let Some(reference) = schema["$ref"].as_str() {
        let prefix = "#/components/schemas/";
        let name = reference
            .strip_prefix(prefix)
            .unwrap_or_else(|| panic!("unsupported ref {}", reference));
        return &doc["components"]["schemas"][name];
    }
    schema
}

fn is_nullable(schema: &Value) -> bool {
    schema["nullable"].as_bool().unwrap_or(false)
}

fn validate_against_schema(doc: &Value, schema: &Value, payload: &Value, at: &str) {
    let resolved = resolve_schema_ref(doc, schema);

    if payload.is_null() {
        assert!(is_nullable(resolved), "{} should not be null", at);
        return;
    }

    if let Some(t) = resolved["type"].as_str() {
        match t {
            "object" => {
                let obj = payload
                    .as_object()
                    .unwrap_or_else(|| panic!("{} expected object", at));

                if let Some(required) = resolved["required"].as_array() {
                    for field in required {
                        let key = field
                            .as_str()
                            .unwrap_or_else(|| panic!("{} required field not string", at));
                        assert!(
                            obj.contains_key(key),
                            "{} missing required field '{}'",
                            at,
                            key
                        );
                    }
                }

                if let Some(props) = resolved["properties"].as_object() {
                    for (k, schema_prop) in props {
                        if let Some(v) = obj.get(k) {
                            validate_against_schema(doc, schema_prop, v, &format!("{}.{}", at, k));
                        }
                    }
                }
            }
            "array" => {
                let arr = payload
                    .as_array()
                    .unwrap_or_else(|| panic!("{} expected array", at));
                let item_schema = &resolved["items"];
                assert!(!item_schema.is_null(), "{} array missing items schema", at);
                for (idx, item) in arr.iter().enumerate() {
                    validate_against_schema(doc, item_schema, item, &format!("{}[{}]", at, idx));
                }
            }
            "string" => assert!(payload.is_string(), "{} expected string", at),
            "number" => assert!(payload.is_number(), "{} expected number", at),
            "integer" => assert!(payload.as_i64().is_some(), "{} expected integer", at),
            "boolean" => assert!(payload.is_boolean(), "{} expected boolean", at),
            other => panic!("{} unsupported type '{}'", at, other),
        }
    }
}

async fn assert_json_contract(
    app: Router,
    doc: &Value,
    method: &str,
    path: &str,
    request_uri: &str,
) {
    let req = Request::builder()
        .method(method)
        .uri(request_uri)
        .body(Body::empty())
        .expect("build request");

    let resp = app.oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::OK, "{} {} status", method, path);

    let body = to_bytes(resp.into_body(), usize::MAX)
        .await
        .expect("body bytes");
    let json: Value = serde_json::from_slice(&body).expect("json body");

    let schema = schema_from_path(doc, path, &method.to_ascii_lowercase());
    validate_against_schema(doc, schema, &json, path);
}

#[tokio::test]
async fn state_endpoints_match_openapi_contract() {
    let app = router(fixture_state());
    let doc = load_openapi_doc();

    let checks = [
        ("GET", "/health", "/health"),
        ("GET", "/healthz", "/healthz"),
        ("GET", "/ready", "/ready"),
        ("GET", "/state/risk", "/state/risk"),
        ("GET", "/state/books", "/state/books"),
        ("GET", "/state/markets", "/state/markets"),
        (
            "GET",
            "/state/history",
            "/state/history?market_id=mkt-1&limit=10",
        ),
        ("GET", "/state/executions", "/state/executions"),
        ("GET", "/state/bias", "/state/bias"),
        ("GET", "/state/inventory", "/state/inventory"),
        ("POST", "/ops/halt", "/ops/halt"),
        ("POST", "/ops/resume", "/ops/resume"),
        ("POST", "/ops/flatten", "/ops/flatten"),
    ];

    for (method, path, uri) in checks {
        assert_json_contract(app.clone(), &doc, method, path, uri).await;
    }
}

#[tokio::test]
async fn metrics_endpoint_returns_prometheus_text() {
    let app = router(fixture_state());
    let resp = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/metrics")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(resp.status(), StatusCode::OK);
    let body = to_bytes(resp.into_body(), usize::MAX).await.expect("bytes");
    let txt = String::from_utf8(body.to_vec()).expect("utf8");
    assert!(
        txt.is_empty() || txt.contains("# TYPE"),
        "unexpected metrics payload"
    );
}

#[test]
fn openapi_contains_all_runtime_paths() {
    let doc = load_openapi_doc();
    let required_paths = [
        "/health",
        "/healthz",
        "/ready",
        "/metrics",
        "/state/risk",
        "/state/books",
        "/state/markets",
        "/state/history",
        "/state/executions",
        "/state/bias",
        "/state/inventory",
        "/ops/halt",
        "/ops/resume",
        "/ops/flatten",
    ];

    for p in required_paths {
        assert!(!doc["paths"][p].is_null(), "missing OpenAPI path: {}", p);
    }
}
