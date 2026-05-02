# token balances

## Overview

With CDP API and SDKs, you can retrieve token balances for any address on supported EVM networks and Solana. This allows you to check balances for both native tokens (like ETH, SOL) and ERC-20/SPL tokens across multiple blockchain networks. The token balance APIs support querying balances for:

-   **EVM Networks**: Ethereum, Base, and Base Sepolia
-   **Solana Networks**: Solana Mainnet and Solana Devnet

Use the `listTokenBalances` method to retrieve token balances for any EVM address. This method returns both native tokens (like ETH) and ERC-20 tokens held by the address.

### Basic Usage

### Supported Networks

The EVM token balance API supports the following networks:

-   **Mainnet**: `ethereum`, `base`
-   **Testnet**: `base-sepolia`

For addresses with many token balances, you can use pagination to retrieve results in manageable chunks:

## Solana Token Balances

Use the `listTokenBalances` method to retrieve token balances for any Solana address. This method returns both native SOL and SPL tokens held by the address.

### Basic Usage

### Supported Networks

The Solana token balance API supports the following networks:

-   **Mainnet**: `solana`
-   **Devnet**: `solana-devnet`

For addresses with many token balances, you can use pagination:

## Working with Your Own Accounts

If you want to check balances for accounts you’ve created with the CDP SDK, you can easily get the address from your account object:

## Response Format

The token balance response includes detailed information about each token:

### EVM Token Balance Response

```
{
  "balances": [
    {
      "token": {
        "contractAddress": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
        "network": "ethereum",
        "symbol": "USDC",
        "name": "USDC",
      },
      "amount": {
        "amount": 1000000,
        "decimals": 6
      }
    }
  ],
  "nextPageToken": "..."
}

```

### Solana Token Balance Response

```
{
  "balances": [
    {
      "token": {
        "mintAddress": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        "symbol": "USDC",
        "name": "USDC",
      },
      "amount": {
        "amount": 1000000,
        "decimals": 6
      }
    }
  ],
  "nextPageToken": "..."
}

```

## What to read next

-   [Managing Accounts](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/managing-accounts): Learn tips and best practices for managing your accounts
-   [API Reference](https://developer.chrome.com/api-reference/v2/introduction): Full API reference for the v2 Server Wallet