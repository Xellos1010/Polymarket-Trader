use axum::{body::to_bytes, body::Body, http::Request, http::StatusCode, Router};
use chrono::Utc;
use parking_lot::RwLock;
use pt_coinbase::CoinbaseOrderSummary;
use pt_core::{
    AllocationDrift, ApprovalToken, Asset, CapitalLedgerEntry, CapitalTierRule,
    CoinbaseOrderBookState, EdgeProfile, EngineMode, EntryExitVector, EquityPaperRun,
    EquityProductSnapshot, ExecutionCostAttribution, ExecutionEvent, ExecutionMode,
    ExecutionPolicy, ExecutionReport, ExecutionStatus, KillSwitchState, MarketHistoryPoint,
    MarketSnapshot, MetricsRegistry, OrderLifecycleState, RebalanceIntent, RebalancePlan,
    RebalancePlanStatus, RiskState, RouteExecutionPlan, RouteOpportunity, Side, StrategyClass,
    UiMode, Venue, VenueCapability, VenueFeeSchedule, WalletBalance,
};
use pt_dashboard::{router, DashboardState};
use serde_json::{json, Value};
    Asset, ExecutionReport, ExecutionStatus, KillSwitchState, LiveArmState, MarketHistoryPoint,
    MarketSnapshot, MetricsRegistry, ProductDetailView, ProductId, ProductStrategyConfigView,
    RiskState, ScannerRow, Side, StrategyLabImportSummary, TradeAction, TradingEligibility,
    WorkstationOrder, WorkstationOrderStatus, WorkstationProduct, Venue,
};
use pt_dashboard::{router, CoinbaseDashboardHandles, DashboardHandles, DashboardState};
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

    let execution_events = vec![ExecutionEvent {
        order_id: "sim-1".to_string(),
        venue: Venue::Sim,
        market_id: Some(market_id.clone()),
        product_id: Some("BTC-USD".to_string()),
        side: Side::Buy,
        state: OrderLifecycleState::Filled,
        qty: 1.0,
        price: 0.495,
        ts: now,
        details: Some("fixture".to_string()),
        reason_code: Some("fixture".to_string()),
        unwind_flag: false,
    }];

    let execution_costs = vec![ExecutionCostAttribution {
        execution_id: "sim-1".to_string(),
        venue: Venue::Sim,
        market_id: Some(market_id.clone()),
        side: Side::Buy,
        qty: 1.0,
        avg_px: 0.495,
        reference_px: 0.494,
        fee_bps: 0.0,
        fee_est: 0.0,
        slippage_bps: 1.0,
        slippage_est: 0.01,
        rebate_bps_est: 0.0,
        rebate_est: 0.0,
        effective_edge: 0.01,
        ts: now,
        strategy_class: Some(StrategyClass::MakerMmSpot),
        route_id: None,
    }];

    let execution_policy = ExecutionPolicy {
        mode: ExecutionMode::MakerFirst,
        allow_taker_on_unwind_only: true,
        post_only: true,
        cancel_replace_cooldown_ms: 250,
        min_rest_ms: 400,
        stale_book_ms: 400,
        vectors: EntryExitVector {
            entry_max_slippage_bps: 8.0,
            exit_max_slippage_bps: 10.0,
            entry_offset_bps: 2.0,
            exit_offset_bps: 2.0,
            max_cross_bps_unwind: 20.0,
        },
        coinbase_fees: VenueFeeSchedule {
            maker_bps: 6.0,
            taker_bps: 12.0,
            rebate_bps_est: 0.0,
        },
        kraken_fees: VenueFeeSchedule {
            maker_bps: 16.0,
            taker_bps: 26.0,
            rebate_bps_est: 0.0,
        },
        gemini_fees: VenueFeeSchedule {
            maker_bps: 20.0,
            taker_bps: 35.0,
            rebate_bps_est: 0.0,
        },
        polymarket_fees: VenueFeeSchedule {
            maker_bps: 0.0,
            taker_bps: 5.0,
            rebate_bps_est: 1.0,
        },
        edge_profiles: EdgeProfile {
            maker_mm_spot_min_bps: 8.0,
            conversion_cycle_min_bps: 100.0,
            position_reentry_min_bps: 40.0,
            per_asset_overrides_bps: HashMap::new(),
        },
    };

    let wallet_balances = vec![WalletBalance {
        venue: Venue::Coinbase,
        account_id: "acc-1".to_string(),
        asset: "BTC".to_string(),
        available: 0.1,
        hold: 0.0,
        usd_value: 5000.0,
        ts: now,
    }];

    let wallet_drifts = vec![AllocationDrift {
        asset: "BTC".to_string(),
        current_weight: 0.30,
        target_weight: 0.25,
        drift_weight: 0.05,
        current_usd: 5000.0,
        target_usd: 4200.0,
        drift_usd: 800.0,
    }];

    let wallet_open_orders = vec![CoinbaseOrderSummary {
        order_id: "cb-1".to_string(),
        product_id: "BTC-USD".to_string(),
        side: "BUY".to_string(),
        status: "OPEN".to_string(),
        order_type: "LIMIT".to_string(),
        average_filled_price: "0".to_string(),
        filled_size: "0".to_string(),
        order_configuration: json!({
            "limit_limit_gtc": {
                "base_size": "0.001",
                "limit_price": "50000.00",
                "post_only": true
            }
        }),
        created_time: now.to_rfc3339(),
        last_update_time: now.to_rfc3339(),
    }];

    let mut coinbase_orderbooks = HashMap::new();
    coinbase_orderbooks.insert(
        "BTC-USD".to_string(),
        CoinbaseOrderBookState {
            product_id: "BTC-USD".to_string(),
            sequence_num: 12,
            bids: vec![(50_000.0, 0.15)],
            asks: vec![(50_001.0, 0.17)],
            last_event_ts: Some(now),
        },
    );

    let route_opportunities = vec![RouteOpportunity {
        route_id: "route-1".to_string(),
        legs: vec![],
        gross_edge_bps: 110.0,
        expected_net_bps: 101.0,
        expected_usd_profit: 0.20,
        capital_required_usd: 2.5,
        strategy_class: StrategyClass::ConversionCycle,
        ts: now,
    }];

    let route_executions = vec![RouteExecutionPlan {
        route_id: "route-1".to_string(),
        legs: vec![],
        approved: false,
        reason: Some("assist_mode_route_candidate".to_string()),
        ts: now,
    }];

    let fee_summary = Some(pt_coinbase::CoinbaseTransactionSummary {
        total_fees: 1.25,
        maker_fee_rate: Some("0.0006".to_string()),
        taker_fee_rate: Some("0.0012".to_string()),
        raw: json!({"fee_tier":{"maker_fee_rate":"0.0006","taker_fee_rate":"0.0012"}}),
    });

    let rebalance_plan = Some(RebalancePlan {
        plan_id: "plan-1".to_string(),
        status: RebalancePlanStatus::Planned,
        intents: vec![RebalanceIntent {
            intent_id: "intent-1".to_string(),
            product_id: "BTC-USD".to_string(),
            asset: Asset::Btc,
            side: Side::Sell,
            usd_notional: 25.0,
            limit_price: 50_000.0,
            max_slippage_bps: 10.0,
        }],
        drifts: wallet_drifts.clone(),
        total_drift_abs_usd: 800.0,
        created_ts: now,
        expires_ts: now + chrono::Duration::seconds(300),
    });

    let rebalance_approval = Some(ApprovalToken {
        token_id: "token-1".to_string(),
        plan_id: "plan-1".to_string(),
        approved: false,
        created_ts: now,
        expires_ts: now + chrono::Duration::seconds(300),
    });

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
    *coinbase.orders.write() = vec![WorkstationOrder {
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
    }];
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
        path: "data/strategy_lab/sample.json".to_string(),
        imported_at: Some(now),
        markets: vec!["BTC-USD".to_string()],
        best_variants: vec!["BTC-USD:sma_baseline".to_string()],
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
        Arc::new(RwLock::new(KillSwitchState::Running)),
        Arc::new(RwLock::new(Vec::new())),
        Arc::new(RwLock::new(latest_books)),
        Arc::new(RwLock::new(history)),
        Arc::new(RwLock::new(executions)),
        Arc::new(RwLock::new(execution_events)),
        Arc::new(RwLock::new(execution_costs)),
        Arc::new(RwLock::new(execution_policy)),
        Arc::new(RwLock::new(bias)),
        Arc::new(RwLock::new(5.0)),
        Arc::new(RwLock::new(wallet_balances)),
        Arc::new(RwLock::new(wallet_drifts)),
        Arc::new(RwLock::new(wallet_open_orders)),
        Arc::new(RwLock::new(coinbase_orderbooks)),
        Arc::new(RwLock::new(route_opportunities)),
        Arc::new(RwLock::new(route_executions)),
        Arc::new(RwLock::new(vec![
            VenueCapability {
                venue: Venue::Coinbase,
                supports_post_only: true,
                supports_amend: true,
                supports_fix: false,
                min_tick: 0.01,
                min_size: 0.00000001,
                fee_model: "tiered_maker_taker".to_string(),
            },
            VenueCapability {
                venue: Venue::Polymarket,
                supports_post_only: true,
                supports_amend: false,
                supports_fix: false,
                min_tick: 0.01,
                min_size: 1.0,
                fee_model: "maker_reward_taker_fee".to_string(),
            },
        ])),
        Arc::new(RwLock::new(fee_summary)),
        Arc::new(RwLock::new(rebalance_plan)),
        Arc::new(RwLock::new(rebalance_approval)),
        Arc::new(RwLock::new(false)),
        None,
        None,
        vec!["BTC-USD".to_string()],
        EngineMode::Paper,
        "default".to_string(),
        Arc::new(RwLock::new(UiMode::Basic)),
        true,
        10.0,
        vec![
            CapitalTierRule {
                min_equity_usd: 0.0,
                reserve_pct: 0.0,
            },
            CapitalTierRule {
                min_equity_usd: 250.0,
                reserve_pct: 0.20,
            },
            CapitalTierRule {
                min_equity_usd: 500.0,
                reserve_pct: 0.30,
            },
        ],
        Arc::new(RwLock::new(Vec::<CapitalLedgerEntry>::new())),
        Arc::new(RwLock::new(Vec::<EquityProductSnapshot>::new())),
        Arc::new(RwLock::new(Vec::<EquityPaperRun>::new())),
    )
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
    let builder = Request::builder().method(method).uri(request_uri);
    let req = if let Some(body) = body {
        builder
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .expect("build request")
    } else {
        builder.body(Body::empty()).expect("build request")
    };
    request_body: Option<&str>,
) {
    let mut builder = Request::builder().method(method).uri(request_uri);
    if request_body.is_some() {
        builder = builder.header("content-type", "application/json");
    }
    let req = builder
        .body(match request_body {
            Some(body) => Body::from(body.to_string()),
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
        r#"{"markets":{"BTC-USD":{"default_variant":"sma_baseline"}}}"#,
    )
    .expect("write import fixture");
    let import_body = format!(r#"{{"path":"{}"}}"#, import_fixture.display());

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
        (
            "GET",
            "/state/execution/orders",
            "/state/execution/orders",
            None,
        ),
        (
            "GET",
            "/state/execution/costs",
            "/state/execution/costs",
            None,
        ),
        (
            "GET",
            "/state/execution/vectors",
            "/state/execution/vectors",
            None,
        ),
        ("GET", "/state/bias", "/state/bias", None),
        ("GET", "/state/inventory", "/state/inventory", None),
        (
            "GET",
            "/state/coinbase/wallet",
            "/state/coinbase/wallet",
            None,
        ),
        (
            "GET",
            "/state/coinbase/allocations",
            "/state/coinbase/allocations",
            None,
        ),
        (
            "GET",
            "/state/coinbase/rebalance-plan",
            "/state/coinbase/rebalance-plan",
            None,
        ),
        (
            "GET",
            "/state/coinbase/orderbook",
            "/state/coinbase/orderbook",
            None,
        ),
        ("GET", "/state/coinbase/auth", "/state/coinbase/auth", None),
        (
            "GET",
            "/state/coinbase/orders",
            "/state/coinbase/orders",
            None,
        ),
        (
            "GET",
            "/state/routes/opportunities",
            "/state/routes/opportunities",
            None,
        ),
        (
            "GET",
            "/state/routes/executions",
            "/state/routes/executions",
            None,
        ),
        ("GET", "/state/fees/summary", "/state/fees/summary", None),
        (
            "GET",
            "/state/listings/l2-archive",
            "/state/listings/l2-archive",
            None,
        ),
        ("GET", "/state/feed/health", "/state/feed/health", None),
        (
            "GET",
            "/state/feed/diagnostics",
            "/state/feed/diagnostics",
            None,
        ),
        (
            "GET",
            "/state/parity/monitor",
            "/state/parity/monitor",
            None,
        ),
        (
            "POST",
            "/state/parity/export-csv",
            "/state/parity/export-csv",
            Some(json!({"limit": 100, "include_failures": true})),
        ),
        (
            "GET",
            "/state/venues/capabilities",
            "/state/venues/capabilities",
            None,
        ),
        (
            "GET",
            "/state/venues/latency",
            "/state/venues/latency",
            None,
        ),
        (
            "GET",
            "/state/venues/fill-quality",
            "/state/venues/fill-quality",
            None,
        ),
        (
            "GET",
            "/state/venues/rejects",
            "/state/venues/rejects",
            None,
        ),
        (
            "GET",
            "/state/wallet-intel/coinbase",
            "/state/wallet-intel/coinbase",
            None,
        ),
        (
            "GET",
            "/state/wallet-intel/polymarket",
            "/state/wallet-intel/polymarket",
            None,
        ),
        (
            "GET",
            "/state/wallet-intel/leaderboard",
            "/state/wallet-intel/leaderboard",
            None,
        ),
        (
            "POST",
            "/state/routes/export-csv",
            "/state/routes/export-csv",
            Some(json!({"limit": 100, "min_expected_net_bps": 0.0})),
        ),
        (
            "POST",
            "/state/wallet-intel/export-csv",
            "/state/wallet-intel/export-csv",
            Some(json!({"source": "all", "limit": 100})),
        ),
        ("POST", "/ops/halt", "/ops/halt", None),
        ("POST", "/ops/resume", "/ops/resume", None),
        ("POST", "/ops/flatten", "/ops/flatten", None),
        (
            "POST",
            "/ops/profile/pilot-ultra-tight",
            "/ops/profile/pilot-ultra-tight",
            None,
        ),
        (
            "POST",
            "/ops/coinbase/rebalance/reject",
            "/ops/coinbase/rebalance/reject",
            None,
        ),
        (
            "POST",
            "/ops/coinbase/auth/reload",
            "/ops/coinbase/auth/reload",
            None,
        ),
        (
            "POST",
            "/ops/coinbase/auth/switch-profile",
            "/ops/coinbase/auth/switch-profile",
            Some(json!({ "profile_id": "primary" })),
        ),
        (
            "POST",
            "/ops/execution/unwind",
            "/ops/execution/unwind",
            None,
        ),
        ("POST", "/ops/unwind/now", "/ops/unwind/now", None),
        (
            "POST",
            "/ops/coinbase/rebalance/approve",
            "/ops/coinbase/rebalance/approve",
            Some(json!({ "token_id": "token-1" })),
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
        ("GET", "/api/v1/listings", "/api/v1/listings", None),
        (
            "GET",
            "/api/v1/listings/{product_id}",
            "/api/v1/listings/BTC-USD",
            None,
        ),
        ("GET", "/api/v1/risk/overview", "/api/v1/risk/overview", None),
        ("GET", "/api/v1/agent/console", "/api/v1/agent/console", None),
        ("GET", "/api/v1/orders", "/api/v1/orders", None),
        ("GET", "/api/v1/strategies", "/api/v1/strategies", None),
        (
            "POST",
            "/api/v1/mode",
            "/api/v1/mode",
            Some("{\"mode\":\"paper\"}"),
        ),
        (
            "POST",
            "/api/v1/live/arm",
            "/api/v1/live/arm",
            Some("{\"reason\":\"test\"}"),
        ),
        (
            "POST",
            "/api/v1/live/disarm",
            "/api/v1/live/disarm",
            Some("{\"reason\":\"test\"}"),
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
            Some(import_body.as_str()),
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
        "/state/execution/orders",
        "/state/execution/costs",
        "/state/execution/vectors",
        "/state/bias",
        "/state/inventory",
        "/state/coinbase/wallet",
        "/state/coinbase/allocations",
        "/state/coinbase/rebalance-plan",
        "/state/coinbase/orderbook",
        "/state/coinbase/auth",
        "/state/coinbase/orders",
        "/state/routes/opportunities",
        "/state/routes/executions",
        "/state/fees/summary",
        "/state/listings/candidates",
        "/state/listings/overlay",
        "/state/listings/l2-archive",
        "/state/feed/health",
        "/state/feed/diagnostics",
        "/state/parity/monitor",
        "/state/parity/export-csv",
        "/state/venues/capabilities",
        "/state/venues/latency",
        "/state/venues/fill-quality",
        "/state/venues/rejects",
        "/state/routes/export-csv",
        "/state/wallet-intel/coinbase",
        "/state/wallet-intel/polymarket",
        "/state/wallet-intel/leaderboard",
        "/state/wallet-intel/export-csv",
        "/ops/halt",
        "/ops/resume",
        "/ops/flatten",
        "/ops/coinbase/rebalance/approve",
        "/ops/coinbase/rebalance/reject",
        "/ops/coinbase/auth/reload",
        "/ops/coinbase/auth/switch-profile",
        "/ops/execution/unwind",
        "/api/v1/products",
        "/api/v1/scanner",
        "/api/v1/products/{product_id}",
        "/api/v1/listings",
        "/api/v1/listings/{product_id}",
        "/api/v1/risk/overview",
        "/api/v1/agent/console",
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
