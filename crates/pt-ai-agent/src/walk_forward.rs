use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalkForwardWindow {
    pub index: usize,
    pub train_start: DateTime<Utc>,
    pub train_end: DateTime<Utc>,
    pub holdout_start: DateTime<Utc>,
    pub holdout_end: DateTime<Utc>,
    pub artifact_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalkForwardPlan {
    pub strategy_id: String,
    pub source_run_id: String,
    pub windows: Vec<WalkForwardWindow>,
}

pub fn build_walk_forward_plan(
    strategy_id: impl Into<String>,
    source_run_id: impl Into<String> + Clone,
    start: DateTime<Utc>,
    train_span: Duration,
    holdout_span: Duration,
    step_span: Duration,
    max_windows: usize,
) -> WalkForwardPlan {
    let strategy_id = strategy_id.into();
    let source_run_id_value = source_run_id.clone().into();
    let mut windows = Vec::new();
    let mut cursor = start;

    for index in 0..max_windows {
        let train_start = cursor;
        let train_end = train_start + train_span;
        let holdout_start = train_end;
        let holdout_end = holdout_start + holdout_span;
        windows.push(WalkForwardWindow {
            index,
            train_start,
            train_end,
            holdout_start,
            holdout_end,
            artifact_id: format!("{strategy_id}:{source_run_id_value}:wf:{index}"),
        });
        cursor += step_span;
    }

    WalkForwardPlan {
        strategy_id,
        source_run_id: source_run_id_value,
        windows,
    }
}

impl WalkForwardPlan {
    pub fn is_valid(&self) -> bool {
        !self.windows.is_empty()
            && self.windows.iter().all(|w| {
                w.train_start < w.train_end
                    && w.train_end <= w.holdout_start
                    && w.holdout_start < w.holdout_end
                    && !w.artifact_id.is_empty()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walk_forward_plan_is_deterministic_and_artifact_linked() {
        let start = DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let plan = build_walk_forward_plan(
            "strategy-a",
            "run-1",
            start,
            Duration::days(30),
            Duration::days(7),
            Duration::days(7),
            3,
        );
        assert!(plan.is_valid());
        assert_eq!(plan.windows.len(), 3);
        assert_eq!(plan.windows[0].artifact_id, "strategy-a:run-1:wf:0");
        assert_eq!(plan.windows[1].train_start, start + Duration::days(7));
    }
}
