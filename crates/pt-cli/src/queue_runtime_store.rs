use crate::queue_runtime::merge_persisted_queue_orders;
use crate::queue_store::ApprovalQueueStore;
use pt_core::WorkstationOrder;

pub fn hydrate_runtime_orders(
    store: &ApprovalQueueStore,
    existing_orders: &[WorkstationOrder],
) -> Result<Vec<WorkstationOrder>, String> {
    let persisted_orders = store.load_orders()?;
    Ok(merge_persisted_queue_orders(
        existing_orders,
        &persisted_orders,
    ))
}

pub fn reconcile_runtime_orders(
    store: &ApprovalQueueStore,
    current_orders: &[WorkstationOrder],
) -> Result<(), String> {
    store.replace_orders(current_orders)
}

#[cfg(test)]
mod tests {
    use super::{hydrate_runtime_orders, reconcile_runtime_orders};
    use crate::queue_store::ApprovalQueueStore;
    use chrono::Utc;
    use pt_core::{OrderRoute, ProductId, Side, WorkstationOrder, WorkstationOrderStatus};
    use std::{env, fs, path::PathBuf};

    fn temp_sqlite_path(name: &str) -> PathBuf {
        env::temp_dir().join(format!(
            "pt-approval-queue-runtime-{name}-{}-{}.sqlite",
            std::process::id(),
            Utc::now().timestamp_nanos_opt().unwrap_or_default()
        ))
    }

    fn sample_order(
        order_id: &str,
        client_order_id: Option<&str>,
        status: WorkstationOrderStatus,
    ) -> WorkstationOrder {
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
    fn hydrate_runtime_orders_merges_persisted_queue_rows_into_runtime_state() {
        let path = temp_sqlite_path("hydrate");
        let store = ApprovalQueueStore::open(path.to_str().expect("path")).expect("open store");
        store
            .sync_orders(&[
                sample_order("draft-1", Some("draft-1"), WorkstationOrderStatus::Draft),
                sample_order(
                    "cancel-1",
                    Some("cancel-1"),
                    WorkstationOrderStatus::CancelRequested,
                ),
            ])
            .expect("persist queue rows");

        let merged = hydrate_runtime_orders(
            &store,
            &[sample_order(
                "open-1",
                Some("open-1"),
                WorkstationOrderStatus::Open,
            )],
        )
        .expect("hydrate runtime orders");

        assert_eq!(merged.len(), 3);
        assert!(merged.iter().any(|order| order.order_id == "open-1"));
        assert!(merged.iter().any(|order| order.order_id == "draft-1"));
        assert!(merged.iter().any(|order| order.order_id == "cancel-1"));

        let _ = fs::remove_file(path);
    }

    #[test]
    fn reconcile_runtime_orders_prunes_stale_rows_after_identity_change() {
        let path = temp_sqlite_path("reconcile");
        let store = ApprovalQueueStore::open(path.to_str().expect("path")).expect("open store");
        store
            .sync_order(&sample_order(
                "draft-local",
                Some("client-1"),
                WorkstationOrderStatus::Draft,
            ))
            .expect("persist draft");

        reconcile_runtime_orders(
            &store,
            &[sample_order(
                "remote-123",
                Some("client-1"),
                WorkstationOrderStatus::Open,
            )],
        )
        .expect("replace queue snapshot");

        let loaded = store.load_orders().expect("load queue rows");
        assert!(loaded.is_empty());

        let _ = fs::remove_file(path);
    }

    #[test]
    fn hydrate_runtime_orders_prefers_runtime_state_for_same_identity() {
        let path = temp_sqlite_path("prefer-runtime");
        let store = ApprovalQueueStore::open(path.to_str().expect("path")).expect("open store");
        store
            .sync_order(&sample_order(
                "draft-local",
                Some("client-2"),
                WorkstationOrderStatus::Draft,
            ))
            .expect("persist draft");

        let mut runtime_order = sample_order(
            "remote-123",
            Some("client-2"),
            WorkstationOrderStatus::CancelRequested,
        );
        runtime_order.reason = Some("cancel requested from runtime".to_string());

        let merged = hydrate_runtime_orders(&store, &[runtime_order.clone()])
            .expect("hydrate runtime state");

        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].order_id, "remote-123");
        assert_eq!(
            merged[0].status,
            Some(WorkstationOrderStatus::CancelRequested)
        );
        assert_eq!(
            merged[0].reason.as_deref(),
            Some("cancel requested from runtime")
        );

        let _ = fs::remove_file(path);
    }
}
