pub mod backtest;
pub mod data;
pub mod indicators;
pub mod ir;
pub mod ir_adapter;
pub mod ir_exec;
pub mod persistence;
pub mod signals;
pub mod tuning;
pub mod types;
pub mod web;

pub use backtest::run_backtest;
pub use data::fetch_coinbase_candles;
pub use ir::{
    CompareNode, InputNode, IrAction, IrDecision, MaKind, RuleNode, SizingHint, StrategyIrDef,
    IR_VERSION,
};
pub use ir_adapter::{from_profile, from_promotion_json};
pub use ir_exec::eval_ir;
pub use persistence::{
    list_runs, load_profile, save_paper_endpoint_report, save_profile, save_run,
};
pub use signals::build_decisions;
pub use tuning::optimize_random_walk_forward;
pub use types::*;
pub use web::{
    router, serve, BacktestRunRequest, LoadProfileRequest, OptimizeRequest, SaveProfileRequest,
    StrategyLabState,
};
