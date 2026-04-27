# transfers

## Overview

Transfers move funds to external addresses, other Coinbase users, or payment methods. In Sandbox, all transfers are simulated—webhooks fire and status transitions occur, but no real transactions happen.

## Prerequisites

Before you begin, you need `cdpcurl`, a Sandbox API key, and a funded account. See the [Quickstart](https://developer.chrome.com/api-reference/payment-apis/sandbox/quickstart) for instructions.

## 1\. Simulate transfer by target type

Set your account ID:

```
export ACCOUNT_ID="account_db458f63-..."

```

Get your account ID

```
cdpcurl -k $CDP_API_KEY \
  'https://sandbox.cdp.coinbase.com/platform/v2/accounts' | sed '1d' | jq -r '.accounts[].accountId'

```

### a. To reserved addresses

Use these reserved addresses to test specific success and failure scenarios:

Reserved Address

Simulated Outcome

`0x1111111111111111111111111111111111111111`

Success

`0x2222222222222222222222222222222222222222`

Transfer invalid target

`0x3333333333333333333333333333333333333333`

Invalid address

`0x4444444444444444444444444444444444444444`

Unsupported network

-   Success
    
-   Transfer invalid target
    
-   Invalid address
    
-   Unsupported network
    

Below is a simulation using `0x1111...` to test successful transfer handling:

```
cdpcurl -k $CDP_API_KEY \
  -X POST \
  -d "{
    \"source\": {
      \"accountId\": \"$ACCOUNT_ID\",
      \"asset\": \"usdc\"
    },
    \"target\": {
      \"network\": \"base\",
      \"address\": \"0x1111111111111111111111111111111111111111\",
      \"asset\": \"usdc\"
    },
    \"amount\": \"5.00\",
    \"asset\": \"usdc\",
    \"execute\": true
  }" \
  'https://sandbox.cdp.coinbase.com/platform/v2/transfers'

```

You should expect: HTTP `2xx` with normal transfer response

Below is a simulation using `0x2222...` to test malformed target error handling:

```
cdpcurl -k $CDP_API_KEY \
  -X POST \
  -d "{
    \"source\": {
      \"accountId\": \"$ACCOUNT_ID\",
      \"asset\": \"usdc\"
    },
    \"target\": {
      \"network\": \"base\",
      \"address\": \"0x2222222222222222222222222222222222222222\",
      \"asset\": \"usdc\"
    },
    \"amount\": \"5.00\",
    \"asset\": \"usdc\",
    \"execute\": true
  }" \
  'https://sandbox.cdp.coinbase.com/platform/v2/transfers'

```

You should expect: HTTP `400`

```
{
  "errorType": "invalid_request",
  "errorMessage": "'target' is invalid: must match one of [Account, Payment Method, Onchain Address, Email Instrument]. Account requires 'accountId'; Payment Method requires 'paymentMethodId'; Onchain Address requires 'network'; Email Instrument requires 'email'"
}

```

Below is a simulation using `0x3333...` to test invalid address error handling:

```
cdpcurl -k $CDP_API_KEY \
  -X POST \
  -d "{
    \"source\": {
      \"accountId\": \"$ACCOUNT_ID\",
      \"asset\": \"usdc\"
    },
    \"target\": {
      \"network\": \"base\",
      \"address\": \"0x3333333333333333333333333333333333333333\",
      \"asset\": \"usdc\"
    },
    \"amount\": \"5.00\",
    \"asset\": \"usdc\",
    \"execute\": true
  }" \
  'https://sandbox.cdp.coinbase.com/platform/v2/transfers'

```

You should expect: HTTP `400`

```
{
  "errorType": "invalid_request",
  "errorMessage": "Invalid onchain address for network base."
}

```

Below is a simulation using `0x4444...` to test invalid network parameter handling:

```
cdpcurl -k $CDP_API_KEY \
  -X POST \
  -d "{
    \"source\": {
      \"accountId\": \"$ACCOUNT_ID\",
      \"asset\": \"usdc\"
    },
    \"target\": {
      \"network\": \"base\",
      \"address\": \"0x4444444444444444444444444444444444444444\",
      \"asset\": \"usdc\"
    },
    \"amount\": \"5.00\",
    \"asset\": \"usdc\",
    \"execute\": true
  }" \
  'https://sandbox.cdp.coinbase.com/platform/v2/transfers'

```

You should expect: HTTP `400`

```
{
  "errorType": "invalid_request",
  "errorMessage": "base is not a supported network."
}

```

### b. To reserved email addresses

Send to a Coinbase user by email. In Sandbox, only whitelisted test emails work:

Test Email

Description

`testuser1@domain.com`

Returns successful validation

`testuser2@domain.com`

Returns successful validation

Get your account ID

```
cdpcurl -k $CDP_API_KEY \
  'https://sandbox.cdp.coinbase.com/platform/v2/accounts' | sed '1d' | jq -r '.accounts[].accountId'

```

```
cdpcurl -k $CDP_API_KEY \
  -X POST \
  -d "{
    \"source\": {
      \"accountId\": \"$ACCOUNT_ID\",
      \"asset\": \"usd\"
    },
    \"target\": {
      \"email\": \"testuser1@domain.com\",
      \"asset\": \"usd\"
    },
    \"amount\": \"5.00\",
    \"asset\": \"usd\",
    \"execute\": true
  }" \
  'https://sandbox.cdp.coinbase.com/platform/v2/transfers'

```

### c. To reserved payment methods

For testing fiat withdrawals to external banks (Fedwire, SWIFT), see the [Payment Methods guide](https://developer.chrome.com/api-reference/payment-apis/sandbox/guides/payment-methods). Payment methods have pre-built test scenarios for different use cases (e.g., active vs inactive methods).

## 2\. Validate before executing

Use `validateOnly: true` to verify recipient details (like email addresses or crypto addresses) before executing a transfer.

Get your account ID

```
cdpcurl -k $CDP_API_KEY \
  'https://sandbox.cdp.coinbase.com/platform/v2/accounts' | sed '1d' | jq -r '.accounts[].accountId'

```

```
cdpcurl -k $CDP_API_KEY \
  -X POST \
  -d "{
    \"source\": {
      \"accountId\": \"$ACCOUNT_ID\",
      \"asset\": \"usd\"
    },
    \"target\": {
      \"email\": \"testuser1@domain.com\",
      \"asset\": \"usd\"
    },
    \"amount\": \"5.00\",
    \"asset\": \"usd\",
    \"validateOnly\": true
  }" \
  'https://sandbox.cdp.coinbase.com/platform/v2/transfers'

```

A `2xx` response means the transfer would succeed. A `4xx` response includes an `errorType` explaining why validation failed.

## 3\. List transfers

View all transfers for your account:

```
cdpcurl -k $CDP_API_KEY \
  'https://sandbox.cdp.coinbase.com/platform/v2/transfers'

```

Example response

```
{
  "transfers": [
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
  ]
}

```

## What to read next