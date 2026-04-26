# conventions

## Authentication

**Header:** `Authorization: Bearer {token}` **Token:** JWT signed using your CDP API Key Secret **Usage:** Most API endpoints (accounts, transfers, payment methods) **Docs:** See [Authentication Guide](https://developer.chrome.com/get-started/authentication/overview)

## Error Responses

All errors return a consistent structure:

```
{
  "errorType": "invalid_request",
  "errorMessage": "Invalid request.",
  "correlationId": "41deb8d59a9dc9a7-IAD",
  "errorLink": "https://docs.cdp.coinbase.com/api-reference/v2/errors#invalid-request"
}

```

**Fields:**

-   `errorType` - Machine-readable error code
-   `errorMessage` - Human-readable description
-   `correlationId` - Unique request identifier for debugging
-   `errorLink` - Documentation link for the error

**Common Error Types:**

-   `invalid_request` - Malformed request
-   `not_found` - Resource doesn’t exist
-   `forbidden` - Permission denied
-   `idempotency_error` - Idempotency key conflict
-   `internal_server_error` - Server error

## HTTP Status Codes

Code

Meaning

When Used

200

Success

GET, PUT requests succeeded

201

Created

POST request created resource

400

Bad Request

Invalid parameters or malformed request

401

Unauthorized

Missing or invalid authentication

403

Forbidden

Authenticated but lacks permission

404

Not Found

Resource doesn’t exist

409

Conflict

Idempotency key conflict

500

Internal Server Error

Server-side error occurred

## Resource IDs

All resources use prefixed identifier patterns:

Resource

Prefix

Example

Account

`account_`

`account_af2937b0-9846-4fe7-bfe9-ccc22d935114`

Transfer

`transfer_`

`transfer_af2937b0-9846-4fe7-bfe9-ccc22d935114`

Payment Method

`pm_`

`pm_af2937b0-9846-4fe7-bfe9-ccc22d935114`

Customer

`customer_`

`customer_af2937b0-9846-4fe7-bfe9-ccc22d935114`

IDs follow the pattern: `{prefix}_{uuid}`

## Amounts

All monetary amounts are represented as strings to preserve precision. **Format:** String representation of decimal number **Examples:**

-   `"100.00"` - 100 USD
-   `"2.5"` - 2.5 BTC
-   `"103.50"` - 103.50 USD

Amounts are specified in atomic units of the asset. Always use strings, never floats, to avoid precision loss.

## Assets

Assets are identified by lowercase symbols. **Fiat:** `usd` **Stablecoins:** `usdc`, `usdt`, `eurc`, `pyusd`

## Timestamps

All datetime fields are returned in UTC using ISO 8601 format with `Z` suffix. **Format:** `YYYY-MM-DDTHH:MM:SSZ` **Example:** `2023-10-08T14:30:00Z`

List endpoints support cursor-based pagination using tokens. **Query Parameters:**

-   `pageSize` - Number of resources per page (default: 20)
-   `pageToken` - Token for next page from previous response

**Response Field:**

-   `nextPageToken` - Token to fetch next page (absent on last page)

**Example:**

```
GET /v2/accounts?pageSize=10&pageToken=eyJsYXN0X2lkIjogImFiYzEyMyJ9

```

## Idempotency

Use idempotency keys to safely retry requests without duplicate operations. **Header:** `X-Idempotency-Key` **Format:** UUID v4 (36 characters) **Pattern:** `^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$` **Example:** `8e03978e-40d5-43e8-bc93-6894a57f9324` **Behavior:**

-   Optional header for POST requests
-   Duplicate requests with same key return identical responses and won’t repeat the same operation
-   Prevents accidental duplicate operations (e.g., creating duplicate transfers)

**Error:** Returns HTTP 422 with `idempotency_error` if another request with the same key is currently processing