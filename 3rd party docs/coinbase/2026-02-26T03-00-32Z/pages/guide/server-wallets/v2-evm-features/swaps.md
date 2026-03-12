# swaps

## Overview

The Server Wallet provides a convenient way to execute swaps using CDP EVM accounts. This feature combines the core Trade APIs (which handle price discovery and quote creation) with the Server Wallet’s ability to sign and broadcast transactions.

## Prerequisites

It is assumed you have already completed the Server Wallet [Quickstart](https://developer.chrome.com/server-wallets/v2/introduction/quickstart) guide and have a CDP account ready to use.

## Using CDP Accounts

When using CDP EVM [Accounts](https://developer.chrome.com/server-wallets/v2/introduction/accounts), we offer two options for submitting swaps:

1.  **All-in-one swap** (recommended): Submit a swap in one call
2.  **Quote, then swap** (advanced): Create a quote first, then execute the swap. This is useful for detailed swap inspection or custom logic.

## Code Examples

For complete code examples showing how to execute swaps with both regular accounts (EOAs) and Smart Accounts using CDP Wallets, see the [Trade API Quickstart guide](https://developer.chrome.com/trade-api/quickstart):

-   **[Regular Accounts (EOAs)](https://developer.chrome.com/trade-api/quickstart#regular-accounts-eoas)** - Price estimation, creating quotes, and executing swaps with standard accounts
-   **[Smart Accounts](https://developer.chrome.com/trade-api/quickstart#smart-accounts)** - Leveraging ERC-4337 features like gas sponsorship and batch operations

The quickstart guide includes examples in both TypeScript and Python, along with links to full implementations on GitHub.

## What to read next

-   [**Trade API Documentation**](https://developer.chrome.com/trade-api/welcome): Learn more about the core Trade APIs for price discovery and quote creation
-   [**Quickstart Guide**](https://developer.chrome.com/trade-api/quickstart): Step-by-step examples for implementing swaps
-   [**Smart Accounts**](https://developer.chrome.com/server-wallets/v2/evm-features/smart-accounts): Learn about using ERC-4337 Smart Accounts for advanced features