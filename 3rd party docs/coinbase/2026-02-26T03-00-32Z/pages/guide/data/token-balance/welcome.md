# welcome

## Overview

The Token Balances API enables you to retrieve public token balances of addresses on Base. This includes tokens (i.e. [ERC-20s](https://ethereum.org/en/developers/docs/standards/tokens/erc-20/)) and the native gas tokens. You can retrieve public token balances using the [REST API](https://developer.chrome.com/api-reference/v2/rest-api/evm-token-balances/list-evm-token-balances) or [CDP SDK](https://developer.chrome.com/data/token-balance/cdp-sdk).

## Core capabilities

-   **Lightning-fast performance**: Sub-1s query latency (P99) with real-time data updates within 1 second from tip of chain
-   **Standardized responses**: Unified data format with 99.9%+ uptime
-   **Universal address queries**: Access public balance data for addresses without ownership requirements
-   **Comprehensive token coverage**: Returns both native ETH and ERC-20 token balances

## Base support and performance

The Token Balances API supports both Base Mainnet and Base Sepolia testnet with sub-1 second P99 latency. Base Mainnet tracks all tokens, while Base Sepolia tracks the top 100 most active token addresses for that network. The API uses a two-step process: first checking a database to find which tokens an address owns, then reading the exact balance from the token’s smart contract to ensure fast responses with accurate amounts.

## Use cases

-   **Portfolio Tracker dApp**: Build a real-time portfolio tracker that monitors Base Mainnet whale wallets. Track how top addresses like [exchange wallets](https://basescan.org/address/0x835678a611b28684005a5e2233695fb6cbbb0007) allocate their holdings across different tokens.
-   **DeFi Analytics Dashboard**: Create analytics tools that analyze token distribution patterns. Monitor how DeFi protocols’ treasury addresses evolve over time or track liquidity provider positions across multiple DEXs.
-   **Trading Infrastructure**: Develop arbitrage bots that monitor specific addresses for balance changes. Enable pre-trade balance verification, market maker inventory tracking, and quick reaction to large holder movements.
-   **Compliance & Risk Management**: Build KYC/AML solutions that monitor address activity. Track token flows between addresses, generate audit trails, and flag suspicious patterns in real-time.

## What to read next

-   **[REST API Reference](https://developer.chrome.com/api-reference/v2/rest-api/evm-token-balances/list-evm-token-balances)**: Complete API documentation
-   **[CDP SDK Guide](https://developer.chrome.com/data/token-balance/cdp-sdk)**: Get started with the SDK