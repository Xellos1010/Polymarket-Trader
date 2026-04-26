# overview

## Overview

To read blockchain data (like balances or transactions) or write to it (like deploying contracts), you need a connection to a blockchain node. **Node** provides blockchain access for developers building on [Base](https://base.org/), giving you instant RPC access without running your own infrastructure. Think of it as a direct line to the Base blockchain. You make requests, and Node handles all the complexity of connecting to and querying the network.

[

## Try it now: Quickstart

Get your RPC endpoint and make your first blockchain request in minutes

](https://developer.chrome.com/data/node/quickstart)

## Key features

-   **Production-ready RPC access:** Instant connection to Base Mainnet and Sepolia testnet with reliable infrastructure and fast response times
-   **Generous free tier:** Get started with 10 million billing units free every month (enough for most development and testing needs)
-   **Zero infrastructure management:** Skip the complexity of running your own blockchain node and just use our endpoints
-   **Standard Ethereum compatibility:** Works seamlessly with any Ethereum-compatible tools and libraries like `ethers.js` and `viem`
-   **Beyond basic RPC:** Includes Paymaster for gas sponsorship and wallet history methods (ready to use immediately)

## Use cases

-   **Build DeFi apps:** Create trading interfaces, lending platforms, or dashboards that display live blockchain data
-   **Create NFT platforms:** Build marketplaces that show who owns which NFTs and track their transfer history
-   **Develop wallet apps:** Display user balances, show transaction history, and send transactions on behalf of users
-   **Deploy smart contracts:** Upload your contracts to Base and interact with them programmatically

## Supported networks

Currently available on:

-   **Base Mainnet** - Production environment for live applications
-   **Base Sepolia** - Testnet for development and testing

Base is a secure, low-cost, builder-friendly Ethereum L2 built to bring the next billion users onchain. [Learn more about Base](https://docs.base.org/).

## Getting started

Ready to connect to Base? Our quickstart guide walks you through everything step-by-step—from trying Node in the browser playground to making your first programmatic request with code examples in multiple languages.

[

## Start building: Quickstart

Get your RPC endpoint and make your first blockchain request in minutes

](https://developer.chrome.com/data/node/quickstart)

## Pricing & Rate Limits

CDP Node operates on a pay-as-you-go model with a generous free tier:

-   **10 million Billing Units (BU) free every month**
-   **After the free tier:** $0.50 per million additional BUs
-   **Rate limit:** 7,500 BU every 5 seconds per project (approximately 50 requests per second)

Each JSON-RPC method consumes a specific number of BUs based on computational complexity. The average call uses 30 BU. **Need higher limits?** Join our [Node Discord channel](https://discord.com/channels/1220414409550336183/1222183017284501535) to request a rate limit increase. For detailed pricing by method, see our [Pricing page](https://developer.chrome.com/data/node/pricing).

## API Reference

When you’re ready to build, explore the available methods:

-   **[Core EVM Methods](https://developer.chrome.com/api-reference/json-rpc-api/core)** - Standard Ethereum JSON-RPC methods ([full spec](https://ethereum.org/en/developers/docs/apis/json-rpc/#json-rpc-methods))
-   **[Paymaster Methods](https://developer.chrome.com/api-reference/json-rpc-api/paymaster)** - Sponsor gas fees for your users
-   **[Wallet History Methods](https://developer.chrome.com/api-reference/json-rpc-api/wallet-history)** - Query historical wallet data

## Support and feedback

-   **[CDP Discord](https://discord.com/channels/1220414409550336183/1222183017284501535)**: Join #node for support and to request rate limit increases