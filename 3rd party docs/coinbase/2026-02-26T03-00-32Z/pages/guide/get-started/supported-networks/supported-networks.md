# supported networks

This page provides an overview of the blockchain networks we currently support.

What is a network?

A **network** is the blockchain infrastructure supporting transactions, dapps, and smart contracts, with its own rules, consensus mechanism, and native tokens. For example, wallets, addresses, and assets are created on a specific network.

## EVM Networks

## Non-EVM Networks

## Use cases

Mainnets vs. Testnets

-   **Mainnet:** Live, fully operational blockchain with real transactions and actual assets
-   **Testnets:** Sandbox environments for testing smart contracts, dapps, and other blockchain functionality without risking real funds. You can obtain test currencies from [faucets](https://developer.chrome.com/faucets/introduction/welcome)

For example:

**Testnet**

**Mainnet**

[Base-Sepolia](https://docs.base.org/chain/network-information#base-testnet-sepolia)

Base-Mainnet

[Ethereum-Sepolia](https://ethereum.org/en/developers/docs/networks/#sepolia), [Ethereum-Hoodi](https://github.com/eth-clients/hoodi)

Ethereum-Mainnet

[Solana-Devnet](https://solana.com/docs/references/clusters#devnet)

Solana-Mainnet

### Mainnets

Use mainnets for production-grade transactions and interactions using CDP APIs:

-   Sending real crypto funds (e.g., withdrawals, payments)
-   Interacting with live DeFi protocols (such as staking)
-   Reading onchain data for dashboards, analytics, etc.

CDP APIs can help simplify production workflows by letting you query balances, transactions, or fetch events without running a node.

### Testnets

Use testnets for development, testing, or QA with no real funds involved:

-   Testing smart contract deployments before mainnet launch
-   Simulating workflows (like login, staking, etc.) with testnet funds
-   Continuous Integration tests that run contract calls
-   Demo purposes for sandbox environments

### Layer 2s

Use Layer 2s, such as the Base network, for lower fees and faster transactions:

-   Powering gas-efficient wallets or dapps
-   Game or social app activity (frequent, low-cost interactions)
-   Bridging assets between networks

## What to read next

-   [Authentication](https://developer.chrome.com/get-started/authentication/cdp-api-keys): Set up API keys for CDP
-   [Quickstart](https://developer.chrome.com/get-started/quickstart): Build your first onchain app
-   [Demo Apps](https://developer.chrome.com/get-started/demo-apps/explore): Explore sample applications