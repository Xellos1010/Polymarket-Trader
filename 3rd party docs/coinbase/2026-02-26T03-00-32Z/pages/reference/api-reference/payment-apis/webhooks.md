# webhooks

CDP Transfers webhooks provide your app with real-time transfer status updates. By subscribing your webhook endpoint you will receive a notification every time a transfer made by your users is created or updated.

## Setup

### Prerequisites

You will need:

-   Your CDP API Key ID and secret
-   A webhook notification HTTPS URL
-   (Recommended) Install [cdpcurl](https://github.com/coinbase/cdpcurl)

### Create a webhook subscription

1.  Prepare your subscription configuration:

```
{
  "description": "CDP Transfers webhook",
  "eventTypes": [
    "payments.transfers.quoted",
    "payments.transfers.processing",
    "payments.transfers.completed",
    "payments.transfers.failed"
  ],
  "target": {
    "url": "https://your-webhook-url.com"
  },
  "labels": {},
  "isEnabled": true
}

```

Important configuration notes:

-   `target.url` should be your webhook endpoint that will receive the events
-   You can also set a `headers` object in `target` if your url requires specific headers:

```
...
"target": {
    "url": "https://your-webhook-url.com",
    "headers": {
      "custom-header": "value"
    }
},
...

```

-   All Transfer event types should be included to ensure you receive notifications for every transfer state change:

Event type

Description

`payments.transfers.quoted`

Transfer has been quoted with fee breakdown

`payments.transfers.processing`

Transfer is being executed

`payments.transfers.completed`

Transfer completed successfully

`payments.transfers.failed`

Transfer failed

2.  Create the webhook subscription:

```
cdpcurl -X POST \
  -i "YOUR_API_KEY_ID" \
  -s "YOUR_API_KEY_SECRET" \
  "https://api.cdp.coinbase.com/platform/v2/data/webhooks/subscriptions" \
  -d '{
    "description": "CDP Transfers webhook",
    "eventTypes": [
      "payments.transfers.quoted",
      "payments.transfers.processing",
      "payments.transfers.completed",
      "payments.transfers.failed"
    ],
    "target": {
      "url": "https://your-webhook-url.com"
    },
    "labels": {},
    "isEnabled": true
  }'

```

Sample webhook subscription response:

```
201 Created
{
  "createdAt": "2025-09-10T13:58:38.681893Z",
  "description": "CDP Transfers webhook",
  "eventTypes": [
    "payments.transfers.quoted",
    "payments.transfers.processing",
    "payments.transfers.completed",
    "payments.transfers.failed"
  ],
  "isEnabled": true,
  "labels": {
    "entity": "<YOUR_ENTITY_ID>"
  },
  "secret": "<SECRET_FOR_WEBHOOK_VERIFICATION>",
  "subscriptionId": "<YOUR_SUBSCRIPTION_ID>",
  "target": {
    "url": "https://your-webhook-url.com"
  }
}

```

Once you’ve created the webhook subscription, you can use the `subscriptionId` from the response to view, update, or delete the subscription. **List all subscriptions**

```
cdpcurl -X GET \
  -i "YOUR_API_KEY_ID" \
  -s "YOUR_API_KEY_SECRET" \
  "https://api.cdp.coinbase.com/platform/v2/data/webhooks/subscriptions"

```

**View specified subscription details by subscription ID**

```
cdpcurl -X GET \
  -i "YOUR_API_KEY_ID" \
  -s "YOUR_API_KEY_SECRET" \
  "https://api.cdp.coinbase.com/platform/v2/data/webhooks/subscriptions/<SUBSCRIPTION_ID>"

```

**Update subscription**

```
cdpcurl -X PUT \
  -i "YOUR_API_KEY_ID" \
  -s "YOUR_API_KEY_SECRET" \
  "https://api.cdp.coinbase.com/platform/v2/data/webhooks/subscriptions/<SUBSCRIPTION_ID>" \
  -d '{
    "description": "Updated: CDP Transfers webhook",
    "eventTypes": [
      "payments.transfers.quoted",
      "payments.transfers.processing",
      "payments.transfers.completed",
      "payments.transfers.failed"
    ],
    "target": {
      "url": "https://your-new-webhook-url.com"
    },
    "labels": {},
    "isEnabled": true
  }'

```

**Delete subscription**

```
cdpcurl -X DELETE \
  -i "YOUR_API_KEY_ID" \
  -s "YOUR_API_KEY_SECRET" \
  "https://api.cdp.coinbase.com/platform/v2/data/webhooks/subscriptions/<SUBSCRIPTION_ID>"

```

### Webhook signature verification

#### How it works

When you create a webhook subscription, the response includes a `secret`. This secret is used to verify that incoming webhooks are authentic. Each webhook request includes an `X-Hook0-Signature` header containing:

-   `t` field - the timestamp
-   `h` field - list of headers included in the signature
-   `v1` field - the signature

#### Implementation

Here’s an example of how to verify webhook signatures:

And in your application:

### Next steps

Once your subscription is created, your endpoint will begin receiving webhook events for transfers!

#### Sample transfer event payloads

-   Quoted
    
-   Processing
    
-   Completed
    
-   Failed
    

Transfer fee quote is ready:

```
{
  "eventID": "<uuid>",
  "eventType": "payments.transfers.quoted",
  "timestamp": "2026-01-01T00:00:00Z",
  "data": {
    "createdAt": "2026-01-01T00:00:00Z",
    "expiresAt": "2026-01-01T00:15:00Z",
    "fees": [
      {
        "amount": "0.19",
        "asset": "usdc",
        "type": "network"
      }
    ],
    "source": {
      "accountId": "account_<uuid>",
      "asset": "usdc"
    },
    "sourceAmount": "100.19",
    "sourceAsset": "usdc",
    "status": "quoted",
    "target": {
      "address": "0x<address>",
      "asset": "usdc",
      "network": "ethereum"
    },
    "targetAmount": "100.00",
    "targetAsset": "usdc",
    "transferId": "transfer_<uuid>",
    "updatedAt": "2026-01-01T00:00:10Z"
  }
}

```

Transfer is being executed:

```
{
  "eventID": "<uuid>",
  "eventType": "payments.transfers.processing",
  "timestamp": "2026-01-01T00:02:30Z",
  "data": {
    "createdAt": "2026-01-01T00:00:00Z",
    "expiresAt": "2026-01-01T00:15:00Z",
    "fees": [
      {
        "amount": "0.19",
        "asset": "usdc",
        "type": "network"
      }
    ],
    "source": {
      "accountId": "account_<uuid>",
      "asset": "usdc"
    },
    "sourceAmount": "100.19",
    "sourceAsset": "usdc",
    "status": "processing",
    "target": {
      "address": "0x<address>",
      "asset": "usdc",
      "network": "ethereum"
    },
    "targetAmount": "100.00",
    "targetAsset": "usdc",
    "transferId": "transfer_<uuid>",
    "updatedAt": "2026-01-01T00:02:30Z"
  }
}

```

Transfer finished successfully:

```
{
  "eventID": "<uuid>",
  "eventType": "payments.transfers.completed",
  "timestamp": "2026-01-01T00:05:00Z",
  "data": {
    "createdAt": "2026-01-01T00:00:00Z",
    "expiresAt": "2026-01-01T00:15:00Z",
    "fees": [
      {
        "amount": "0.19",
        "asset": "usdc",
        "type": "network"
      }
    ],
    "source": {
      "accountId": "account_<uuid>",
      "asset": "usdc"
    },
    "sourceAmount": "100.19",
    "sourceAsset": "usdc",
    "status": "completed",
    "target": {
      "address": "0x<address>",
      "asset": "usdc",
      "network": "ethereum"
    },
    "targetAmount": "100.00",
    "targetAsset": "usdc",
    "transferId": "transfer_<uuid>",
    "updatedAt": "2026-01-01T00:05:00Z"
  }
}

```

Transfer failed:

```
{
  "eventID": "<uuid>",
  "eventType": "payments.transfers.failed",
  "timestamp": "2026-01-01T00:05:00Z",
  "data": {
    "createdAt": "2026-01-01T00:00:00Z",
    "expiresAt": "2026-01-01T00:15:00Z",
    "failureReason": "Insufficient Balance",
    "fees": [
      {
        "amount": "0.19",
        "asset": "usdc",
        "type": "network"
      }
    ],
    "source": {
      "accountId": "account_<uuid>",
      "asset": "usdc"
    },
    "sourceAmount": "100.19",
    "sourceAsset": "usdc",
    "status": "failed",
    "target": {
      "address": "0x<address>",
      "asset": "usdc",
      "network": "ethereum"
    },
    "targetAmount": "100.00",
    "targetAsset": "usdc",
    "transferId": "transfer_<uuid>",
    "updatedAt": "2026-01-01T00:05:00Z"
  }
}

```

## Best practices

To ensure reliable webhook delivery:

-   **Test endpoints locally** before enabling subscriptions in production
-   **Handle concurrent requests** - ensure your target URL can process multiple events simultaneously
-   **Process events asynchronously** - return a `200` response quickly and process the event in the background
-   **Monitor webhook receiver health** - track delivery success rates to your target URL
-   **Set up subscription monitoring** - use a scheduled job to periodically call the List Subscriptions API and verify critical subscriptions have `isEnabled: true`