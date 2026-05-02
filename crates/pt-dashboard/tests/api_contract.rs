use axum::{body::to_bytes, body::Body, http::Request, http::StatusCode, Router};
use chrono::Utc;
use parking_lot::RwLock;
use pt_core::{
    Asset, ExecutionReport, ExecutionStatus, KillSwitchState, LiveArmState, MarketHistoryPoint,
    MarketSnapshot, MetricsRegistry, ProductDetailView, ProductId, ProductStrategyConfigView,
    RiskState, ScannerRow, Side, StrategyLabImportSummary, TradeAction, TradingEligibility, Venue,
    WorkstationOrder, WorkstationOrderStatus, WorkstationProduct,
};
use pt_dashboard::{router, CoinbaseDashboardHandles, DashboardHandles, DashboardState};
use serde_json::{json, Value};
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

    let coinbase = CoinbaseDashboardHandles::default();
    *coinbase.mode.write() = "paper".to_string();
    *coinbase.live_arm.write() = LiveArmState {
        armed: false,
        mode: Some("paper".to_string()),
        ..LiveArmState::default()
    };
    *coinbase.products.write() = vec![WorkstationProduct {
        product_id: ProductId::from("BTC-USD"),
        instrument: Some(pt_core::Instrument::Spot),
        base_currency: "BTC".to_string(),
        quote_currency: "USD".to_string(),
        status: "online".to_string(),
        price: 60_000.0,
        volume_24h: 1_000_000.0,
        live_tradable: true,
        scan_only: false,
        trading_disabled: false,
    }];
    *coinbase.scanner.write() = vec![ScannerRow {
        product_id: ProductId::from("BTC-USD"),
        instrument: Some(pt_core::Instrument::Spot),
        live_tradable: true,
        scan_only: false,
        spread_bps: 4.0,
        imbalance: 0.42,
        tape_direction: 0.35,
        realized_volatility: 0.12,
        fill_rate_estimate: 0.67,
        active_strategy: "coinbase_microstructure".to_string(),
        score: 0.88,
        current_risk_eligibility: TradingEligibility {
            product_id: ProductId::from("BTC-USD"),
            live_tradable: true,
            scan_only: false,
            eligible: true,
            reasons: Vec::new(),
        },
        best_bid: 59_990.0,
        best_ask: 60_010.0,
        mid_price: 60_000.0,
        action: Some(TradeAction::Buy),
        priority_fill: true,
        one_way_persistence: 4,
        ts: Some(now),
    }];
    *coinbase.orders.write() = vec![
        WorkstationOrder {
            order_id: "cb-order-1".to_string(),
            client_order_id: Some("client-1".to_string()),
            product_id: ProductId::from("BTC-USD"),
            instrument: Some(pt_core::Instrument::Spot),
            side: Some(Side::Buy),
            route: Some(pt_core::OrderRoute::Maker),
            status: Some(WorkstationOrderStatus::Open),
            live: false,
            post_only: true,
            limit_price: Some(59_990.0),
            base_size: 0.01,
            quote_notional: 250.0,
            expected_net_bps: 12.0,
            reason: Some("fixture".to_string()),
            created_at: Some(now),
            updated_at: Some(now),
        },
        WorkstationOrder {
            order_id: "manual-order-1".to_string(),
            client_order_id: Some("manual-client-1".to_string()),
            product_id: ProductId::from("BTC-USD"),
            instrument: Some(pt_core::Instrument::Spot),
            side: Some(Side::Sell),
            route: Some(pt_core::OrderRoute::Maker),
            status: Some(WorkstationOrderStatus::Draft),
            live: false,
            post_only: true,
            limit_price: Some(60_020.0),
            base_size: 0.01,
            quote_notional: 150.0,
            expected_net_bps: 8.0,
            reason: Some("manual review".to_string()),
            created_at: Some(now),
            updated_at: Some(now),
        },
    ];
    *coinbase.strategies.write() = vec![ProductStrategyConfigView {
        product_id: ProductId::from("BTC-USD"),
        strategy_name: "coinbase_microstructure".to_string(),
        enabled: true,
        live_enabled: true,
        score_threshold: 0.35,
        quote_size_usd: 25.0,
        plugin_signal: 0.1,
    }];
    *coinbase.imports.write() = vec![StrategyLabImportSummary {
        import_id: "import-1".to_string(),
        artifact_id: Some("artifact-1".to_string()),
        path: "data/strategy_lab/sample.json".to_string(),
        imported_at: Some(now),
        markets: vec!["BTC-USD".to_string()],
        best_variants: vec!["BTC-USD:sma_baseline".to_string()],
        source_run_id: Some("jr-fixture-1".to_string()),
        promotion_status: "imported_only".to_string(),
        replay_acceptance_status: None,
        objective_score: Some(0.42),
        confidence: None,
        timeframe: Some("300s_candles".to_string()),
    }];
    *coinbase.product_details.write() = HashMap::from([(
        "BTC-USD".to_string(),
        ProductDetailView {
            product: coinbase.products.read()[0].clone(),
            microstructure: pt_core::MarketMicrostructureSnapshot {
                product_id: ProductId::from("BTC-USD"),
                instrument: Some(pt_core::Instrument::Spot),
                best_bid: 59_990.0,
                best_ask: 60_010.0,
                mid_price: 60_000.0,
                spread_bps: 4.0,
                imbalance: 0.42,
                tape_direction: 0.35,
                realized_volatility: 0.12,
                fill_rate_estimate: 0.67,
                one_way_persistence: 4,
                ts: Some(now),
            },
            strategy: pt_core::StrategyVector {
                product_id: ProductId::from("BTC-USD"),
                strategy_name: "coinbase_microstructure".to_string(),
                microstructure_score: 0.42,
                momentum_score: 0.35,
                volatility_score: -0.12,
                plugin_score: 0.1,
                composite_score: 0.88,
                action: Some(TradeAction::Buy),
                priority_fill: true,
            },
            eligibility: TradingEligibility {
                product_id: ProductId::from("BTC-USD"),
                live_tradable: true,
                scan_only: false,
                eligible: true,
                reasons: Vec::new(),
            },
            orders: coinbase.orders.read().clone(),
            imports: coinbase.imports.read().clone(),
        },
    )]);

    DashboardState::new(DashboardHandles {
        metrics: Arc::new(MetricsRegistry::default()),
        risk_state: Arc::new(RwLock::new(RiskState {
            killswitch: "Running".to_string(),
            daily_pnl: 1.25,
            max_daily_loss: 1.0,
            open_notional: 2.5,
            unhedged_delta: 0.5,
            open_markets: 1,
            stale_books: 0,
            last_update_ms: now.timestamp_millis(),
        })),
        kill_switch: Arc::new(RwLock::new(KillSwitchState::Running)),
        latest_books: Arc::new(RwLock::new(latest_books)),
        market_history: Arc::new(RwLock::new(history)),
        recent_executions: Arc::new(RwLock::new(executions)),
        fused_bias: Arc::new(RwLock::new(bias)),
        inventory_usd: Arc::new(RwLock::new(5.0)),
        coinbase,
    })
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
    body: Option<Value>,
) {
    let mut builder = Request::builder().method(method).uri(request_uri);
    if body.is_some() {
        builder = builder.header("content-type", "application/json");
    }
    let req = builder
        .body(match body {
            Some(b) => Body::from(b.to_string()),
            None => Body::empty(),
        })
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
    let import_fixture = std::env::temp_dir().join("pt-dashboard-import.json");
    fs::write(
        &import_fixture,
        r#"{"artifact_id":"lab-artifact-contract-1","meta":{"granularity_sec":300,"journal_run_id":"jr-contract-1"},"markets":{"BTC-USD":{"default_variant":"sma_baseline","variants":{"sma_baseline":{"metrics":{"sharpe_like":1.25,"total_return":0.05}}}}}}"#,
    )
    .expect("write import fixture");
    let import_body = format!(r#"{{"path":"{}"}}"#, import_fixture.display());
    let import_json: Value = serde_json::from_str(&import_body).expect("import request json");

    let checks = [
        ("GET", "/health", "/health", None),
        ("GET", "/healthz", "/healthz", None),
        ("GET", "/ready", "/ready", None),
        ("GET", "/state/risk", "/state/risk", None),
        ("GET", "/state/books", "/state/books", None),
        ("GET", "/state/markets", "/state/markets", None),
        (
            "GET",
            "/state/history",
            "/state/history?market_id=mkt-1&limit=10",
            None,
        ),
        ("GET", "/state/executions", "/state/executions", None),
        ("GET", "/state/bias", "/state/bias", None),
        ("GET", "/state/inventory", "/state/inventory", None),
        ("POST", "/ops/halt", "/ops/halt", None),
        ("POST", "/ops/resume", "/ops/resume", None),
        ("POST", "/ops/flatten", "/ops/flatten", None),
        ("GET", "/api/v1/products", "/api/v1/products", None),
        ("GET", "/api/v1/scanner", "/api/v1/scanner", None),
        (
            "GET",
            "/api/v1/products/{product_id}",
            "/api/v1/products/BTC-USD",
            None,
        ),
        ("GET", "/api/v1/orders", "/api/v1/orders", None),
        ("GET", "/api/v1/strategies", "/api/v1/strategies", None),
        (
            "POST",
            "/api/v1/mode",
            "/api/v1/mode",
            Some(json!({"mode": "paper"})),
        ),
        (
            "POST",
            "/api/v1/live/arm",
            "/api/v1/live/arm",
            Some(json!({"reason": "test"})),
        ),
        (
            "POST",
            "/api/v1/live/disarm",
            "/api/v1/live/disarm",
            Some(json!({"reason": "test"})),
        ),
        (
            "POST",
            "/api/v1/orders/{order_id}/cancel",
            "/api/v1/orders/cb-order-1/cancel",
            None,
        ),
        (
            "POST",
            "/api/v1/strategy-lab/import",
            "/api/v1/strategy-lab/import",
            Some(import_json),
        ),
    ];

    for (method, path, uri, body) in checks {
        assert_json_contract(app.clone(), &doc, method, path, uri, body).await;
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
        "/api/v1/products",
        "/api/v1/scanner",
        "/api/v1/products/{product_id}",
        "/api/v1/orders",
        "/api/v1/strategies",
        "/api/v1/mode",
        "/api/v1/live/arm",
        "/api/v1/live/disarm",
        "/api/v1/orders/{order_id}/cancel",
        "/api/v1/strategy-lab/import",
    ];

    for p in required_paths {
        assert!(!doc["paths"][p].is_null(), "missing OpenAPI path: {}", p);
    }
}
