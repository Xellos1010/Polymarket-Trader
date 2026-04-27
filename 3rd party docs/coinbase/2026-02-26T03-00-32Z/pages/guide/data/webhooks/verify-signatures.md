# verify signatures

## Overview

Verifying webhook signatures ensures that incoming webhooks are authentic and sent by Coinbase, protecting your application from malicious requests and replay attacks.

Without signature verification, your webhook endpoint is vulnerable to:

-   **Spoofed webhooks**: Attackers could send fake event data to your endpoint
-   **Replay attacks**: Old webhook events could be resent to trigger duplicate processing
-   **Man-in-the-middle attacks**: Modified webhook payloads could go undetected

## How it works

When you create a webhook subscription, the response includes a `secret` that serves as your signing key. Each webhook request includes an `X-Hook0-Signature` header that looks like this:

```
t=1728394718,h=content-type x-hook0-id,v1=a1b2c3d4e5f6...

```

The signature header contains three parts:

Field

Description

Example

`t`

Unix timestamp when the webhook was sent

`1728394718`

`h`

Space-separated list of header names included in signature

`content-type x-hook0-id`

`v1`

HMAC-SHA256 signature of the payload

`a1b2c3d4e5f6...`

Verification process

1.  **Extract signature components**: Parse the `t`, `h`, and `v1` values from the header
2.  **Build signed payload**: Concatenate `timestamp.headerNames.headerValues.rawBody`
3.  **Compute expected signature**: Create HMAC-SHA256 hash using your secret
4.  **Compare signatures**: Use timing-safe comparison to match expected vs. provided
5.  **Verify timestamp**: Ensure the webhook isn’t too old (prevents replay attacks)

### 1\. Create a verification function

First, handle the verification logic in a reusable function which will:

-   Parse the signature header to extract the timestamp, header names, and signature
-   Build the signed payload by concatenating the timestamp, headers, and raw body
-   Compute the expected signature using HMAC-SHA256
-   Compare the signatures using a timing-safe comparison to prevent timing attacks
-   Validate the timestamp to ensure the webhook isn’t too old (replay attack prevention)

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

### 2\. Verify webhooks in your application

Now integrate the verification function into your webhook endpoint. This example shows:

-   How to configure Express to preserve the raw request body (required for signature verification)
-   How to extract the signature header and webhook secret
-   How to call the verification function before processing the webhook
-   How to handle both valid and invalid webhooks appropriately

```
const express = require("express");
const app = express();
// Important: Get raw body for signature verification
app.use(express.raw({ type: "application/json" }));
app.post("/webhook", (req, res) => {
    // Step 1: Extract the raw payload (must be string for signature verification)
    const payload = req.body.toString();
    
    // Step 2: Get the signature from the X-Hook0-Signature header
    const signature = req.headers["x-hook0-signature"];
    
    // Step 3: Get your webhook secret (from the subscription response)
    const secret = process.env.WEBHOOK_SECRET;
    
    // Step 4: Verify the webhook signature
    if (verifyWebhookSignature(payload, signature, secret, req.headers)) {
        console.log("✅ Authentic webhook");
        
        // Step 5: Parse the JSON payload (only after verification!)
        const event = JSON.parse(payload);
        
        // Step 6: Process your webhook event
        console.log("Transaction detected:", event.data.transactionHash);
        // Add your business logic here...
        
        // Step 7: Return 200 to acknowledge receipt
        res.status(200).send("OK");
    } else {
        console.log("❌ Invalid webhook - rejected");
        res.status(400).send("Invalid signature");
    }
});

```

### Example webhook payload

Here’s what a complete webhook request looks like:

```
{
  "id": "evt_1a2b3c4d5e6f",
  "type": "onchain.activity.detected",
  "createdAt": "2025-10-08T13:58:38.681893Z",
  "data": {
    "subscriptionId": "sub_abc123",
    "networkId": "base-mainnet",
    "blockNumber": 12345678,
    "blockHash": "0xabc123...",
    "transactionHash": "0xdef456...",
    "logIndex": 42,
    "contractAddress": "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913",
    "eventName": "Transfer",
    "from": "0xf20d2e37514195ebedb0bc735ba6090ce103d38c",
    "to": "0x1234567890123456789012345678901234567890",
    "value": "1000000"
  }
}

```

The webhook request will include these HTTP headers:

```
POST /webhook HTTP/1.1
Host: your-webhook-url.com
Content-Type: application/json
X-Hook0-Signature: t=1728394718,h=content-type x-hook0-id,v1=a1b2c3d4e5f6...
X-Hook0-Id: evt_1a2b3c4d5e6f
Content-Length: 512

```

## Security best practices

## Error handling

Handle common verification failures gracefully:

```
app.post('/webhook', (req, res) => {
  const payload = req.body.toString();
  const signature = req.headers['x-hook0-signature'];
  const secret = process.env.WEBHOOK_SECRET;
  
  // Check for missing signature
  if (!signature) {
    console.error('Missing X-Hook0-Signature header');
    return res.status(400).send('Missing signature');
  }
  
  // Check for missing secret
  if (!secret) {
    console.error('Webhook secret not configured');
    return res.status(500).send('Server configuration error');
  }
  
  try {
    const isValid = verifyWebhookSignature(payload, signature, secret, req.headers);
    
    if (!isValid) {
      console.error('Invalid webhook signature');
      return res.status(400).send('Invalid signature');
    }
    
    // Process the webhook
    const event = JSON.parse(payload);
    
    // Handle specific event types
    if (event.type === 'onchain.activity.detected') {
      // Process transaction event
      console.log('Transaction detected:', event.data.transactionHash);
    }
    
    res.status(200).send('OK');
    
  } catch (error) {
    console.error('Webhook processing error:', error);
    res.status(500).send('Processing error');
  }
});

```

## What to read next

-   **[Quickstart](https://developer.chrome.com/data/webhooks/quickstart)**: Set up your first webhook subscription
-   **[REST API Reference](https://developer.chrome.com/api-reference/v2/rest-api/onchain-data/onchain-data)**: View the complete webhook API documentation