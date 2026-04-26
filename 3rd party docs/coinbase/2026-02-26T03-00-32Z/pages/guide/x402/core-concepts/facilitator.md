# facilitator

This page explains the role of the **facilitator** in the x402 protocol. The facilitator is an optional but recommended service that simplifies the process of verifying and settling payments between clients (buyers) and servers (sellers).

## What is a Facilitator?

The facilitator is a service that:

-   Verifies payment payloads submitted by clients.
-   Settles payments on the blockchain on behalf of servers.

By using a facilitator, servers do not need to maintain direct blockchain connectivity or implement payment verification logic themselves. This reduces operational complexity and ensures accurate, real-time validation of transactions.

## Facilitator Responsibilities

-   **Verify payments:** Confirm that the client’s payment payload meets the server’s declared payment requirements.
-   **Settle payments:** Submit validated payments to the blockchain and monitor for confirmation.
-   **Provide responses:** Return verification and settlement results to the server, allowing the server to decide whether to fulfill the client’s request.

The facilitator does not hold funds or act as a custodian - it performs verification and execution of onchain transactions based on signed payloads provided by clients.

## Why Use a Facilitator?

Using a facilitator provides:

-   **Gas sponsorship:** The facilitator pays gas fees for settling payments, so neither buyers nor sellers need to hold native tokens or worry about gas costs.
-   **Reduced operational complexity:** Servers do not need to interact directly with blockchain nodes.
-   **Protocol consistency:** Standardized verification and settlement flows across services.
-   **Faster integration:** Services can start accepting payments with minimal blockchain-specific development.

While it is possible to implement verification and settlement locally, using a facilitator accelerates adoption and ensures correct protocol behavior.

## CDP’s Facilitator

Coinbase Developer Platform (CDP) operates a hosted facilitator service. CDP’s x402 facilitator offers:

-   **Stablecoin payments on Base and Solana:** Supports any EIP-3009 compliant token on Base (commonly USDC) and SPL tokens on Solana.
-   **Built-in compliance screening:** Includes Know-Your-Transaction (KYT) checks that decline payments involving sanctioned or high-risk addresses.
-   **Generous free tier:** 1,000 transactions free per month, then $0.001 per transaction.
-   **High performance settlement:** Payments are submitted with fast confirmation times and high throughput.

To get started with CDP’s facilitator, see the [quickstart for sellers](https://developer.chrome.com/x402/quickstart-for-sellers). Using CDP’s facilitator allows sellers to quickly integrate payments without managing blockchain infrastructure, while providing a predictable and low-cost experience for buyers.

### Pricing

CDP’s x402 facilitator operates on a pay-as-you-go pricing model with a generous free tier:

Tier

Monthly Transactions

Cost per Transaction

Free

Up to 1,000

$0.00

Usage-based

Above 1,000

$0.001

## Available Facilitators

For a list of available facilitators and the networks they support, see [Network Support](https://developer.chrome.com/x402/network-support).

## How It Works

To understand how facilitators fit into the complete x402 payment flow, see [How x402 Works](https://developer.chrome.com/x402/core-concepts/how-it-works).

## Summary

The facilitator acts as an independent verification and settlement layer within the x402 protocol. It helps servers confirm payments and submit transactions onchain without requiring direct blockchain infrastructure. Coinbase’s hosted facilitator simplifies this further by offering a ready-to-use environment with a generous free tier for payments on Base and Solana. Next, explore:

-   [How x402 Works](https://developer.chrome.com/x402/core-concepts/how-it-works): See the complete payment flow
-   [Network Support](https://developer.chrome.com/x402/network-support): Find available facilitators and networks
-   [HTTP 402](https://developer.chrome.com/x402/core-concepts/http-402): Understand how payment requirements are communicated