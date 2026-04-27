# create payment link

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Headers

An optional [UUID v4](https://www.uuidgenerator.net/version4) request header for making requests safely retryable.

Required string length: `36`

#### Body

The payment amount as a string for precise decimal representation. Must be between 0.01 and 100000000 (100 million). Must have at most 2 decimal places.

The currency code for the payment. Only USDC is currently supported.

Required string length: `1 - 10`

The blockchain network for the payment. Defaults to base if not specified. More networks will be added in the future.

Human-readable description of the payment.

Maximum string length: `500`

Example:

`"Payment for order #12345"`

Optional expiration timestamp for the payment link in RFC 3339 format. Must be a future timestamp. After this time, the link will be automatically expired and cannot accept new payments. If not provided, defaults to 1 year from creation time.

Example:

`"2026-03-20T10:30:00Z"`

Optional metadata as key-value pairs to be passed through the payment flow.

Example:

```
{  
  "invoiceId": "12345",  
  "reference": "Payment for invoice #12345",  
  "customerId": "cust_abc123"  
}
```

URL to redirect to on successful payment. Must use HTTPS protocol.

Maximum string length: `2048`

Example:

`"https://example.com/success"`

URL to redirect to on failed payment. Must use HTTPS protocol.

Maximum string length: `2048`

Example:

`"https://example.com/failed"`

#### Response

Payment link created successfully.

Unique payment link identifier.

Example:

`"68f7a946db0529ea9b6d3a12"`

The generated payment link URL.

Example:

`"https://pay.coinbase.com/pl_01h8441j23abcd1234567890ef"`

The status of the payment link.

-   `ACTIVE` The payment link is active and can accept payments.
-   `DEACTIVATED` The payment link has been manually deactivated.
-   `EXPIRED` The payment link has expired based on the expiresAt timestamp.
-   `COMPLETED` The payment link has been successfully paid.
-   `FAILED` The payment link has failed due to a payment error.

Available options

:

`ACTIVE`,

`PROCESSING`,

`DEACTIVATED`,

`EXPIRED`,

`COMPLETED`,

`FAILED`

Numeric value representing the amount (maximum 2 decimal places).

The currency code for the amount.

network

enum<string>

default:base

required

The blockchain network for the payment. Defaults to base if not specified. More networks will be added in the future.

The blockchain address where funds should be sent.

Example:

`"0x742d35Cc6634C0532925a3b844Bc454e4438f44e"`

createdAt

string<date-time>

required

Timestamp in RFC 3339 format.

Example:

`"2024-03-20T10:30:00Z"`

updatedAt

string<date-time>

required

Timestamp in RFC 3339 format.

Example:

`"2024-03-20T10:30:00Z"`

The token contract address (for ERC-20 tokens).

Example:

`"0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913"`

Human-readable description of the payment.

Example:

`"Payment for order #12345"`

Timestamp in RFC 3339 format.

Example:

`"2024-03-20T10:30:00Z"`

Optional metadata as key-value pairs to be passed through the payment flow.

Example:

```
{  
  "invoiceId": "12345",  
  "reference": "Payment for invoice #12345",  
  "customerId": "cust_abc123"  
}
```

Optional URL to redirect the user to after successful payment authorization. This indicates the user has successfully authorized the payment, not that the payment has been completed.

Example:

`"https://example.com/success"`

Optional URL to redirect the user to after failed payment authorization.

Example:

`"https://example.com/failed"`

Financial breakdown of the payment link transaction showing the total amount charged, fees deducted, and net amount received.

The blockchain transaction hash for completed payments. Only populated when status is COMPLETED.

Example:

`"0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"`