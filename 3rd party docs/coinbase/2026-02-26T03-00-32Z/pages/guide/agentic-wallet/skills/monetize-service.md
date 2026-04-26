# monetize service

## Overview

Build and deploy a paid API that other agents can use via x402. Use when you or the user want to monetize an API, offer a service, sell a service to other agents, or charge for endpoints.

## How it works

x402 is an HTTP-native payment protocol. When a client hits a protected endpoint without paying, the server returns HTTP 402 with payment requirements. The client signs a USDC payment and retries with a payment header. The facilitator verifies and settles the payment, and the server returns the response.

## Prerequisites

-   Must be authenticated (`npx awal@latest status` to check)

## 1\. Get the payment address

Get the wallet address that will receive payments:

Use this address as the `payTo` value in your server configuration.

## 2\. Set up the project

```
mkdir x402-server && cd x402-server
npm init -y
npm install express x402-express

```

Create `index.js`:

```
const express = require("express");
const { paymentMiddleware } = require("x402-express");
const app = express();
app.use(express.json());
const PAY_TO = "<address from step 1>";
// x402 payment middleware
const payment = paymentMiddleware(PAY_TO, {
  "GET /api/example": {
    price: "$0.01",
    network: "base",
    config: {
      description: "Description of what this endpoint returns",
    },
  },
});
// Protected endpoint
app.get("/api/example", payment, (req, res) => {
  res.json({ data: "This costs $0.01 per request" });
});
app.listen(3000, () => console.log("Server running on port 3000"));

```

## 3\. Run and test

Test with curl (should get a 402 response with payment requirements):

```
curl -i http://localhost:3000/api/example

```

Test with a paid request:

```
npx awal@latest x402 pay http://localhost:3000/api/example

```

## API reference

### paymentMiddleware(payTo, routes, facilitator?)

Parameter

Type

Description

`payTo`

string

Ethereum address (0x…) to receive USDC payments

`routes`

object

Route config mapping route patterns to payment config

`facilitator`

object?

Optional custom facilitator (defaults to x402.org)

### Route config

Each key is `"METHOD /path"`. The value is either a price string or a config object:

```
// Simple — just a price
{ "GET /api/data": "$0.05" }
// Full config
{
  "POST /api/query": {
    price: "$0.25",
    network: "base",
    config: {
      description: "Human-readable description",
      inputSchema: {
        bodyType: "json",
        bodyFields: {
          query: { type: "string", description: "The query to run" },
        },
      },
      outputSchema: {
        type: "object",
        properties: {
          result: { type: "string" },
        },
      },
    },
  },
}

```

### Route config fields

Field

Type

Description

`price`

string

USDC price (e.g. “0.01","0.01", "1.00”)

`network`

string

Blockchain network: “base” or “base-sepolia”

`config.description`

string?

What this endpoint does (shown to clients)

`config.inputSchema`

object?

Expected request body/query schema

`config.outputSchema`

object?

Response body schema

`config.maxTimeoutSeconds`

number?

Max time for payment settlement

## Patterns

### Multiple endpoints with different prices

```
const payment = paymentMiddleware(PAY_TO, {
  "GET /api/cheap": { price: "$0.001", network: "base" },
  "GET /api/expensive": { price: "$1.00", network: "base" },
  "POST /api/query": { price: "$0.25", network: "base" },
});
app.get("/api/cheap", payment, (req, res) => { /* ... */ });
app.get("/api/expensive", payment, (req, res) => { /* ... */ });
app.post("/api/query", payment, (req, res) => { /* ... */ });

```

### Health check (no payment)

Register free endpoints before the payment middleware:

```
app.get("/health", (req, res) => res.json({ status: "ok" }));
// Payment middleware only applies to routes registered after it
app.get("/api/data", payment, (req, res) => { /* ... */ });

```

### POST with body schema

```
const payment = paymentMiddleware(PAY_TO, {
  "POST /api/analyze": {
    price: "$0.10",
    network: "base",
    config: {
      description: "Analyze text sentiment",
      inputSchema: {
        bodyType: "json",
        bodyFields: {
          text: { type: "string", description: "Text to analyze" },
        },
      },
      outputSchema: {
        type: "object",
        properties: {
          sentiment: { type: "string" },
          score: { type: "number" },
        },
      },
    },
  },
});
app.post("/api/analyze", payment, (req, res) => {
  const { text } = req.body;
  res.json({ sentiment: "positive", score: 0.95 });
});

```

### Using the CDP facilitator

For production use with the Coinbase facilitator:

```
npm install @coinbase/x402

```

```
const { facilitator } = require("@coinbase/x402");
const payment = paymentMiddleware(PAY_TO, routes, facilitator);

```

This requires `CDP_API_KEY_ID` and `CDP_API_KEY_SECRET` environment variables from the [CDP Portal](https://portal.cdp.coinbase.com/).

## Pricing guidelines

Use Case

Suggested Price

Simple data lookup

0.001−0.001 - 0.01

API proxy / enrichment

0.01−0.01 - 0.10

Compute-heavy query

0.10−0.10 - 0.50

AI inference

0.05−0.05 - 1.00

## Checklist

-   Get wallet address with `npx awal@latest address`
-   Install `express` and `x402-express`
-   Define routes with prices and descriptions
-   Register payment middleware before protected routes
-   Keep health/status endpoints before payment middleware
-   Test with `curl` (should get 402) and `npx awal@latest x402 pay` (should get 200)
-   Announce your service so other agents can find and use it