pub mod backtest;
pub mod data;
pub mod indicators;
pub mod persistence;
pub mod signals;
pub mod tuning;
pub mod types;
pub mod web;

pub use backtest::run_backtest;
pub use data::fetch_coinbase_candles;
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
