# overview

## Overview

**Deposit Destinations** allow you to manage where funds can be deposited into your accounts.

## Crypto destinations

Crypto deposit destinations are cryptocurrency addresses that you can generate via the API. Once created, these addresses can receive cryptocurrency payments on their specified network and will settle in your account balance. **Key Features:**

-   Generate unique crypto addresses for each account
-   Attach metadata to track the purpose or source of deposits

**Supported Networks:** The networks available for deposit destinations depend on your customer type ([Coinbase Business](https://developer.chrome.com/coinbase-business/introduction/welcome) vs [Coinbase Prime](https://developer.chrome.com/prime/introduction/welcome)). See the [API and Network Support](https://developer.chrome.com/api-reference/payment-apis/supported-networks-assets) page for the complete list of networks and assets available for each customer type.

## Examples

**Customer Deposits:** Generate a unique deposit address for each customer to track their deposits separately:

```
{
  "accountId": "account_456",
  "type": "crypto",
  "network": "base",
  "metadata": {
    "customer_id": "123e4567-e89b-12d3-a456-426614174000",
    "reference": "789"
  }
}

```

**Invoice Payments:** Create deposit addresses tied to specific invoices:

```
{
  "accountId": "account_456",
  "type": "crypto",
  "network": "ethereum",
  "metadata": {
    "invoice_id": "12345",
    "order_id": "67890"
  }
}

```

You can attach metadata to any crypto deposit destination you create to track the purpose or source of deposits. This metadata helps you identify and reconcile incoming payments in your system.

**Example:**

```
{
  "depositDestinationId": "depositDestination_123",
  "accountId": "account_456",
  "type": "crypto",
  "address": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
  "network": "base",
  "metadata": {
    "customer_id": "123e4567-e89b-12d3-a456-426614174000",
    "order_id": "12345",
    "invoice_number": "98765"
  }
}

```

## Filtering and listing

Use the list endpoint to retrieve all deposit destinations. You can filter by **Account ID** to see deposit destinations for a specific account. **Example:**

```
GET /v2/deposit-destinations?accountId=account_123

```

## Unsupported assets

## What to read next

-   [Create a Deposit Destination](#) - Generate a new crypto deposit address
-   [List Deposit Destinations](#) - View all your deposit destinations