# best practices

## Overview

Follow these best practices to build secure, production-ready applications with the SQL API.

## Prioritize indexed fields for filtering

To maximize query speed, always filter your data using [indexed fields](https://developer.chrome.com/data/sql-api/schema). Filtering on non-indexed fields forces full table scans, which are resource-intensive. **Key indexed fields for filtering** (use these in your WHERE clauses):

-   `event_signature` (use this instead of `event_name`)
-   `address`
-   `block_timestamp`

```
-- Good: Uses indexed fields for fast lookup
SELECT address, block_timestamp, transaction_hash
FROM events
WHERE block_timestamp='2025-10-04T23:04:09.000Z'
AND event_signature = 'Transfer(address,address,uint256)'
AND address = '0x456...def';

```

## Select only necessary fields

Avoid using `SELECT *`. Selecting unnecessary fields consumes more RAM and accesses more disk space, which increases the likelihood of your query reaching resource limits.

```
-- Good: Only selects the necessary fields
SELECT transaction_hash, log_index
FROM base.events
WHERE address = '0x456...def';

```

## Leverage caching with `maxAgeMs`

Developers should take advantage of SQL API’s [cache controls](https://developer.chrome.com/api-reference/v2/rest-api/sql-api/run-sql-query#body-cache) when executing high frequency queries. Ensure you’re using the maximum `maxAgeMs` that your queries can reasonably tolerate. This allows the API to return a cached response, which limits direct database calls.

```
curl --request POST \
  --url https://api.cdp.coinbase.com/platform/v2/data/query/run \
  --header 'Authorization: Bearer <token>' \
  --header 'Content-Type: application/json' \
  --data '{
  "sql": "SELECT block_number, transaction_hash FROM base.events WHERE address = '0x456...def'",
  "cache": {
    "maxAgeMs": 5000
  }
}'

```

## Separate dev and prod project IDs

Always use a different project ID for your development (dev) and production (prod) integrations. Clear separation ensures that testing and debugging activities do not impact the performance or rate limits of your live production environment.