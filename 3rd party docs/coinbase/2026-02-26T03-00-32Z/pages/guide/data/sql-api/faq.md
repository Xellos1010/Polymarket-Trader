# faq

### What SQL features are supported?

CoinbaSeQL supports all standard SQL query features including SELECT statements, WHERE filtering, JOINs, aggregations (COUNT, SUM, AVG, MIN, MAX), subqueries, Common Table Expressions (CTEs), UNION operations, and CASE statements. See the [CoinbaSeQL reference](https://developer.chrome.com/data/sql-api/sql) for details.

### What’s the difference between SQL API and Wallet History API?

-   **SQL API**: Write custom SQL queries against any blockchain data (events, transactions, blocks, transfers). Flexible and powerful.
-   **Wallet History API**: Pre-built endpoints for wallet-specific data. Simple and fast for common wallet operations.

Use SQL API when you need custom queries or data beyond wallet history. Use Wallet History API for simple wallet transaction and balance lookups.

### Do I need API keys?

-   **SQL Playground** (browser): No API keys needed—just sign in to [CDP Portal](https://portal.cdp.coinbase.com/)
-   **REST API** (programmatic): Yes, create free [Client API keys](https://portal.cdp.coinbase.com/projects/api-keys/client-key)

### What are the query limits?

-   **Maximum result set**: 10,000 rows
-   **Query timeout**: 30 seconds
-   **Maximum JOINs**: 5 per query
-   **Query length**: 50,000 characters maximum
-   **Rate limit**: 100 requests per second

### How do I optimize slow queries?

1.  **Use indexed columns in WHERE clauses**: For example, when querying `base.events`, query by `event_signature` and `address`. Check the schema of each table
2.  **Use specific block ranges**: Query smaller block ranges by `block_timestamp` rather than the entire blockchain’s history from genesis
3.  **Filter early**: Put selective filters in WHERE clauses
4.  **Avoid SELECT \***: Select only the columns you need

### What happens if my query times out?

If your query exceeds the 30-second timeout, you’ll receive a `timed_out` error. To fix:

-   Filter by `event_signature` to remove any irrelevant event logs
-   Reduce the block range in your WHERE clause via the `block_timestamp` field
-   Simplify complex JOINs (avoid `OR` in JOINs)

### What networks are supported?

SQL API supports **Base Mainnet** and **Base Sepolia**. Each table is prefixed with the network (e.g., `base.events`, `base.transactions` for mainnet, `base_sepolia.events` for testnet).

### How fresh is the data?

SQL API data is **< 250ms from chain tip** with query response latency **< 500ms**. This means you get near real-time blockchain data.

### What data types does SQL API support?

SQL API uses ClickHouse data types including:

-   **Numeric**: UInt8, UInt16, UInt32, UInt64, UInt128, UInt256, Int8, Int16, Int32, Int64, Int128, Int256
-   **String**: String
-   **Boolean**: Bool
-   **Temporal**: Date, DateTime, DateTime64
-   **Complex**: Array, Map, Tuple

See the [schema reference](https://developer.chrome.com/data/sql-api/schema) for field-level type information.

### How do I handle re-orgs?

Each table includes an `action` field:

-   `1` or `'added'`: Data was added to the chain
-   `-1` or `'removed'`: Data was removed due to reorganization

To query only active (non-reorged) data, filter where the sum of actions is greater than 0, or simply filter for `action = 1` or `action = 'added'`. The `log_id` on each row can be used to identify duplicates.

### Where can I get help?

Join **#onchain-data** in the [CDP Discord](https://discord.com/invite/cdp) to connect with our team and other developers.