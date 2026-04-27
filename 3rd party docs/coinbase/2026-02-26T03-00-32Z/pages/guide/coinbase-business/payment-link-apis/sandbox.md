# sandbox

## Overview

The Payment Links API sandbox environment provides a safe, isolated testing space where you can develop and test your payment link integrations without affecting production data or processing real transactions. The sandbox mirrors production functionality while using test data and simulated payment flows.

## Key Differences: Sandbox vs Production

Feature

Sandbox

Production

**API Endpoint**

`business.coinbase.com/sandbox/api/v1/payment-links`

`business.coinbase.com/api/v1/payment-links`

**API Keys**

Same CDP API keys

Same CDP API keys

**Transactions**

Simulated (no real value)

Real payment processing

**Webhooks**

Requires `sandbox: true` label

No label required

**Rate Limits**

Same as production

Standard production limits

**Data Retention**

30 days (auto-purged)

Permanent

## Getting Started

### 1\. Prerequisites

Before using the sandbox environment, ensure you have:

-   A Coinbase Business account
-   CDP API credentials (same keys work for both sandbox and production)
-   Familiarity with the [Payment Links API](https://developer.chrome.com/coinbase-business/payment-link-apis/overview)

### 2\. Configure Your Application

The only change required to use sandbox is updating your API endpoint to include the `/sandbox` path segment:

## Testing Payments with Testnet Funds

To fully test your sandbox integration, you’ll need a wallet with testnet USDC on Base Sepolia. Here’s how to set one up:

### Step 1: Get a Wallet on Base Sepolia

You can use any EVM-compatible wallet. The easiest options for non-crypto developers:

-   Coinbase Wallet (Recommended)
    
-   MetaMask
    

1.  Download [Coinbase Wallet](https://chromewebstore.google.com/detail/coinbase-wallet-extension/hnfanknocfeofbddgcijnmhnfnkdnaad) (browser extension) or [Base](https://base.app/download) (mobile)
2.  Create a new wallet (no cryptocurrency purchase needed)
3.  In settings, enable **Developer Mode** or **Testnets**
4.  Switch network to **Base Sepolia**

Your wallet address will look like: `0x1234...abcd`

### Step 2: Get Testnet USDC from the Faucet

Use the Coinbase Developer Portal Faucet to claim free testnet USDC:

1.  Go to the [Coinbase Developer Portal Faucet](https://portal.cdp.coinbase.com/products/faucet)
2.  Sign in with your Coinbase account (the same account you use for Coinbase Business)
3.  Select **Network**: Base Sepolia
4.  Select **Token**: USDC
5.  Enter your wallet address from Step 1
6.  Click **Claim**

You’ll receive 1 USDC (testnet) - enough to test multiple payment links.

### Step 3: Test a Payment

Now you can test the full payment flow:

1.  Create a sandbox payment link (see [Getting Started](#getting-started) above)
2.  Open the payment link URL in your browser
3.  Connect your testnet wallet when prompted
4.  Approve the USDC payment
5.  Verify your webhook receives the `payment_link.payment.success` event
6.  Confirm the transaction on [Base Sepolia Explorer](https://sepolia.basescan.org/)

## API Endpoints

All Payment Links API endpoints are available in sandbox with the `/sandbox` path prefix:

Endpoint

Sandbox URL

Create Payment Link

`POST /sandbox/api/v1/payment-links`

List Payment Links

`GET /sandbox/api/v1/payment-links`

Get Payment Link

`GET /sandbox/api/v1/payment-links/{id}`

Deactivate Payment Link

`DELETE /sandbox/api/v1/payment-links/{id}`

The request and response schemas are identical to production. No changes to API contracts are needed when switching between environments.

## Webhook Events

Sandbox webhook events work the same as production but are differentiated using the `sandbox` label. This allows you to:

-   Test webhook integration without receiving production events
-   Validate your webhook handler with realistic event payloads
-   Test failure scenarios and retry logic safely

### Subscribing to Sandbox Webhooks

To receive webhook events for sandbox payment links, create a subscription with `"sandbox": "true"` in the labels:

```
cdpcurl -X POST \
  -i "YOUR_API_KEY_ID" \
  -s "YOUR_API_KEY_SECRET" \
  "https://api.cdp.coinbase.com/platform/v2/data/webhooks/subscriptions" \
  -d '{
    "description": "Sandbox payment link webhooks",
    "eventTypes": [
      "payment_link.payment.success",
      "payment_link.payment.failed",
      "payment_link.payment.expired"
    ],
    "target": {
      "url": "https://your-webhook-url.com/sandbox",
      "method": "POST"
    },
    "labels": {
      "sandbox": "true"
    },
    "isEnabled": true
  }'

```

### Webhook Event Types

The same event types are available for both sandbox and production:

Event Type

Description

`payment_link.payment.success`

Payment link successfully paid

`payment_link.payment.failed`

Payment link payment failed

`payment_link.payment.expired`

Payment link expired without payment

For complete webhook setup instructions and signature verification, see the [Webhooks documentation](https://developer.chrome.com/coinbase-business/payment-link-apis/webhooks).

## Best Practices

## Limitations & Considerations

-   **Simulated Transactions**: Sandbox payments do not involve real funds
-   **No Wallet Updates**: Payments will not appear in your Coinbase app wallet
-   **Data Retention**: Sandbox data is automatically purged after 30 days
-   **Performance**: Response times may vary from production
-   **Third-Party Services**: Some third-party integrations may use mocked responses

## Transitioning to Production

When you’re ready to move from sandbox to production:

## Troubleshooting

## Additional Resources