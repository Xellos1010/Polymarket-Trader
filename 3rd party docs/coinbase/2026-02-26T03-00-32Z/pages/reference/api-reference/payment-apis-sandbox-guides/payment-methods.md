# payment methods

## Overview

Payment methods represent external financial instruments (like bank accounts via Fedwire and SWIFT) that you can use to move money. They are entity-level and you can use them with any of your accounts. **Key differences:**

-   **Accounts** hold assets and balances (e.g., your USD account)
-   **Payment methods** are external destinations (e.g., your JPMorgan Chase bank account)

**Example:** You have $1000 in your Coinbase USD account. You can withdraw $100 to your JPMorgan Chase bank, then later withdraw $200 to your Bank of America account. Each external bank is a separate payment method. In Sandbox, three test payment methods are automatically created at the entity level and can be used with all your accounts.

## Pre-configured payment methods

Three test payment methods are automatically created in your Sandbox environment:

Payment Rail

Bank

Status

Behavior

Fedwire

JPMorgan Chase Bank NA

`active`

Transfers succeed

Fedwire

Bank of America NA

`inactive`

Transfers fail

SWIFT

Deutsche Bank

`active`

Transfers succeed

## Prerequisites

Complete the [Quickstart](https://developer.chrome.com/api-reference/payment-apis/sandbox/quickstart) before proceeding if you do not yet have Sandbox API keys or a funded account. Ensure you have set your API key as an environment variable:

```
export CDP_API_KEY=~/Downloads/cdp_api_key.json

```

## 1\. List payment methods

See your available test methods, their `paymentMethodId` values, and other details:

```
cdpcurl -k $CDP_API_KEY \
  'https://sandbox.cdp.coinbase.com/platform/v2/payment-methods'

```

Example response

```
{
  "paymentMethods": [
    {
      "paymentMethodId": "paymentMethod_398435cb-03bd-5568-b8d5-44accd7ce305",
      "active": true,
      "paymentRail": "fedwire",
      "fedwire": {
        "bankName": "JPMorgan Chase Bank NA",
        "accountLast4": "9012",
        "routingNumber": "021000021",
        "asset": "usd"
      }
    },
    {
      "paymentMethodId": "paymentMethod_82933da3-bd1e-5c8d-a05f-9ef912e5bce9",
      "active": false,
      "paymentRail": "fedwire",
      "fedwire": {
        "bankName": "Bank of America NA",
        "accountLast4": "1098",
        "routingNumber": "026009593",
        "asset": "usd"
      }
    },
    {
      "paymentMethodId": "paymentMethod_d984c884-7fef-51e8-98a2-742ba6e32515",
      "active": true,
      "paymentRail": "swift",
      "swift": {
        "bankName": "Deutsche Bank",
        "bic": "DEUTDEFF",
        "ibanLast4": "3000",
        "asset": "usd"
      }
    }
  ]
}

```

## 2\. Test transfer flows

Use payment methods as targets to test transfer flows.

Set your IDs as environment variables:

```
export ACCOUNT_ID="account_abc123..."           # Your account ID
export ACTIVE_PM_ID="paymentMethod_xyz789..."   # Active payment method ID
export INACTIVE_PM_ID="paymentMethod_def456..." # Inactive payment method ID

```

### Successful transfer

This simulates transferring from your Sandbox account to an **active payment method**:

```
cdpcurl -k $CDP_API_KEY \
  -X POST \
  -d "{
    \"source\": {
      \"accountId\": \"$ACCOUNT_ID\",
      \"asset\": \"usd\"
    },
    \"target\": {
      \"paymentMethodId\": \"$ACTIVE_PM_ID\",
      \"asset\": \"usd\"
    },
    \"amount\": \"5.00\",
    \"asset\": \"usd\",
    \"execute\": true
  }" \
  'https://sandbox.cdp.coinbase.com/platform/v2/transfers'

```

The transfer completes successfully, your account balance decreases, and you receive webhook events (`payment.transfer.processing` → `payment.transfer.completed`).

Example response

```
{
  "transferId": "transfer_8b707d29-4690-4948-b645-de1cd1f5fd05",
  "status": "completed",
  "source": {
    "accountId": "account_db458f63-418a-4a91-a045-fab93ac35c3f",
    "asset": "usd"
  },
  "target": {
    "paymentMethodId": "paymentMethod_398435cb-03bd-5568-b8d5-44accd7ce305",
    "asset": "usd"
  },
  "sourceAmount": "5",
  "sourceAsset": "usd",
  "targetAmount": "5",
  "targetAsset": "usd",
  "createdAt": "2026-02-11T23:19:24.086Z",
  "updatedAt": "2026-02-11T23:19:24.183Z"
}

```

### Failed transfer

This simulates transferring to an **inactive payment method** to test error handling:

```
cdpcurl -k $CDP_API_KEY \
  -X POST \
  -d "{
    \"source\": {
      \"accountId\": \"$ACCOUNT_ID\",
      \"asset\": \"usd\"
    },
    \"target\": {
      \"paymentMethodId\": \"$INACTIVE_PM_ID\",
      \"asset\": \"usd\"
    },
    \"amount\": \"5.00\",
    \"asset\": \"usd\",
    \"execute\": true
  }" \
  'https://sandbox.cdp.coinbase.com/platform/v2/transfers'

```

The transfer fails with an error response. Use this to test how your application handles failed transfers.

Example error response

```
{
  "errorType": "invalid_request",
  "errorMessage": "Target payment method payment method id is invalid.",
  "errorLink": "https://docs.cdp.coinbase.com/api-reference/v2/errors#invalid-request",
  "correlationId": "9cc7d1cd1cae8b7e-IAD"
}

```

For more transfer examples, see the [Transfers guide](https://developer.chrome.com/api-reference/payment-apis/sandbox/guides/transfers).

## API reference

-   [List Payment Methods](https://developer.chrome.com/api-reference/payment-apis/rest-api/payment-methods-under-development/list-payment-methods)
-   [Create a Transfer](https://developer.chrome.com/api-reference/payment-apis/rest-api/transfers-under-development/create-a-transfer)