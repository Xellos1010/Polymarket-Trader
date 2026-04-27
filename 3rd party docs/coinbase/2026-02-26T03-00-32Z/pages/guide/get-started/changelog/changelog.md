# changelog

![EW Blog Header img](https://images.ctfassets.net/q5ulk4bp65r7/1pGqVEKUGwCgKRxHxRCvMg/59711d4e4651e67ee634be22b77da8bd/embedded_wallets_blog.png?w=768&fm=png)

## Embedded Wallets: Beta Launch

CDP’s new [Embedded Wallets](https://www.coinbase.com/developer-platform/products/embeddedwallets?utm_source=docs&utm_campaign=changelog) product is now in beta. It gives builders (and users) simple wallet management without passkeys or seed phrases, and has onramps, trading, and 4.1% USDC rewards built-in using CDP’s unified set of APIs. [Try it out](https://portal.cdp.coinbase.com/products/embedded-wallets).

## CDP Server Wallets: Network Expansion

-   **Server Wallets now supports [Solana sends](https://developer.chrome.com/wallet-api/v2/solana-features/sending-transactions)**: sign & broadcast in one call with sub-200ms latency, 250 TPS throughput, and batching.
-   **[Smart Accounts](https://developer.chrome.com/wallet-api/v2/evm-features/smart-accounts)** now support a broader range of EVM networks including Ethereum mainnet, Base, Arbitrum, Optimism, Zora, Polygon, BNB, and Avalanche
-   **[EVM send API](https://developer.chrome.com/wallet-api/v2/evm-features/sending-transactions)** supports Arbitrum, Polygon, Optimism, Avalanche in addition to Base and Ethereum Mainnet now.

## CDP Security Suite: Policy Engine Demos

Since launching [Server Wallets](https://www.coinbase.com/developer-platform/products/wallets?utm_source=docs&utm_campaign=changelog) a few weeks ago, we’ve gotten a ton of interest and questions about one of its core features: Policy Engine. The team put together some quick demos to show you how to easily [set limits on USDC transfers](https://youtube.com/shorts/s2Pn3r8YqLc), block scams with [approve/deny wallet lists](https://youtube.com/shorts/EJPUZsVQlF0), and even create [multi-rule setups](https://youtube.com/shorts/19KFQOFTOvE) and advanced transaction flows.

## Onramp API: Apple Pay Integration Guide

Apple Pay Onramp [integration guide](https://developer.chrome.com/onramp/headless-onramp/overview) is now available in the docs. Apple Pay is one of the most frictionless onboarding experiences for buying crypto and even helped Moonshot [increase conversion by 25%](https://www.coinbase.com/developer-platform/discover/case-studies/moonshot?utm_source=docs&utm_campaign=changelog).

## Onramp/Offramp: Session Token Upgrade

Effective 7/31/2025, all Coinbase Onramp & Offramp URLs must be securely initialized using the `sessionToken` parameter. This migration is mandatory for continued access to Coinbase Onramp and Offramp APIs.