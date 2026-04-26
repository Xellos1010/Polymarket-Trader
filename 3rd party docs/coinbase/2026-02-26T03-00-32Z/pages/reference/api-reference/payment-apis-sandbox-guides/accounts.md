# accounts

## Overview

Accounts are containers that hold assets and can be used for transacting. In Sandbox, create accounts through the Portal UI.

## Prerequisites

Before you begin, you need `cdpcurl` and a Sandbox API key. See the [Quickstart](https://developer.chrome.com/api-reference/payment-apis/sandbox/quickstart) for instructions.

## 1\. List accounts

See all your Sandbox accounts:

```
cdpcurl -k $CDP_API_KEY \
  'https://sandbox.cdp.coinbase.com/platform/v2/accounts'

```

Example response

```
{
  "accounts": [
    {
      "accountId": "account_db458f63-418a-4a91-a045-fab93ac35c3f",
      "name": "My Test Account",
      "createdAt": "2026-02-11T20:00:00Z",
      "updatedAt": "2026-02-11T20:00:00Z"
    }
  ]
}

```

## 2\. Get account details

View detailed account information including balances:

```
cdpcurl -k $CDP_API_KEY \
  'https://sandbox.cdp.coinbase.com/platform/v2/accounts/YOUR_ACCOUNT_ID'

```

Example response

```
{
  "accountId": "account_db458f63-418a-4a91-a045-fab93ac35c3f",
  "name": "My Test Account",
  "balances": [
    {
      "asset": "usd",
      "amount": "1000.00"
    },
    {
      "asset": "usdc",
      "amount": "500.00"
    }
  ],
  "createdAt": "2026-02-11T20:00:00Z",
  "updatedAt": "2026-02-11T23:00:00Z"
}

```

## Using Portal UI

You can create, manage, and fund accounts through the Portal UI:

## What to read next