# webhooks

Onramp webhooks provide your app with real-time transaction status updates. By subscribing your webhook endpoint you will receive a notification every time a transaction made by your users is created or updated.

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
  "description": "Onramp transaction status webhook",
  "eventTypes": [
    "onramp.transaction.created",
    "onramp.transaction.updated", 
    "onramp.transaction.success",
    "onramp.transaction.failed"
  ],
  "target": {
    "url": "https://your-webhook-url.com",
    "method": "POST"
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
    "method": "POST",
    "headers": {
      "custom-header": "value"
    }
},
...

```

-   All Onramp event types should be included to ensure you receive notifications for every Onramp transaction state change:

Event type

Description

`onramp.transaction.created`

New Onramp transaction created

`onramp.transaction.updated`

Onramp transaction status changed

`onramp.transaction.success`

Onramp transaction completed successfully

`onramp.transaction.failed`

Onramp transaction failed

Offramp transactions are also supported:

Event type

Description

`offramp.transaction.created`

New Offramp transaction created

`offramp.transaction.updated`

Offramp transaction status changed

`offramp.transaction.success`

Offramp transaction completed successfully

`offramp.transaction.failed`

Offramp transaction failed

2.  Create the webhook subscription:

```
cdpcurl -X POST \
  -i "YOUR_API_KEY_ID" \
  -s "YOUR_API_KEY_SECRET" \
  "https://api.cdp.coinbase.com/platform/v2/data/webhooks/subscriptions" \
  -d '{
    "description": "Onramp transaction status webhook",
    "eventTypes": [
      "onramp.transaction.created",
      "onramp.transaction.updated",
      "onramp.transaction.success",
      "onramp.transaction.failed"
    ],
    "target": {
      "url": "https://your-webhook-url.com",
      "method": "POST"
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
  "description": "Onramp transaction status webhook",
  "eventTypes": [
    "onramp.transaction.created",
    "onramp.transaction.updated",
    "onramp.transaction.success",
    "onramp.transaction.failed"
  ],
  "isEnabled": true,
  "labelKey": "project",
  "labelValue": "<YOUR_CDP_PROJECT_ID>",
  "labels": {
    "project": "<YOUR_CDP_PROJECT_ID>"
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
    "description": "Updated: Onramp transaction status webhook",
    "eventTypes": [
      "onramp.transaction.created",
      "onramp.transaction.updated",
      "onramp.transaction.success",
      "onramp.transaction.failed"
    ],
    "target": {
      "url": "https://your-new-webhook-url.com",
      "method": "POST"
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

When you create a webhook subscription, the response includes a top-level `secret`. Use this value to verify that incoming webhooks are authentic. Each webhook request includes an `X-Hook0-Signature` header containing:

-   `t` field - the timestamp
-   `h` field - list of headers included in the signature
-   `v1` field - the signature

#### Implementation

Here’s an example of how to verify webhook signatures:

```
const crypto = require('crypto');
/**
 * Verify webhook signature and timestamp
 * @param {string} payload - Raw request body as string
 * @param {string} signatureHeader - X-Hook0-Signature header value  
 * @param {string} secret - Webhook secret from the subscription response
 * @param {Object} headers - HTTP headers from webhook request
 * @param {number} maxAgeMinutes - Max age for webhook (default: 5 minutes)
 * @returns {boolean} true if webhook is authentic and within allowed time window
 */
function verifyWebhookSignature(payload, signatureHeader, secret, headers, maxAgeMinutes = 5) {
    try {
        // Parse signature header: t=timestamp,h=headers,v1=signature
        const elements = signatureHeader.split(',');
        const timestamp = elements.find(e => e.startsWith('t=')).split('=')[1];
        const headerNames = elements.find(e => e.startsWith('h=')).split('=')[1];
        const providedSignature = elements.find(e => e.startsWith('v1=')).split('=')[1];
        
        // Build header values string
        const headerNameList = headerNames.split(' ');
        const headerValues = headerNameList.map(name => headers[name] || '').join('.');
        
        // Build signed payload
        const signedPayload = `${timestamp}.${headerNames}.${headerValues}.${payload}`;
        
        // Compute expected signature
        const expectedSignature = crypto
            .createHmac('sha256', secret)
            .update(signedPayload, 'utf8')
            .digest('hex');
        
        // Compare signatures securely
        const signaturesMatch = crypto.timingSafeEqual(
            Buffer.from(expectedSignature, 'hex'),
            Buffer.from(providedSignature, 'hex')
        );
        
        // Verify timestamp to prevent replay attacks
        const webhookTime = parseInt(timestamp) * 1000; // Convert to milliseconds
        const currentTime = Date.now();
        const ageMinutes = (currentTime - webhookTime) / (1000 * 60);
        
        if (ageMinutes > maxAgeMinutes) {
            console.error(`Webhook timestamp exceeds maximum age: ${ageMinutes.toFixed(1)} minutes > ${maxAgeMinutes} minutes`);
            return false;
        }
        
        return signaturesMatch;
        
    } catch (error) {
        console.error('Webhook verification error:', error);
        return false;
    }
}

```

And in your application:

```
const express = require("express");
const app = express();
// Important: Get raw body for signature verification
app.use(express.raw({ type: "application/json" }));
app.post("/webhook", (req, res) => {
    const payload = req.body.toString(); // Raw string needed for signature
    const signature = req.headers["x-hook0-signature"];
    const secret = process.env.WEBHOOK_SECRET; // Store securely from the subscription response
    
    if (verifyWebhookSignature(payload, signature, secret, req.headers)) {
        console.log("✅ Authentic webhook");
        
        // Parse and process the event
        const event = JSON.parse(payload);
        
        // Handle your transaction event here
        
        res.status(200).send("OK");
    } else {
        console.log("❌ Invalid webhook - rejected");
        res.status(400).send("Invalid signature");
    }
});

```

### Next steps

Once your subscription is created, your endpoint will begin receiving webhook events for transactions!

#### Sample transaction event payloads

```
// Guest checkout transaction event
{
  "coinbaseFee": {
    "currency": "USD",
    "value": "0"
  },
  "completedAt": "0001-01-01T00:00:00Z",
  "country": "US",
  "createdAt": "2025-09-02T02:34:13Z",
  "eventType": "onramp.transaction.updated",
  "exchangeRate": {
    "currency": "USDC",
    "value": "1"
  },
  "networkFee": {
    "currency": "USD",
    "value": "0.19"
  },
  "paymentMethod": "CARD",
  "paymentSubtotal": {
    "currency": "USD",
    "value": "4.81"
  },
  "paymentTotal": {
    "currency": "USD",
    "value": "5"
  },
  "paymentTotalUsd": {
    "currency": "USD",
    "value": "5"
  },
  "purchaseAmount": {
    "currency": "USDC",
    "value": "4.81"
  },
  "purchaseCurrency": "USDC",
  "purchaseNetwork": "ethereum",
  "status": "ONRAMP_TRANSACTION_STATUS_IN_PROGRESS",
  "transactionId": "1f087a54-ff1f-62e8-9f85-aa77ac0499a5",
  "txHash": "0x",
  "type": "ONRAMP_TRANSACTION_TYPE_BUY_AND_SEND",
  "userId": "4132b63ee21128686458155b28570289",
  "userType": "USER_TYPE_GUEST",
  "walletAddress": "0xe0512E358C347cc2b1A42d057065CE642068b7Ba",
  "partnerUserRef": "example_user_ref"
}
// Apple Pay Onramp API order event
{
  "orderId": "123e4567-e89b-12d3-a456-426614174000",
  "eventType": "onramp.transaction.success",
  "paymentTotal": "100.75",
  "paymentSubtotal": "100",
  "paymentCurrency": "USD",
  "paymentMethod": "GUEST_CHECKOUT_APPLE_PAY",
  "purchaseAmount": "100.000000",
  "purchaseCurrency": "USDC",
  "fees": [
    {
      "feeType": "FEE_TYPE_NETWORK",
      "feeAmount": "0.5",
      "feeCurrency": "USD"
    },
    {
      "feeType": "FEE_TYPE_EXCHANGE",
      "feeAmount": "0.25",
      "feeCurrency": "USD"
    }
  ],
  "exchangeRate": "1",
  "destinationAddress": "0x1234567890abcdef1234567890abcdef12345678",
  "destinationNetwork": "base",
  "status": "ONRAMP_ORDER_STATUS_COMPLETED",
  "txHash": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
  "createdAt": "2025-09-10T10:30:00Z",
  "updatedAt": "2025-09-10T10:35:00Z",
  "partnerUserRef": "example_user_ref"
}

```

## Best practices

To ensure reliable webhook delivery:

-   **Test endpoints locally** before enabling subscriptions in production
-   **Handle concurrent requests** - ensure your target URL can process multiple events simultaneously
-   **Process events asynchronously** - return a `200` response quickly and process the event in the background
-   **Monitor webhook receiver health** - track delivery success rates to your target URL
-   **Set up subscription monitoring** - use a scheduled job (e.g., cron, systemd timer) to periodically call the List Subscriptions API and verify critical subscriptions have `isEnabled: true`