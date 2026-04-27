# welcome

## Overview

An onchain trade (otherwise known as a swap) is a transaction that exchanges one token for another, enabling automated trading, arbitrage opportunities, and sophisticated DeFi front-end applications.

The Trade API provides real-time price discovery and token swaps across multiple DEXes, enabling traders to access deep liquidity pools and execute at the best available prices. For the smoothest developer experience, use CDP Trade API with our [Wallets](https://developer.chrome.com/server-wallets/v2/introduction/welcome) - supporting both regular accounts (EOAs) and Smart Accounts with gas sponsorship. We also support the use of external wallet libraries like [viem](https://viem.sh/) and [web3.py](https://pypi.org/project/web3) for building with your own infrastructure.

## Key benefits

-   **High-performance swaps:** Sub-500ms execution latency for competitive algorithmic trading and arbitrage strategies
-   **Advanced trading features:** Built-in slippage protection and optimized gas management for professional traders
-   **Multi-strategy support:** Execute spot trading, arbitrage, market making, and yield optimization strategies from a single API
-   **Use with CDP ecosystem or bring your own:** Smooth integration via CDP wallet accounts (EOAs), Smart Accounts (ERC-4337), or bring your own wallet infrastructure
-   **Enterprise-grade scalability:** Robust rate limits supporting high-frequency trading and multiple concurrent trading bots
-   **Complete trade auditability:** Comprehensive swap metadata, price impact analysis, and fee breakdowns for compliance and backtesting

## Use cases

-   **Agentic systems (e.g. [x402](https://developer.chrome.com/x402/welcome)) or DeFi bots:** Automatically execute swaps using wallets with policy-enforced, enclave-secured signing - both EOAs and Smart Accounts
-   **Smart Account powered apps:** Enable gasless swaps for users via paymaster sponsorship, batch multiple operations, and leverage account abstraction
-   **Autonomous DeFi vaults:** Programmatic yield rebalancing with built-in risk controls and CDP-managed wallet infrastructure
-   **Regulatory-aligned automation:** Secure, compliant swaps for fintech apps, stablecoin payments, and enterprise treasury flows with KYT/KYC enforcement
-   **AI-driven trading agents:** Real-time, policy-constrained execution with sub-500ms latency—no hot keys, no custom infra

## CDP and external integrations

The Trade API supports two main approaches for executing swaps, both compatible with regular accounts (EOAs) and Smart Accounts:

Approach

Features

Account Types

Best For

[**CDP Wallet**](https://developer.chrome.com/server-wallets/v2/evm-features/swaps)

• Managed infrastructure  
• Automatic signing  
• Built-in security  
• USDC rewards  
• Gas sponsorship

• EOAs  
• Smart Accounts

Most use cases

[**External Library**](https://developer.chrome.com/trade-api/quickstart)

• Full control  
• Custom wallet infrastructure  
• BYO account

• EOAs  
• Smart Accounts (with setup)

Advanced needs

## Why use Trade API?

The Trade API provides unmatched advantages for building trading applications and automated strategies over existing aggregator solutions:

Feature

Trade API

Traditional Solutions

**Infrastructure**

✅ Zero setup required

❌ Complex self-hosting

**Execution Speed**

✅ Sub-500ms latency

❌ 1-3 second delays

**Bot Development**

✅ Native wallet integration

❌ Manual key management

**Multi-strategy Support**

✅ Concurrent execution

❌ Sequential only

**Gas Management**

✅ Automatic optimization

❌ Manual configuration

**Slippage Protection**

✅ MEV-aware routing

❌ Manual implementation

More on swap aggregators

Performing a token swap involves finding a place to execute the trade (also known as a liquidity pool). These liquidity pools are inside decentralized exchanges (DEXes).An **aggregator** powers swap routing through various DEXes (Uniswap, SushiSwap, Curve, Balancer, etc.) to get the best possible price. Instead of interacting with a single DEX, an aggregator will optimize the swap by comparing and splitting trades across DEXes, securing the most favorable exchange rate.

## What to read next

-   **[Quickstart](https://developer.chrome.com/trade-api/quickstart):** Get started with the Trade API and learn how to execute your first swap.
-   **[API Reference](https://developer.chrome.com/api-reference/v2/introduction):** Explore the full CDP API v2 documentation.
-   **[Server Wallet v2](https://developer.chrome.com/server-wallets/v2/introduction/quickstart):** Learn more about our new Server Wallet, including account management and transaction signing.
-   **[USDC Rewards](https://developer.chrome.com/server-wallets/v2/introduction/usdc-rewards):** Learn how to earn 3.35% rewards on USDC balances in your CDP wallets.