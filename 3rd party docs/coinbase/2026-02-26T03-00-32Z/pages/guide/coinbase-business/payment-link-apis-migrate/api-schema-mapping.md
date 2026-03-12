# api schema mapping

## API endpoint mapping

### Create a payment request

-   Charge API
    
-   Payment Link API
    

```
POST https://api.commerce.coinbase.com/charges
X-CC-Api-Key: YOUR_API_KEY
{
  "name": "Order #12345",
  "description": "Payment for order #12345",
  "pricing_type": "fixed_price",
  "local_price": {
    "amount": "100.00",
    "currency": "USD"
  },
  "redirect_url": "https://example.com/success",
  "cancel_url": "https://example.com/cancel",
  "metadata": {
    "order_id": "12345",
    "customer_id": "cust_abc123"
  }
}

```

```
POST https://business.coinbase.com/api/v1/payment-links
Authorization: Bearer YOUR_JWT_TOKEN
X-Idempotency-Key: 8e03978e-40d5-43e8-bc93-6894a57f9324
{
  "amount": "100.00",
  "currency": "USDC",
  "network": "base",
  "description": "Payment for order #12345",
  "expiresAt": "2024-12-31T23:59:59Z",
  "successRedirectUrl": "https://example.com/success",
  "failRedirectUrl": "https://example.com/failed",
  "metadata": {
    "order_id": "12345",
    "customer_id": "cust_abc123"
  }
}

```

Key changes

-   Amount is now a direct string field (not nested in `local_price`)
-   Currency is specified directly (Payment Link API currently supports USDC)
-   Network specification is now explicit (defaults to `base`)
-   Redirect URLs have been renamed for clarity
-   `name` field is removed; use `description` instead
-   Idempotency is supported via `X-Idempotency-Key` header

### List payment requests

-   Charge API
    
-   Payment Link API
    

```
GET https://api.commerce.coinbase.com/charges
X-CC-Api-Key: YOUR_API_KEY

```

```
GET https://business.coinbase.com/api/v1/payment-links?pageSize=20&status=ACTIVE
Authorization: Bearer YOUR_JWT_TOKEN

```

Key changes

-   Pagination uses `pageSize` and `pageToken` instead of Commerce’s cursor-based approach
-   Can filter by status (ACTIVE, DEACTIVATED, EXPIRED, COMPLETED, FAILED)
-   Response includes `nextPageToken` for retrieving next page

### Retrieve a specific payment request

-   Charge API
    
-   Payment Link API
    

```
GET https://api.commerce.coinbase.com/charges/{charge_code_or_charge_id}
X-CC-Api-Key: YOUR_API_KEY

```

```
GET https://business.coinbase.com/api/v1/payment-links/{paymentLinkId}
Authorization: Bearer YOUR_JWT_TOKEN

```

Key changes

-   Use the payment link ID (24-character hexadecimal format)
-   No support for retrieval by code (use ID only)

### Cancel/deactivate a payment request

-   Charge API
    
-   Payment Link API
    

```
No explicit endpoint available
- Charges expire based on expires_at timestamp
- Cannot manually cancel active charges

```

```
POST https://business.coinbase.com/api/v1/payment-links/{paymentLinkId}/deactivate
Authorization: Bearer YOUR_JWT_TOKEN

```

Key changes

-   Explicit deactivation endpoint available
-   Deactivated payment links cannot accept further payments
-   Status changes to `DEACTIVATED`

## Response schema mapping

### Payment details

Charge API Field

Payment Link API Field

Notes

`id`

`id`

Format changed from UUID to 24-character hexadecimal

`code`

N/A

Code not available in Payment Link API

`hosted_url`

`url`

The shareable payment URL

`name`

`description`

Name field replaced by description

`description`

`description`

Same purpose

`pricing.local.amount`

`amount`

Flattened structure

`pricing.local.currency`

`currency`

Flattened structure

N/A

`network`

New field specifying blockchain network

N/A

`address`

New field with destination blockchain address

N/A

`tokenAddress`

New field with token contract address

`expires_at`

`expiresAt`

Same purpose, RFC 3339 format

`metadata`

`metadata`

Same purpose

`redirect_url`

`successRedirectUrl`

Renamed for clarity

`cancel_url`

`failRedirectUrl`

Renamed for clarity

`created_at`

`createdAt`

Same purpose, RFC 3339 format

N/A

`updatedAt`

New field tracking last update

`timeline`

`status`

Status now a single field instead of timeline array

`web3_data`

N/A

Transaction details handled differently

## Status mapping

