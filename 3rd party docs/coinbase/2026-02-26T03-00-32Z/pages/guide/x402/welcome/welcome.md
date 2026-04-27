# welcome

## Overview

x402 is a new open payment protocol developed by Coinbase that enables instant, automatic stablecoin payments directly over HTTP. By reviving the [HTTP 402 Payment Required](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/402) status code, x402 lets services monetize APIs and digital content onchain, allowing clients, both human and machine, to programmatically pay for access without accounts, sessions, or complex authentication.

## Who is x402 for?

-   **Sellers:** Service providers who want to monetize their APIs or content. x402 enables direct, programmatic payments from clients with minimal setup.
-   **Buyers:** Human developers and AI agents seeking to access paid services without accounts or manual payment flows.

Both sellers and buyers interact directly through HTTP requests, with payment handled transparently through the protocol.

## Use cases

x402 enables a range of use cases, including:

-   API services paid per request
-   AI agents that autonomously pay for API access
-   Paywalls for digital content
-   Microservices and tooling monetized via microtransactions
-   Proxy services that aggregate and resell API capabilities

## How it works

x402 uses a simple request-response flow with programmatic payments. For a detailed explanation, see [How x402 Works](https://developer.chrome.com/x402/core-concepts/how-it-works). At a high level:

1.  The buyer requests a resource from the server (e.g. an API call, see [client/server roles](https://developer.chrome.com/x402/core-concepts/client-server))
2.  If payment is required, the server responds with a [402 Payment Required](https://developer.chrome.com/x402/core-concepts/http-402), including payment instructions in the `PAYMENT-REQUIRED` header
3.  The buyer constructs and sends a payment payload via the `PAYMENT-SIGNATURE` header
4.  The server verifies and settles the payment via the [facilitator](https://developer.chrome.com/x402/core-concepts/facilitator). If valid, the server returns the requested resource

## Key Features

x402 provides a modern payment layer designed for developers and AI agents:

-   **CAIP-2 Network Identifiers**: Industry-standard chain identification (e.g., `eip155:8453` for Base)
-   **Multi-Network Support**: Single client handles EVM and Solana networks
-   **Extensions System**: Enable service discovery via [Bazaar](https://developer.chrome.com/x402/bazaar), authentication, and more
-   **TypeScript & Go SDKs**: Production-ready client and server libraries

## Beyond legacy limitations

x402 is designed for a modern internet economy, solving key limitations of legacy systems:

-   **Reduce fees and friction:** Direct onchain payments without intermediaries, high fees, or manual setup.
-   **Micropayments & usage-based billing:** Charge per call or feature with simple, programmable pay-as-you-go flows.
-   **Machine-to-machine transactions:** Let AI agents pay and access services autonomously with no keys or human input needed.

## Offload your infra

The x402 [Facilitator](https://developer.chrome.com/x402/core-concepts/facilitator) handles payment verification and settlement so that sellers don’t need to maintain their own blockchain infrastructure. The Coinbase Developer Platform (CDP) offers a Coinbase-hosted facilitator service that processes stablecoin payments on Base and Solana with a generous free tier of 1,000 transactions per month, offering a streamlined and predictable experience for both buyers and sellers. Facilitators handle verification and settlement, so sellers do not need to maintain blockchain infrastructure. CDP’s x402 facilitator provides:

-   Stablecoin payments on Base and Solana with a generous free tier (1,000 transactions/month, then $0.001/transaction)
-   Fast, onchain settlement of transactions
-   Simplified setup for sellers to start accepting payments

## Facilitator roadmap

CDP’s x402 facilitator is designed as a facilitator on top of an open standard, which is not tied to any single provider. Over time, the facilitator will include:

-   A discovery layer for buyers (human and agents) to find available services ([Bazaar](https://developer.chrome.com/x402/bazaar))
-   Support for additional payment flows (e.g., pay for work done, credit based billing, etc.)
-   Optional attestations for sellers to enforce KYC or geographic restrictions
-   Support for additional assets and networks

The goal is to make programmatic commerce accessible, permissionless, and developer-friendly.

## What to read next

-   [Quickstart for Sellers](https://developer.chrome.com/x402/quickstart-for-sellers): Get started with x402 by accepting payments from clients.
-   [Quickstart for Buyers](https://developer.chrome.com/x402/quickstart-for-buyers): Get started with x402 by paying for services.
-   [X402 with Embedded Wallets](https://developer.chrome.com/embedded-wallets/x402-payments): Enable your users to make x402 payments with embedded wallets.
-   [Network Support](https://developer.chrome.com/x402/network-support): See available facilitators and supported networks.
-   [How x402 Works](https://developer.chrome.com/x402/core-concepts/how-it-works): Understand the complete payment flow.
-   [Join our community on Discord](https://discord.gg/cdp): Get help and stay up to date with the latest developments.