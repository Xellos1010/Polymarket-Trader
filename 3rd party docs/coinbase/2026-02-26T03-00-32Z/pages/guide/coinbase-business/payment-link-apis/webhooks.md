# webhooks

Payment Link webhooks provide your app with real-time payment status updates. By subscribing your webhook endpoint you will receive a notification every time a payment link status changes.

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
  "description": "Payment link status webhook",
  "eventTypes": [
    "payment_link.payment.success",
    "payment_link.payment.failed",
    "payment_link.payment.expired"
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

-   `labels` is a required field. You must include it as an empty object (`{}`)
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

-   All Payment Link event types should be included to ensure you receive notifications for every payment link state change:

Event type

Description

`payment_link.payment.success`

Payment link successfully paid

`payment_link.payment.failed`

Payment link payment failed

`payment_link.payment.expired`

Payment link expired without payment

2.  Create the webhook subscription:

```
cdpcurl -X POST \
  -i "YOUR_API_KEY_ID" \
  -s "YOUR_API_KEY_SECRET" \
  "https://api.cdp.coinbase.com/platform/v2/data/webhooks/subscriptions" \
  -d '{
    "description": "Payment link status webhook",
    "eventTypes": [
      "payment_link.payment.success",
      "payment_link.payment.failed",
      "payment_link.payment.expired"
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
  "description": "Payment link status webhook",
  "eventTypes": [
    "payment_link.payment.success",
    "payment_link.payment.failed",
    "payment_link.payment.expired"
  ],
  "isEnabled": true,
  "labelKey": "user_uuid",
  "labelValue": "<YOUR_USER_UUID>",
  "labels": {
    "user_uuid": "<YOUR_USER_UUID>"
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
    "description": "Updated: Payment link status webhook",
    "eventTypes": [
      "payment_link.payment.success",
      "payment_link.payment.failed",
      "payment_link.payment.expired"
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
        // Handle your payment link event here
        res.status(200).send("OK");
    } else {
        console.log("❌ Invalid webhook - rejected");
        res.status(400).send("Invalid signature");
    }
});

```

### Next steps

Once your subscription is created, your endpoint will begin receiving webhook events for payment links!

#### Sample payment link event payloads

```
// Payment link success event
{
  "address": "0x54a6cCC8e3AA5BB34469dA2C4fB016E87B0DaaC9",
  "amount": "100.00",
  "createdAt": "2025-11-13T20:15:49.329Z",
  "currency": "USDC",
  "description": "Payment for order #12345",
  "eventType": "payment_link.payment.success",
  "expiresAt": "2026-12-31T23:59:59Z",
  "failRedirectUrl": "https://example.com/failed",
  "id": "69163c762331ed43dc64a6ef",
  "metadata": {
    "customerId": "cust_abc123",
    "orderId": "12345"
  },
  "network": "base",
  "settlement": {
    "feeAmount": "1.25",
    "netAmount": "98.75",
    "totalAmount": "100.00"
  },
  "status": "COMPLETED",
  "successRedirectUrl": "https://example.com/success",
  "tokenAddress": "0x036CbD53842c5426634e7929541eC2318f3dCF7e",
  "transactionHash": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
  "updatedAt": "2025-11-13T20:16:55.097011545Z",
  "url": "https://payments.coinbase.com/payment-links/pl_01k9zdrbdeezj98vaqaxrfare0"
}
// Payment link failed event
{
  "address": "0x54a6cCC8e3AA5BB34469dA2C4fB016E87B0DaaC9",
  "amount": "100.00",
  "createdAt": "2025-11-13T20:15:49.329Z",
  "currency": "USDC",
  "description": "Payment for order #12345",
  "eventType": "payment_link.payment.failed",
  "expiresAt": "2026-12-31T23:59:59Z",
  "failRedirectUrl": "https://example.com/failed",
  "id": "69163c762331ed43dc64a6ef",
  "metadata": {
    "customerId": "cust_abc123",
    "orderId": "12345"
  },
  "network": "base",
  "settlement": {
    "feeAmount": "1.25",
    "netAmount": "98.75",
    "totalAmount": "100.00"
  },
  "status": "FAILED",
  "successRedirectUrl": "https://example.com/success",
  "tokenAddress": "0x036CbD53842c5426634e7929541eC2318f3dCF7e",
  "updatedAt": "2025-11-13T20:16:55.097011545Z",
  "url": "https://payments.coinbase.com/payment-links/pl_01k9zdrbdeezj98vaqaxrfare0"
}
// Payment link expired event
{
  "address": "0x54a6cCC8e3AA5BB34469dA2C4fB016E87B0DaaC9",
  "amount": "100.00",
  "createdAt": "2025-11-13T20:15:49.329Z",
  "currency": "USDC",
  "description": "Payment for order #12345",
  "eventType": "payment_link.payment.expired",
  "expiresAt": "2026-12-31T23:59:59Z",
  "failRedirectUrl": "https://example.com/failed",
  "id": "69163c762331ed43dc64a6ef",
  "metadata": {
    "customerId": "cust_abc123",
    "orderId": "12345"
  },
  "network": "base",
  "settlement": {
    "feeAmount": "1.25",
    "netAmount": "98.75",
    "totalAmount": "100.00"
  },
  "status": "EXPIRED",
  "successRedirectUrl": "https://example.com/success",
  "tokenAddress": "0x036CbD53842c5426634e7929541eC2318f3dCF7e",
  "updatedAt": "2025-11-13T20:16:55.097011545Z",
  "url": "https://payments.coinbase.com/payment-links/pl_01k9zdrbdeezj98vaqaxrfare0"
}

```