# quickstart

## Overview

Get started with Onchain Webhooks in just a few steps. This guide will help you create a webhook subscription via our [REST endpoints](https://developer.chrome.com/api-reference/v2/rest-api/onchain-data/onchain-data) and receive the events at a target destination.

## Prerequisites

## 1\. Construct subscription payload

Create a JSON payload to be used with `cdpcurl` in the next step:

```
{
  "description": "USD Base Coin Transfers",
  "eventTypes": [
    "onchain.activity.detected",
  ],
  "target": {
    "url": "https://your-webhook-url.com",
    "method": "POST"
  },
  "labels": {
    "contract_address": "0xd9aaec86b65d86f6a7b5b1b0c42ffa531710b6ca", # USD Base Coin Contract Address
    "event_name": "Transfer",
  },
  "isEnabled": true
}

```

### Configuration fields

Below is a list of all payload information that can be provided when creating a webhook subscription:

Field

Description

Required

Notes

`target.url`

Your webhook endpoint URL

Yes

Must be a valid HTTPS URL

`labels.contract_address`

Smart contract address to monitor

Yes

Hex address with `0x` prefix

`labels.event_name`

Smart contract event name

Yes\* (this OR `event_signature`)

Event name from ABI (e.g., `Transfer`)

`labels.event_signature`

Smart contract event signature

Yes\* (this OR `event_name`)

Full signature (e.g., `Transfer(address,address,uint256)`)

`eventTypes`

Array of event types

No

Use `["onchain.activity.detected"]` if provided

`isEnabled`

Enable/disable webhook

No

Defaults to `true`

`target.headers`

Custom HTTP headers

No

Object with header key-value pairs

`labels.transaction_from`

Transaction source address

No

`labels.transaction_to`

Transaction destination address

No

`labels.network`

Network name (e.g. `base-mainnet` or `base-sepolia`)

No

Defaults to `base-mainnet`

`labels.params.[any_param]`

Any smart contract parameter

No

Add any parameter from the contract event for hyper-granular filtering (e.g., `params.from`, `params.to`, `params.value`)

You can also set a `headers` object in `target` if your URL requires specific headers:

```
"target": {
    "url": "https://your-webhook-url.com",
    "method": "POST",
    "headers": {
      "custom-header": "value"
    }
},

```

## 2\. Create subscription

Using the configuration you created in the previous step, create the webhook subscription using `cdpcurl`:

```
cdpcurl -X POST \
  -i "YOUR_API_KEY_ID" \
  -s "YOUR_API_KEY_SECRET" \
  "https://api.cdp.coinbase.com/platform/v2/data/webhooks/subscriptions" \
  -d '{
  "description": "USD Base Coin Transfers",
  "eventTypes": [
    "onchain.activity.detected",
  ],
  "target": {
    "url": "https://your-webhook-url.com",
    "method": "POST"
  },
  "labels": {
    "contract_address": "0xd9aaec86b65d86f6a7b5b1b0c42ffa531710b6ca", # USD Base Coin Contract Address
    "event_name": "Transfer",
  },
  "isEnabled": true
}'

```

You should see a response similar to the following:

```
201 Created
{
  "createdAt": "2025-10-08T13:58:38.681893Z",
  "description": "USD Base Coin Transfers",
  "eventTypes": [
    "onchain.activity.detected"
  ],
  "isEnabled": true,
  "labels": {
    "project": "<YOUR_CDP_PROJECT_ID>",
    "contract_address": "0xd9aaec86b65d86f6a7b5b1b0c42ffa531710b6ca", # USD Base Coin Contract Address
    "event_name": "Transfer",
  },
  "metadata": {
    "secret": "<SECRET_FOR_WEBHOOK_VERIFICATION>"
  },
  "subscriptionId": "<YOUR_SUBSCRIPTION_ID>",
  "target": {
    "url": "https://your-webhook-url.com"
  }
}

```

## Additional endpoints

See the following examples to view, update, or delete the subscription using the `subscriptionId` from the response.

### List all subscriptions

```
cdpcurl -X GET \
  -i "YOUR_API_KEY_ID" \
  -s "YOUR_API_KEY_SECRET" \
  "https://api.cdp.coinbase.com/platform/v2/data/webhooks/subscriptions"

```

### View subscription details

```
cdpcurl -X GET \
  -i "YOUR_API_KEY_ID" \
  -s "YOUR_API_KEY_SECRET" \
  "https://api.cdp.coinbase.com/platform/v2/data/webhooks/subscriptions/<SUBSCRIPTION_ID>"

```

### Update subscription

```
cdpcurl -X PUT \
  -i "YOUR_API_KEY_ID" \
  -s "YOUR_API_KEY_SECRET" \
  "https://api.cdp.coinbase.com/platform/v2/data/webhooks/subscriptions/<SUBSCRIPTION_ID>" \
  -d '{
    "description": "Updated: USD Base Coin Transfers",
    "eventTypes": [
    "onchain.activity.detected",
    ],
    "target": {
      "url": "https://your-webhook-url.com",
      "method": "POST"
    },
    "labels": {},
    "isEnabled": true
  }'

```

### Delete subscription

```
cdpcurl -X DELETE \
  -i "YOUR_API_KEY_ID" \
  -s "YOUR_API_KEY_SECRET" \
  "https://api.cdp.coinbase.com/platform/v2/data/webhooks/subscriptions/<SUBSCRIPTION_ID>"

```

## What to read next

-   **[Verify webhook signatures](https://developer.chrome.com/data/webhooks/verify-signatures)**: Learn how to verify webhook signatures to ensure events are coming from Coinbase
-   **[REST API Reference](https://developer.chrome.com/api-reference/v2/rest-api/onchain-data/onchain-data)**: View the complete webhook API documentation
-   **[Support](https://developer.chrome.com/support/join-cdp-discord)**: Join our Discord for help and community support