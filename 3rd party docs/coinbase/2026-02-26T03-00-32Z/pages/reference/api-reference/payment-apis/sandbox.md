# sandbox

## Overview

The CDP Payments API sandbox environment provides a safe, isolated testing space where you can develop and test your payment integrations without affecting production data or processing real transactions. The sandbox mirrors production functionality while using test data and simulated payment flows.

## Key Differences: Sandbox vs Production

Feature

Sandbox

Production

**API Endpoint**

`sandbox.cdp.coinbase.com`

`api.cdp.coinbase.com`

**API Keys**

Sandbox-specific credentials

Production credentials

**Transactions**

Simulated (no real value)

Real payment processing

**Test Accounts**

Unlimited test accounts

Real user accounts

**Rate Limits**

Same as production

Standard production limits

**Data Persistence**

Permanent

Permanent

**Webhooks**

Supported

Supported

## Getting Started

### 1\. Create Sandbox API Credentials

To access the sandbox environment, you’ll need to create sandbox-specific API credentials:

### 2\. Testing Workflows

Use following Postman Collection and Environment with the key created in previous step to test CDP Sandbox.

-   [CDP Payments - Postman Collection](https://developer.chrome.com/api-reference/payment-apis/CDP%20Payments%20Sandbox.postman_collection-docs.json)
-   [CDP Payments - Postman Environment](https://developer.chrome.com/api-reference/payment-apis/CDP%20Payments%20Sandbox.postman_environment-docs.json)

## Best Practices

## Test data for transfers

When testing email-based transfers in the sandbox environment, only specific whitelisted email addresses will return successful validation responses. This approach prevents privacy concerns while providing predictable test behavior.

### Whitelisted email addresses

The following email addresses are whitelisted for sandbox testing and will return a `2xx` success response when used as transfer targets:

Test Email

Description

`testuser1@domain.com`

Returns successful validation

`testuser2@domain.com`

Returns successful validation

A request body that tests email validation might look like the following:

```
{
  "source": {
    "accountId": "{{accountId}}",
    "asset": "USD"
  },
  "target": {
    "email": "testuser1@domain.com",
    "asset": "USD"
  },
  "amount": "10",
  "validateOnly": true
}

```

### Non-whitelisted emails

Any email address **not** in the whitelist will return a `4xx` validation error indicating the user was not found.

### Reserved onchain addresses for simulated outcomes

When testing onchain transfers in the sandbox environment, you can use reserved onchain addresses to simulate deterministic success or failure outcomes. Each address returns a predictable response based on the address used.

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

#### Sample request and response payloads

-   Success
    
-   Transfer invalid target
    
-   Invalid address
    
-   Unsupported network
    

Request payload:

```
{
  "source": {
    "accountId": "{{accountId}}",
    "asset": "USDC"
  },
  "target": {
    "network": "base",
    "address": "0x1111111111111111111111111111111111111111",
    "asset": "USDC"
  },
  "amount": "10.00"
}

```

Expected response: HTTP `2xx` with a normal transfer response.

Request payload:

```
{
  "source": {
    "accountId": "{{accountId}}",
    "asset": "USDC"
  },
  "target": {
    "network": "base",
    "address": "0x2222222222222222222222222222222222222222",
    "asset": "USDC"
  },
  "amount": "10.00"
}

```

Expected response: HTTP `400`:

```
{
  "errorType": "invalid_request",
  "errorMessage": "'target' is invalid: must match one of [Account, Payment Method, Onchain Address, Email Instrument]. Account requires 'accountId'; Payment Method requires 'paymentMethodId'; Onchain Address requires 'network'; Email Instrument requires 'email'"
}

```

Request payload:

```
{
  "source": {
    "accountId": "{{accountId}}",
    "asset": "USDC"
  },
  "target": {
    "network": "base",
    "address": "0x3333333333333333333333333333333333333333",
    "asset": "USDC"
  },
  "amount": "10.00"
}

```

Expected response: HTTP `400`:

```
{
  "errorType": "invalid_request",
  "errorMessage": "Invalid onchain address for network base."
}

```

Request payload:

```
{
  "source": {
    "accountId": "{{accountId}}",
    "asset": "USDC"
  },
  "target": {
    "network": "base",
    "address": "0x4444444444444444444444444444444444444444",
    "asset": "USDC"
  },
  "amount": "10.00"
}

```

Expected response: HTTP `400`:

```
{
  "errorType": "invalid_request",
  "errorMessage": "base is not a supported network."
}

```

## Limitations & Considerations

-   **Performance**: Response times may vary from production
-   **Third-Party Services**: Some third-party integrations use mocked responses
-   **Rate Limits**: Same rate limits as production apply to prevent abuse
-   **Compliance Checks**: Simplified compliance flows (no real KYC/AML)

## Transitioning to Production

When you’re ready to move from sandbox to production:

## Need Help?

If you encounter issues with the sandbox environment, see the [Troubleshooting](https://developer.chrome.com/api-reference/payment-apis/troubleshooting) page for common issues and solutions.