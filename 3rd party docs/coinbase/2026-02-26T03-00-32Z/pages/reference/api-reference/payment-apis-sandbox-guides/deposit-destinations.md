# deposit destinations

## Overview

Deposit destinations are addresses where you can receive crypto payments. In Sandbox, these are **placeholder addresses** for API testing. They are not real blockchain addresses, and the Sandbox does not connect to any blockchain network.

## Prerequisites

Before you begin, you need `cdpcurl` and Sandbox API key. See the [Quickstart](https://developer.chrome.com/api-reference/payment-apis/sandbox/quickstart) for instructions.

## Programmatically

### 1\. Create a deposit destination

Create a [deposit destination](https://developer.chrome.com/api-reference/payment-apis/rest-api/deposit-destinations-under-development/create-crypto-deposit-destination):

```
cdpcurl -k $CDP_API_KEY \
  -X POST \
  -d '{
    "accountId": "YOUR_ACCOUNT_ID",
    "network": "base",
    "asset": "usdc"
  }' \
  'https://sandbox.cdp.coinbase.com/platform/v2/deposit-destinations'

```

Example response

```
{
  "depositDestinationId": "dd_abc123...",
  "address": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
  "network": "base",
  "asset": "usdc",
  "accountId": "account_db458f63-418a-4a91-a045-fab93ac35c3f",
}

```

### 2\. List your deposit destinations

See all deposit destinations for an account:

```
cdpcurl -k $CDP_API_KEY \
  'https://sandbox.cdp.coinbase.com/platform/v2/deposit-destinations?accountId=YOUR_ACCOUNT_ID'

```

Example response

```
{
  "depositDestinations": [
    {
      "depositDestinationId": "dd_abc123...",
      "address": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
      "network": "base",
      "asset": "usdc",
      "accountId": "account_db458f63-418a-4a91-a045-fab93ac35c3f",
    }
  ]
}

```

### 3\. Simulate a deposit

Simulate an incoming deposit to test webhook integration and balance updates. The following example simulates an external sender depositing funds to a created deposit destination:

```
cdpcurl -k $CDP_API_KEY \
  -X POST \
  -d '{
    "deposit_address": "<DEPOSIT_DESTINATION_ADDRESS>",
    "amount": "100.00",
    "asset_symbol": "usdc",
    "network": "base"
  }' \
  'https://sandbox.cdp.coinbase.com/fake/deposit/crypto'

```

Example response

```
{
  "transfer_id": "transfer_b340437d-4705-446f-8852-2345c83ace60",
  "created_at": "2026-02-24T03:12:10.077Z"
}

```

**What happens when you simulate a deposit:**

1.  **Webhook events fire:**
    -   `payment.transfer.processing`
    -   `payment.transfer.completed`
2.  **Transfer records are created:**
    -   Appear in the [List Transfers](https://developer.chrome.com/api-reference/payment-apis/rest-api/transfers-under-development/list-transfers) API
3.  **Balance is credited:**
    -   The account balance updates immediately

## Using Portal UI

You can create deposit destinations and simulate deposits through the Portal UI.

### 1\. Create a deposit address

### 2\. Simulate a deposit

Simulate incoming deposits to test your webhook integration and balance updates. This simulates an external sender depositing funds **to your deposit address**, which then automatically credits your account balance.

**What happens when you simulate a deposit:**

1.  **Webhook events fire:**
    -   `payment.transfer.processing`
    -   `payment.transfer.completed`
2.  **Transfer records are created:**
    -   Appear in the [List Transfers](https://developer.chrome.com/api-reference/payment-apis/rest-api/transfers-under-development/list-transfers) API
3.  **Balance is credited:**
    -   The account balance updates immediately

## What to read next