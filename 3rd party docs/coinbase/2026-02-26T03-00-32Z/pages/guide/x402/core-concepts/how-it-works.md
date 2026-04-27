# how it works

This page explains the complete payment flow in x402, from initial request to payment settlement.

## Overview

x402 enables programmatic payments over HTTP using a simple request-response flow. When a client requests a paid resource, the server responds with payment requirements, the client submits payment, and the server delivers the resource.

## Payment Flow

![](https://mintcdn.com/coinbase-prod/-GFospv_XOADGRLp/x402/images/x402-protocol-flow.png?fit=max&auto=format&n=-GFospv_XOADGRLp&q=85&s=aaf7050553a7f9cd27a26c2b12030c2d)

### Step-by-Step Process

1.  **Client makes HTTP request** - The [client](https://developer.chrome.com/x402/core-concepts/client-server) sends a standard HTTP request to a resource server for a protected endpoint.
2.  **Server responds with 402** - The resource server returns an [HTTP 402 Payment Required](https://developer.chrome.com/x402/core-concepts/http-402) status code with payment requirements in the `PAYMENT-REQUIRED` header.
3.  **Client creates payment** - The client examines the payment requirements and creates a payment payload using their [wallet](https://developer.chrome.com/x402/core-concepts/wallet) based on the specified scheme.
4.  **Client resubmits with payment** - The client sends the same HTTP request again, this time including the `PAYMENT-SIGNATURE` header containing the signed payment payload.
5.  **Server verifies payment** - The resource server validates the payment payload either:
    -   Locally (if running their own verification)
    -   Via a [facilitator](https://developer.chrome.com/x402/core-concepts/facilitator) service (recommended)
6.  **Facilitator validates** - If using a facilitator, it checks the payment against the scheme and network requirements, returning a verification response.
7.  **Server processes request** - If payment is valid, the server fulfills the original request. If invalid, it returns another 402 response.
8.  **Payment settlement** - The server initiates blockchain settlement either:
    -   Directly by submitting to the blockchain
    -   Through the facilitator’s `/settle` endpoint
9.  **Facilitator submits onchain** - The facilitator broadcasts the transaction to the blockchain based on the payment’s network and waits for confirmation.
10.  **Settlement confirmation** - Once confirmed onchain, the facilitator returns a payment execution response.
11.  **Server delivers resource** - The server returns a 200 OK response with:
     -   The requested resource in the response body
     -   A `PAYMENT-RESPONSE` header containing the settlement details

## Key Components

-   **[Client & Server](https://developer.chrome.com/x402/core-concepts/client-server)** - The roles and responsibilities of each party
-   **[Facilitator](https://developer.chrome.com/x402/core-concepts/facilitator)** - Optional service that handles payment verification and settlement
-   **[HTTP 402](https://developer.chrome.com/x402/core-concepts/http-402)** - How payment requirements are communicated
-   **[Networks & Facilitators](https://developer.chrome.com/x402/network-support)** - Available networks and facilitator options

## Why This Design?

The x402 protocol is designed to be:

-   **Stateless** - No sessions or authentication required
-   **HTTP-native** - Works with existing web infrastructure
-   **Blockchain-agnostic** - Supports multiple networks through facilitators
-   **Developer-friendly** - Simple integration with standard HTTP libraries

## Next Steps

-   Ready to accept payments? See [Quickstart for Sellers](https://developer.chrome.com/x402/quickstart-for-sellers)
-   Want to make payments? See [Quickstart for Buyers](https://developer.chrome.com/x402/quickstart-for-buyers)
-   Looking for specific networks? Check [Network Support](https://developer.chrome.com/x402/network-support)