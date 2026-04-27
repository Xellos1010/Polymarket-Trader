# webhooks

Invoicing webhooks provide your app with real-time invoice status updates. By subscribing your webhook endpoint you will receive a notification every time an invoice status changes.

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
  "description": "Invoice status webhook",
  "eventTypes": [
    "invoicing.invoice.scheduled",
    "invoicing.invoice.open",
    "invoicing.invoice.paid",
    "invoicing.invoice.overdue",
    "invoicing.invoice.void",
    "invoicing.invoice.deleted"
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

-   `labels` is a required field, but can be an empty object `{}`
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

-   All Invoicing event types should be included to ensure you receive notifications for every invoice state change:

Event type

Description

`invoicing.invoice.scheduled`

Invoice scheduled to be sent

`invoicing.invoice.open`

Invoice sent and awaiting payment

`invoicing.invoice.paid`

Invoice paid in full

`invoicing.invoice.overdue`

Invoice past due date

`invoicing.invoice.void`

Invoice voided

`invoicing.invoice.deleted`

Invoice deleted

2.  Create the webhook subscription:

```
cdpcurl -X POST \
  -i "YOUR_API_KEY_ID" \
  -s "YOUR_API_KEY_SECRET" \
  "https://api.cdp.coinbase.com/platform/v2/data/webhooks/subscriptions" \
  -d '{
    "description": "Invoice status webhook",
    "eventTypes": [
      "invoicing.invoice.scheduled",
      "invoicing.invoice.open",
      "invoicing.invoice.paid",
      "invoicing.invoice.overdue",
      "invoicing.invoice.void",
      "invoicing.invoice.deleted"
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
  "description": "Invoice status webhook",
  "eventTypes": [
    "invoicing.invoice.scheduled",
    "invoicing.invoice.open",
    "invoicing.invoice.paid",
    "invoicing.invoice.overdue",
    "invoicing.invoice.void",
    "invoicing.invoice.deleted"
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
    "description": "Updated: Invoice status webhook",
    "eventTypes": [
      "invoicing.invoice.scheduled",
      "invoicing.invoice.open",
      "invoicing.invoice.paid",
      "invoicing.invoice.overdue",
      "invoicing.invoice.void",
      "invoicing.invoice.deleted"
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

When you create a webhook subscription, the response includes a `secret`. Use this value to verify that incoming webhooks are authentic. Each webhook request includes an `X-Hook0-Signature` header containing:

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
        // Handle your invoice event here
        res.status(200).send("OK");
    } else {
        console.log("❌ Invalid webhook - rejected");
        res.status(400).send("Invalid signature");
    }
});

```

### Next steps

Once your subscription is created, your endpoint will begin receiving webhook events for invoices!

#### Sample invoice event payload

```
{
  "id": "68f7a946db0529ea9b6d3a12",
  "invoiceNumber": "INV-1234",
  "contactName": "John Doe",
  "contactEmail": "john.doe@example.com",
  "contactAddress": {
    "addressLine1": "123 Main Street",
    "city": "San Francisco",
    "state": "CA",
    "country": "US",
    "postalCode": "94103"
  },
  "dueDate": "2025-01-15T00:00:00Z",
  "sendDate": "2025-01-01T09:00:00Z",
  "invoiceDate": "2025-01-01T00:00:00Z",
  "lineItems": [
    {
      "itemName": "Web Development Services",
      "quantity": 10,
      "unitPrice": {
        "value": "100.00",
        "currency": "USDC"
      }
    }
  ],
  "totalAmountDue": {
    "value": "1000.00",
    "currency": "USDC"
  },
  "status": "PAID",
  "eventType": "invoicing.invoice.paid",
  "createdAt": "2025-01-01T08:30:00Z",
  "updatedAt": "2025-01-02T14:22:00Z",
  "createdBy": "750a84dd-2460-504c-8bb9-f6fa731a2361",
  "lastUpdatedBy": "750a84dd-2460-504c-8bb9-f6fa731a2361",
  "entityName": "Acme Corporation",
  "entityAddress": {
    "addressLine1": "456 Business Ave",
    "city": "San Francisco",
    "state": "CA",
    "country": "US",
    "postalCode": "94105"
  },
  "paymentMethod": {
    "crypto": {
      "paymentLinkUrl": "https://pay.coinbase.com/pl_01h8441j23abcd1234567890ef",
      "paymentLinkId": "68f7a946db0529ea9b6d3a12",
      "transactionHash": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
    }
  }
}

```