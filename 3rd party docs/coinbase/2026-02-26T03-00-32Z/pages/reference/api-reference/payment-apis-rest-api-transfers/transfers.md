# transfers

**Transfers** represent both the request and execution of fund transfers from a source to a target. They provide upfront fee quotes and track the complete lifecycle from initiation through completion or failure.

## Transfer lifecycle

When you create a transfer, it will be in one of these statuses:

-   **`quoted`** - Transfer was created with `execute: false` and is waiting for manual execution, or was created with `execute: true` and is momentarily being quoted before executing. Execute by calling `/v2/transfers/{transferId}/execute`.
-   **`processing`** - Transfer is executing. No action needed - monitor progress via the transfers webhook.
-   **`completed`** - Transfer completed successfully.
-   **`failed`** - Transfer failed. This can happen due to an execution error or because the fee quote expired before the transfer was executed. See `failureReason` for details.

## Execution control

-   **`execute: true`**: Transfer will automatically execute after being quoted
-   **`execute: false`**: Transfer will be created in `quoted` status and you must call the `/execute` endpoint. Use this to obtain a fee quote or validate a transfer destination before deciding whether to execute the transfer.

## Fee quotes

Every transfer provides a comprehensive fee quote in the `fees` array. This allows you to show users exactly what they’ll pay before any money moves. To review fees before execution:

1.  Create a transfer with `execute: false`
2.  Review the `fees` array in the response
3.  Call `POST /v2/transfers/{transferId}/execute` when ready to proceed

For automatic execution without fee review, create a transfer with `execute: true`.

## Transfer validation

Use `validateOnly: true` to validate a transfer without initiating or persisting it. This is useful for verifying that a target can receive funds before committing to any costly operations.

### Use cases

Transfer validation is particularly useful when you need to:

-   **Verify recipient addresses** before performing foreign exchange (FX) conversions
-   **Pre-validate email recipients** to ensure users exist before showing transfer confirmation
-   **Check onchain addresses** are valid for the specified network before committing funds

### How it works

When you set `validateOnly: true`:

-   The transfer is **validated but not persisted** to the database
-   The transfer will **not appear in the portal** or in list transfer responses
-   A `2xx` response indicates the transfer parameters are valid
-   A `4xx` error indicates a validation failure with an `errorType` describing the issue

### Example request

```
{
  "source": {
    "accountId": "account_af2937b0-9846-4fe7-bfe9-ccc22d935114",
    "asset": "usd"
  },
  "target": {
    "email": "recipient@example.com",
    "asset": "usd"
  },
  "amount": "100.00",
  "asset": "usd",
  "validateOnly": true
}

```

### Validation response

When validation succeeds, the response includes the validated transfer details without persistence-related fields:

```
{
  "source": {
    "accountId": "account_af2937b0-9846-4fe7-bfe9-ccc22d935114",
    "asset": "usd"
  },
  "target": {
    "email": "recipient@example.com",
    "asset": "usd"
  },
  "sourceAmount": "100.00",
  "sourceAsset": "usd",
  "targetAmount": "100.00",
  "targetAsset": "usd"
}

```

### Validation errors

When validation fails, the response includes an `errorType` indicating the issue:

Error Type

Description

`invalid_request`

The request format is invalid or missing required fields, including invalid recipient addresses or unsupported networks

`not_found`

The specified target user or account was not found

See [Errors](https://developer.chrome.com/api-reference/payment-apis/errors) for the complete list of error types.

### Sandbox testing

## Sources and targets

-   A **source** must be an Account.
-   A **target** can be an Account, Payment Method, Onchain Address, or Email Address

## Amount type

The `amountType` field specifies whether the given amount is received by the target or taken from the source:

-   **`source`** (default): The target receives the amount minus any fees
-   **`target`**: The target receives the exact amount specified; fees are added to the amount taken from the source

**Example**: To send exactly $100 to the recipient (with fees paid separately):

```
{
  "amount": "100.00",
  "amountType": "target"
}

```

## Fees

Each fee in the `fees` array has a `type` indicating its purpose:

Fee Type

Description

`bank`

Traditional banking fees (e.g., wire transfer fees)

`conversion`

Asset conversion/exchange fees

`network`

Blockchain network fees (gas costs)

`other`

Other processing fees

**Example fees array:**

```
{
  "fees": [
    { "type": "bank", "amount": "15.00", "asset": "usd" },
    { "type": "conversion", "amount": "1.00", "asset": "usd" },
    { "type": "network", "amount": "0.001", "asset": "eth" }
  ]
}

```

**Fee expiration**: Fee quotes are valid for a limited time. The `expiresAt` field shows exactly when the fee quote will expire. If you don’t execute before this time, you’ll need to create a new transfer to get updated fees.

## Exchange rate

For transfers involving currency conversion, the `exchangeRate` object provides rate information:

```
{
  "exchangeRate": {
    "sourceAsset": "usd",
    "targetAsset": "btc",
    "rate": "0.00001"
  }
}

```

The `rate` indicates how many units of the target asset equal one unit of the source asset.

## Transfer completion

When a transfer reaches `completed` status, it contains the final execution details:

-   `completedAt` - When the transfer finished
-   `executedAt` - When the transfer moved from `quoted` to `processing`
-   `targetAmount` - The actual amount delivered to the target
-   `details` - Additional information (e.g., deposit destination reference)

## Failure details

When a transfer fails, the `failureReason` field contains a human-readable description of what went wrong. A transfer can reach `failed` status in two ways:

-   **Execution error** — the transfer was executing and encountered an error (e.g., insufficient balance, network failure).
-   **Quote expiration** — the transfer was in `quoted` status and the fee quote expired before `/execute` was called. Create a new transfer to get a fresh quote.

**Example:**

```
{
  "status": "failed",
  "failureReason": "Insufficient balance to complete this transfer."
}

```

## Travel rule

For transfers that require travel rule compliance, use the `travelRule` object:

Field

Description

`isSelf`

Whether the receiving wallet belongs to the sender

`isIntermediary`

Whether Coinbase is acting as an intermediary VASP

`originator`

Information about the sender (name, address, VASP details)

`beneficiary`

Information about the receiver (name, address, wallet type)

**When to set `isIntermediary: true`:** Set this when your organization is a VASP using Coinbase to send crypto on behalf of your end customer. In this scenario, you must provide the `originator` object with:

-   Originator name and address
-   Your VASP information (`virtualAssetServiceProvider` with `name`, `address`, `identifier`)

**Example:**

```
{
  "travelRule": {
    "isSelf": false,
    "isIntermediary": true,
    "originator": {
      "name": "John Doe",
      "address": {
        "line1": "123 Main St",
        "city": "San Francisco",
        "postCode": "94105",
        "countryCode": "US"
      },
      "virtualAssetServiceProvider": {
        "name": "Your VASP Name",
        "identifier": "5493001KJTIIGC8Y1R17"
      }
    },
    "beneficiary": {
      "name": "Jane Smith",
      "walletType": "custodial"
    }
  }
}

```