Understanding status transitions is crucial for order fulfillment:

Charge API Status

Payment Link API Status

Description

NEW

ACTIVE

Payment request is active and awaiting payment

SIGNED

ACTIVE

In Payment Link, signing happens within the payment flow

PENDING

*(Check via transaction monitoring)*

Payment detected, awaiting confirmation

COMPLETED

COMPLETED

Payment confirmed and finalized

EXPIRED

EXPIRED

Payment request expired without payment

FAILED

FAILED

Payment attempt failed

N/A

DEACTIVATED

Payment link manually deactivated

## Webhooks

### Charge API webhooks

Commerce provides webhook events:

-   `charge:created`
-   `charge:pending`
-   `charge:confirmed`
-   `charge:failed`

### Payment Link API webhooks

Payment Link API now supports webhooks for real-time payment status notifications:

-   `payment_link.payment.success` - Payment link successfully paid
-   `payment_link.payment.failed` - Payment link payment failed
-   `payment_link.payment.expired` - Payment link expired without payment

See the [Webhooks documentation](https://developer.chrome.com/coinbase-business/payment-link-apis/webhooks) for detailed setup instructions, including:

-   Creating webhook subscriptions using CDP API
-   Webhook signature verification
-   Sample event payloads

## Node.js implementation examples

-   Charge API
    
-   Payment Link API
    

```
const axios = require('axios');
async function createCharge(orderData) {
  const response = await axios.post(
    'https://api.commerce.coinbase.com/charges',
    {
      name: orderData.name,
      description: orderData.description,
      pricing_type: 'fixed_price',
      local_price: {
        amount: orderData.amount,
        currency: 'USD'
      },
      redirect_url: orderData.successUrl,
      cancel_url: orderData.cancelUrl,
      metadata: orderData.metadata
    },
    {
      headers: {
        'X-CC-Api-Key': process.env.COMMERCE_API_KEY,
        'Content-Type': 'application/json'
      }
    }
  );
  return {
    id: response.data.id,
    hostedUrl: response.data.hosted_url,
    expiresAt: response.data.expires_at
  };
}
async function getChargeStatus(chargeId) {
  const response = await axios.get(
    `https://api.commerce.coinbase.com/charges/${chargeId}`,
    {
      headers: {
        'X-CC-Api-Key': process.env.COMMERCE_API_KEY
      }
    }
  );
  const timeline = response.data.timeline;
  const currentStatus = timeline[timeline.length - 1].status;
  return currentStatus;
}

```

```
const axios = require('axios');
const { v4: uuidv4 } = require('uuid');
// Helper function to generate JWT bearer token
// See: https://docs.cdp.coinbase.com/api-reference/v2/authentication
function generateBearerToken() {
  // Implementation depends on your CDP API key setup
  // This is a placeholder - refer to authentication docs
  return 'YOUR_JWT_TOKEN';
}
async function createPaymentLink(orderData) {
  const bearerToken = generateBearerToken();
  const idempotencyKey = uuidv4();
  const response = await axios.post(
    'https://business.coinbase.com/api/v1/payment-links',
    {
      amount: orderData.amount,
      currency: 'USDC',
      network: 'base',
      description: orderData.description,
      expiresAt: new Date(Date.now() + 24 * 60 * 60 * 1000).toISOString(), // 24 hours
      successRedirectUrl: orderData.successUrl,
      failRedirectUrl: orderData.failUrl,
      metadata: orderData.metadata
    },
    {
      headers: {
        'Authorization': `Bearer ${bearerToken}`,
        'Content-Type': 'application/json',
        'X-Idempotency-Key': idempotencyKey
      }
    }
  );
  return {
    id: response.data.id,
    url: response.data.url,
    address: response.data.address,
    tokenAddress: response.data.tokenAddress,
    expiresAt: response.data.expiresAt
  };
}
async function getPaymentLinkStatus(paymentLinkId) {
  const bearerToken = generateBearerToken();
  const response = await axios.get(
    `https://business.coinbase.com/api/v1/payment-links/${paymentLinkId}`,
    {
      headers: {
        'Authorization': `Bearer ${bearerToken}`
      }
    }
  );
  return response.data.status;
}
async function deactivatePaymentLink(paymentLinkId) {
  const bearerToken = generateBearerToken();
  const response = await axios.post(
    `https://business.coinbase.com/api/v1/payment-links/${paymentLinkId}/deactivate`,
    {},
    {
      headers: {
        'Authorization': `Bearer ${bearerToken}`
      }
    }
  );
  return response.data;
}

```