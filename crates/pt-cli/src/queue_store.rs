use chrono::Utc;
use pt_core::{WorkstationOrder, WorkstationOrderStatus};
use rusqlite::{params, Connection};
use std::{collections::HashSet, fs, path::Path};

pub struct ApprovalQueueStore {
    conn: Connection,
}

impl ApprovalQueueStore {
    pub fn open(sqlite_path: &str) -> Result<Self, String> {
        ensure_parent_dir(sqlite_path)?;
        let conn = Connection::open(sqlite_path).map_err(|e| e.to_string())?;
        conn.pragma_update(None, "journal_mode", "WAL")
            .map_err(|e| e.to_string())?;
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS approval_queue_orders (
                order_id TEXT PRIMARY KEY,
                status TEXT NOT NULL,
                payload TEXT NOT NULL,
                created_at_ms INTEGER NOT NULL,
                updated_at_ms INTEGER NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_approval_queue_orders_status_updated
                ON approval_queue_orders (status, updated_at_ms DESC);
            ",
        )
        .map_err(|e| e.to_string())?;
        Ok(Self { conn })
    }

    pub fn sync_order(&self, order: &WorkstationOrder) -> Result<(), String> {
        match queue_status_label(order.status.as_ref()) {
            Some(status) => {
                let payload = serde_json::to_string(order).map_err(|e| e.to_string())?;
                let created_at_ms = order
                    .created_at
                    .unwrap_or_else(Utc::now)
                    .timestamp_millis();
                let updated_at_ms = order
                    .updated_at
                    .or(order.created_at)
                    .unwrap_or_else(Utc::now)
                    .timestamp_millis();
                self.conn
                    .execute(
                        "
                        INSERT INTO approval_queue_orders (
                            order_id,
                            status,
                            payload,
                            created_at_ms,
                            updated_at_ms
                        )
                        VALUES (?1, ?2, ?3, ?4, ?5)
                        ON CONFLICT(order_id) DO UPDATE SET
                            status = excluded.status,
                            payload = excluded.payload,
                            updated_at_ms = excluded.updated_at_ms
                        ",
                        params![order.order_id, status, payload, created_at_ms, updated_at_ms],
                    )
                    .map_err(|e| e.to_string())?;
            }
            None => {
                self.conn
                    .execute(
                        "DELETE FROM approval_queue_orders WHERE order_id = ?1",
                        params![order.order_id],
                    )
                    .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub fn sync_orders(&self, orders: &[WorkstationOrder]) -> Result<(), String> {
        for order in orders {
            self.sync_order(order)?;
        }
        Ok(())
    }

    pub fn replace_orders(&self, orders: &[WorkstationOrder]) -> Result<(), String> {
        let queue_orders = orders
            .iter()
            .filter(|order| queue_status_label(order.status.as_ref()).is_some())
            .collect::<Vec<_>>();
        let active_ids = queue_orders
            .iter()
            .map(|order| order.order_id.clone())
            .collect::<HashSet<_>>();

        for order in queue_orders {
            self.sync_order(order)?;
        }

        let mut stmt = self
            .conn
            .prepare("SELECT order_id FROM approval_queue_orders")
            .map_err(|e| e.to_string())?;
        let existing_ids = stmt
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| e.to_string())?;

        for row in existing_ids {
            let order_id = row.map_err(|e| e.to_string())?;
            if !active_ids.contains(&order_id) {
                self.conn
                    .execute(
                        "DELETE FROM approval_queue_orders WHERE order_id = ?1",
                        params![order_id],
                    )
                    .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub fn load_orders(&self) -> Result<Vec<WorkstationOrder>, String> {
        let mut stmt = self
            .conn
            .prepare(
                "
                SELECT payload
                FROM approval_queue_orders
                ORDER BY updated_at_ms DESC, order_id ASC
                ",
            )
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| e.to_string())?;

        let mut orders = Vec::new();
        for row in rows {
            let payload = row.map_err(|e| e.to_string())?;
            let order: WorkstationOrder =
                serde_json::from_str(&payload).map_err(|e| e.to_string())?;
            if queue_status_label(order.status.as_ref()).is_some() {
                orders.push(order);
            }
        }
        Ok(orders)
    }
}

fn queue_status_label(status: Option<&WorkstationOrderStatus>) -> Option<&'static str> {
    match status {
        Some(WorkstationOrderStatus::Draft) => Some("draft"),
        Some(WorkstationOrderStatus::CancelRequested) => Some("cancel_requested"),
        _ => None,
    }
}

fn ensure_parent_dir(path: &str) -> Result<(), String> {
    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::ApprovalQueueStore;
    use chrono::Utc;
    use pt_core::{OrderRoute, ProductId, Side, WorkstationOrder, WorkstationOrderStatus};
    use std::{env, fs, path::PathBuf};

    fn temp_sqlite_path(name: &str) -> PathBuf {
        env::temp_dir().join(format!(
            "pt-approval-queue-{name}-{}-{}.sqlite",
            std::process::id(),
            Utc::now().timestamp_nanos_opt().unwrap_or_default()
        ))
    }

    fn sample_order(order_id: &str, status: WorkstationOrderStatus) -> WorkstationOrder {
        let now = Utc::now();
        WorkstationOrder {
            order_id: order_id.to_string(),
            client_order_id: Some(format!("client-{order_id}")),
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
    fn load_orders_round_trips_queue_relevant_statuses() {
        let path = temp_sqlite_path("roundtrip");
        let store = ApprovalQueueStore::open(path.to_str().expect("path")).expect("open store");

        store
            .sync_orders(&[
                sample_order("draft-1", WorkstationOrderStatus::Draft),
                sample_order("cancel-1", WorkstationOrderStatus::CancelRequested),
                sample_order("open-1", WorkstationOrderStatus::Open),
            ])
            .expect("sync orders");

        let loaded = store.load_orders().expect("load orders");
        assert_eq!(loaded.len(), 2);
        assert!(loaded.iter().any(|order| order.order_id == "draft-1"));
        assert!(loaded.iter().any(|order| order.order_id == "cancel-1"));
        assert!(!loaded.iter().any(|order| order.order_id == "open-1"));

        let _ = fs::remove_file(path);
    }

    #[test]
    fn sync_order_prunes_rows_when_status_leaves_queue() {
        let path = temp_sqlite_path("prune");
        let store = ApprovalQueueStore::open(path.to_str().expect("path")).expect("open store");

        let mut order = sample_order("draft-2", WorkstationOrderStatus::Draft);
        store.sync_order(&order).expect("persist draft order");
        assert_eq!(store.load_orders().expect("load orders").len(), 1);

        order.status = Some(WorkstationOrderStatus::Open);
        order.updated_at = Some(Utc::now());
        store.sync_order(&order).expect("prune promoted order");
        assert!(store.load_orders().expect("load orders").is_empty());

        let _ = fs::remove_file(path);
    }

    #[test]
    fn sync_order_updates_existing_payload_for_restart_recovery() {
        let path = temp_sqlite_path("update");
        let store = ApprovalQueueStore::open(path.to_str().expect("path")).expect("open store");

        let mut order = sample_order("draft-3", WorkstationOrderStatus::Draft);
        store.sync_order(&order).expect("persist initial order");

        order.status = Some(WorkstationOrderStatus::CancelRequested);
        order.reason = Some("cancel requested from dashboard".to_string());
        order.updated_at = Some(Utc::now());
        store.sync_order(&order).expect("update queue order");

        let loaded = store.load_orders().expect("load orders");
        assert_eq!(loaded.len(), 1);
        assert_eq!(
            loaded[0].status,
            Some(WorkstationOrderStatus::CancelRequested)
        );
        assert_eq!(
            loaded[0].reason.as_deref(),
            Some("cancel requested from dashboard")
        );

        let _ = fs::remove_file(path);
    }

    #[test]
    fn replace_orders_prunes_stale_rows_when_snapshot_changes_identity() {
        let path = temp_sqlite_path("replace");
        let store = ApprovalQueueStore::open(path.to_str().expect("path")).expect("open store");

        let initial = sample_order("draft-local", WorkstationOrderStatus::Draft);
        store.sync_order(&initial).expect("persist initial draft order");
        assert_eq!(store.load_orders().expect("load orders").len(), 1);

        let mut submitted = initial.clone();
        submitted.order_id = "remote-123".to_string();
        submitted.status = Some(WorkstationOrderStatus::Open);
        submitted.updated_at = Some(Utc::now());

        store
            .replace_orders(&[submitted])
            .expect("replace queue snapshot after submit");
        assert!(store.load_orders().expect("load orders").is_empty());

        let _ = fs::remove_file(path);
    }
}
