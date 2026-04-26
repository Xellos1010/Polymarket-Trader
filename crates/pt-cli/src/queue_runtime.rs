use pt_core::{WorkstationOrder, WorkstationOrderStatus};
use std::collections::HashSet;

pub fn is_queue_relevant(order: &WorkstationOrder) -> bool {
    matches!(
        order.status,
        Some(WorkstationOrderStatus::Draft | WorkstationOrderStatus::CancelRequested)
    )
}

pub fn merge_persisted_queue_orders(
    existing_orders: &[WorkstationOrder],
    persisted_orders: &[WorkstationOrder],
) -> Vec<WorkstationOrder> {
    let mut merged = existing_orders
        .iter()
        .filter(|order| !is_queue_relevant(order))
        .cloned()
        .collect::<Vec<_>>();

    let mut seen_queue_keys = HashSet::new();
    for order in existing_orders.iter().chain(persisted_orders.iter()) {
        if !is_queue_relevant(order) {
            continue;
        }
        let key = queue_identity_key(order);
        if seen_queue_keys.insert(key) {
            merged.push(order.clone());
        }
    }

    merged.sort_by(|a, b| b.updated_at.cmp(&a.updated_at).then_with(|| b.created_at.cmp(&a.created_at)));
    merged
}

fn queue_identity_key(order: &WorkstationOrder) -> String {
    order
        .client_order_id
        .as_ref()
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("client:{value}"))
        .unwrap_or_else(|| format!("order:{}", order.order_id))
}

#[cfg(test)]
mod tests {
    use super::{is_queue_relevant, merge_persisted_queue_orders};
    use chrono::Utc;
    use pt_core::{OrderRoute, ProductId, Side, WorkstationOrder, WorkstationOrderStatus};

    fn sample_order(order_id: &str, client_order_id: Option<&str>, status: WorkstationOrderStatus) -> WorkstationOrder {
        let now = Utc::now();
        WorkstationOrder {
            order_id: order_id.to_string(),
            client_order_id: client_order_id.map(str::to_string),
            product_id: ProductId::from("BTC-USD"),
            instrument: None,
            side: Some(Side::Buy),
            route: Some(OrderRoute::Maker),
            status: Some(status),
            live: false,
            post_only: true,
            limit_price: Some(60_000.0),
            base_size: 0.01,
            quote_notional: 100.0,
            expected_net_bps: 6.5,
            reason: Some("test".to_string()),
            created_at: Some(now),
            updated_at: Some(now),
        }
    }

    #[test]
    fn queue_relevance_only_keeps_reviewable_statuses() {
        assert!(is_queue_relevant(&sample_order("draft-1", None, WorkstationOrderStatus::Draft)));
        assert!(is_queue_relevant(&sample_order(
            "cancel-1",
            None,
            WorkstationOrderStatus::CancelRequested
        )));
        assert!(!is_queue_relevant(&sample_order("open-1", None, WorkstationOrderStatus::Open)));
        assert!(!is_queue_relevant(&sample_order(
            "filled-1",
            None,
            WorkstationOrderStatus::Filled
        )));
    }

    #[test]
    fn merge_keeps_non_queue_orders_and_hydrates_persisted_queue_rows() {
        let merged = merge_persisted_queue_orders(
            &[sample_order("open-1", Some("open-1"), WorkstationOrderStatus::Open)],
            &[
                sample_order("draft-1", Some("draft-1"), WorkstationOrderStatus::Draft),
                sample_order(
                    "cancel-1",
                    Some("cancel-1"),
                    WorkstationOrderStatus::CancelRequested,
                ),
            ],
        );

        assert_eq!(merged.len(), 3);
        assert!(merged.iter().any(|order| order.order_id == "open-1"));
        assert!(merged.iter().any(|order| order.order_id == "draft-1"));
        assert!(merged.iter().any(|order| order.order_id == "cancel-1"));
    }

    #[test]
    fn merge_prefers_existing_queue_state_for_same_identity() {
        let mut existing = sample_order("draft-local", Some("client-1"), WorkstationOrderStatus::CancelRequested);
        existing.reason = Some("cancel requested from dashboard".to_string());

        let persisted = sample_order("draft-local", Some("client-1"), WorkstationOrderStatus::Draft);
        let merged = merge_persisted_queue_orders(&[existing.clone()], &[persisted]);

        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].status, Some(WorkstationOrderStatus::CancelRequested));
        assert_eq!(merged[0].reason.as_deref(), Some("cancel requested from dashboard"));
    }

    #[test]
    fn merge_deduplicates_identity_changes_by_client_order_id() {
        let persisted = sample_order("draft-local", Some("client-2"), WorkstationOrderStatus::Draft);
        let mut existing = sample_order("remote-123", Some("client-2"), WorkstationOrderStatus::CancelRequested);
        existing.reason = Some("already reloaded from runtime".to_string());

        let merged = merge_persisted_queue_orders(&[existing.clone()], &[persisted]);

        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].order_id, "remote-123");
        assert_eq!(merged[0].status, Some(WorkstationOrderStatus::CancelRequested));
    }
}
