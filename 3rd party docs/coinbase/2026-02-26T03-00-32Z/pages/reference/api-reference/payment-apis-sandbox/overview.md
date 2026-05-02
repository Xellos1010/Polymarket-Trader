# overview

## Key concepts in Sandbox

Resource

Direction

Description

**[Account](https://developer.chrome.com/api-reference/payment-apis/rest-api/accounts/accounts)**

\-

Your asset balance within Coinbase that you fund with test amounts (e.g., your USD balance with $1000 test funds)

**[Deposit destination](https://developer.chrome.com/api-reference/payment-apis/rest-api/deposit-destinations-under-development/overview)**

Incoming crypto

Placeholder addresses for receiving crypto. Simulate deposits via Portal UI

**[Payment method](https://developer.chrome.com/api-reference/payment-apis/rest-api/payment-methods/payment-methods)**

Outgoing fiat

External bank accounts for fiat withdrawals. Three pre-configured test banks shared across all accounts (Fedwire JPMorgan, Fedwire Bank of America, SWIFT Deutsche Bank)

**[Transfer](https://developer.chrome.com/api-reference/payment-apis/rest-api/transfers/transfers)**

Both

Move funds to crypto addresses, emails, or payment methods. All simulated

## Sandbox vs. Production

Sandbox and production offer the same endpoints and functionality, but with different data and behavior.

### Operational differences

Sandbox

Production

**Base URL**

`sandbox.cdp.coinbase.com`

`api.cdp.coinbase.com`

**API keys**

Sandbox-specific credentials

Production credentials

**Rate limits**

Same as production

Standard production limits

**Data persistence**

Permanent

Permanent

**Performance**

Response times may vary

Standard production performance

**Third-party services**

Mocked responses

Real integrations

**Compliance checks**

Simplified (no real KYC/AML)

Full compliance flows

### Resource differences

Sandbox

Production

**[Accounts](https://developer.chrome.com/api-reference/payment-apis/rest-api/accounts/accounts)**

Create via Sandbox UI (funding only via UI)

Link existing Prime portfolio or Coinbase Business account

**[Deposit destinations](https://developer.chrome.com/api-reference/payment-apis/rest-api/deposit-destinations-under-development/overview)**

Placeholder addresses; simulate deposits via Sandbox UI

Real blockchain addresses

**[Payment methods](https://developer.chrome.com/api-reference/payment-apis/rest-api/payment-methods/payment-methods)**

Three test methods: Fedwire (active), Fedwire (inactive), SWIFT (active)

Automatically linked from Prime/Business

**[Transfers](https://developer.chrome.com/api-reference/payment-apis/rest-api/transfers/transfers)**

Simulated (webhooks fire, no blockchain activity)

Real blockchain transactions

**[Webhooks](https://developer.chrome.com/api-reference/payment-apis/webhooks)**

Supported

Supported

## Best practices

## What to read next