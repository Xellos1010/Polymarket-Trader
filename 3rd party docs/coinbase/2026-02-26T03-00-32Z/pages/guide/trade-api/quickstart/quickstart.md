# quickstart

## Overview

An onchain trade (otherwise known as a swap) is a transaction that exchanges one token for another. In this guide, you will learn how to:

-   Estimate a swap price
-   Execute a swap with regular accounts (EOAs) or Smart Accounts

### Price estimation

The SDK provides two methods for estimating swap prices:

Method

Use Case

Function

Quick estimate

UI displays, real-time rates, liquidity checks

\- `getSwapPrice` (Ts)  
\- `get_swap_price` (Py)

Swap quote

Pre-execution, approvals, custom handling

\- `quoteSwap` (Ts)  
\- `quote_swap` (Py)

### CDP vs. external libraries

You can execute swaps using either:

1.  **CDP Wallet Integration** (Recommended for most use cases)
    -   Use our managed wallet infrastructure
    -   Automatic transaction signing and submission
    -   Built-in security and compliance features
    -   **Smart Account support** with gas sponsorship via paymasters
    -   See the [Server Wallet v2](https://developer.chrome.com/server-wallets/v2/introduction/quickstart) guide for details on setup and the [Server Wallet Swaps guide](https://developer.chrome.com/server-wallets/v2/evm-features/swaps) for more information on executing swaps
2.  **External Library Integration** (For custom infrastructure)
    -   Use libraries like [viem](https://viem.sh/) or [web3.py](https://pypi.org/project/web3)
    -   Full control over transaction signing
    -   Support for custom wallet types (including Smart Accounts with additional setup)
    -   Continue reading below for examples

Both options include built-in [slippage](https://developer.chrome.com/trade-api/quickstart#slippage) protection to ensure your swap executes at a fair price, even in volatile market conditions. Gas fees are automatically calculated and optimized for the most cost-effective route.

### Slippage

Slippage is the difference between the expected price of a trade and the actual price at which it executes.

More on slippage protection

In these examples, we set a 1% slippage tolerance (using `slippageBps: 100` in TypeScript or `slippage_bps: 100` in Python), meaning the trade will only execute if the final price is within 1% of the expected price.This **protects** you from unfavorable trades if the price moves significantly between when you submit the transaction and when it’s executed.

## Prerequisites

It is assumed you have:

-   [Node.js](https://nodejs.org/en) 22.x+ if using Typescript
-   [Python](https://www.python.org/downloads/) 3.10+ if using Python
-   [Created](https://portal.cdp.coinbase.com/create-account) and [signed in](https://portal.cdp.coinbase.com/signin) to an existing CDP account
-   Created the necessary keys to authenticate requests (see the [prerequisites](https://developer.chrome.com/server-wallets/v2/introduction/quickstart#prerequisites) section in v2 Server Wallet for setup instructions)
-   (Optional) For Smart Accounts: Understanding of [account abstraction (ERC-4337)](https://eips.ethereum.org/EIPS/eip-4337) concepts

## Regular Accounts (EOAs)

### 1\. Estimate a swap price

To begin, let’s walk through an example of how to estimate a swap price with a regular account (EOA).

📖 **Full examples**: [TypeScript](https://github.com/coinbase/cdp-sdk/blob/main/examples/typescript/evm/swaps/getSwapPrice.ts) | [Python](https://github.com/coinbase/cdp-sdk/blob/main/examples/python/evm/swaps/get_swap_price.py)

### 2\. Create a swap quote

Once you’re ready to commit to a swap, you can create a swap quote using the CDP API. This gives you the transaction data needed for execution as opposed to the quick price estimate that we demonstrated above.

📖 **Full examples**: [TypeScript](https://github.com/coinbase/cdp-sdk/blob/main/examples/typescript/evm/swaps/account.quoteSwap.ts) | [Python](https://github.com/coinbase/cdp-sdk/blob/main/examples/python/evm/swaps/account.quote_swap.py)

### 3\. Execute a swap

Now that we have a swap quote, we can execute it onchain. The easiest way to do this is by using CDP Wallets. We also offer a smoother developer experience using our Server Wallet. Read the [Server Wallet Swaps guide](https://developer.chrome.com/server-wallets/v2/evm-features/swaps) for more information.

📖 **Full examples**:

-   Quote & Execute: [TypeScript](https://github.com/coinbase/cdp-sdk/blob/main/examples/typescript/evm/swaps/account.quoteSwapAndExecute.ts) | [Python](https://github.com/coinbase/cdp-sdk/blob/main/examples/python/evm/swaps/account.quote_swap_and_execute.py)
-   All-in-one: [TypeScript](https://github.com/coinbase/cdp-sdk/blob/main/examples/typescript/evm/swaps/account.swap.ts) | [Python](https://github.com/coinbase/cdp-sdk/blob/main/examples/python/evm/swaps/account.swap.py)

## Smart Accounts

Smart Accounts provide additional features like gas sponsorship and batch operations. The key differences are:

-   Use the Smart Account address (not the owner’s EOA) as the `taker`
-   Transactions return `userOpHash` instead of `transactionHash`
-   Must wait for user operations to complete

### 1\. Estimate a swap price

When estimating prices for Smart Accounts, use the Smart Account address as the `taker`:

📖 **Full examples**: [TypeScript](https://github.com/coinbase/cdp-sdk/blob/main/examples/typescript/evm/swaps/getSwapPrice.ts) | [Python](https://github.com/coinbase/cdp-sdk/blob/main/examples/python/evm/swaps/get_swap_price.py)

### 2\. Create a swap quote

Once you’re ready to commit to a swap, you can create a swap quote using the CDP API. This gives you the transaction data needed for execution as opposed to the quick price estimate that we demonstrated above.

📖 **Full examples**: [TypeScript](https://github.com/coinbase/cdp-sdk/blob/main/examples/typescript/evm/smart-accounts/smartAccount.quoteSwap.ts) | [Python](https://github.com/coinbase/cdp-sdk/blob/main/examples/python/evm/smart-accounts/smart_account.quote_swap.py)

### 3\. Execute a swap

Now that we have a swap quote, we can execute it onchain. The easiest way to do this is by using CDP Wallets. Executing swaps with Smart Accounts returns a user operation hash instead of a transaction hash. You must wait for the user operation to complete:

📖 **Full examples**:

-   Quote & Execute: [TypeScript](https://github.com/coinbase/cdp-sdk/blob/main/examples/typescript/evm/smart-accounts/smartAccount.quoteSwapAndExecute.ts) | [Python](https://github.com/coinbase/cdp-sdk/blob/main/examples/python/evm/smart-accounts/smart_account.quote_swap_and_execute.py)
-   All-in-one: [TypeScript](https://github.com/coinbase/cdp-sdk/blob/main/examples/typescript/evm/smart-accounts/swap.ts) | [Python](https://github.com/coinbase/cdp-sdk/blob/main/examples/python/evm/smart-accounts/swap.py)

## Using External Wallets

If you prefer to use your own wallet, signing infrastructure, and node for broadcasting transactions, you can use the core Trade APIs (like `getSwapPrice` and `quoteSwap`) without a CDP account. 📖 **Full examples**: [TypeScript](https://github.com/coinbase/cdp-sdk/blob/main/examples/typescript/evm/ecosystem/viem/viem.account.swap.ts) | [Python](https://github.com/coinbase/cdp-sdk/blob/main/examples/python/evm/ecosystem/web3py/web3_account.quote_swap_and_execute.py)

## What to read next

-   **[Welcome](https://developer.chrome.com/trade-api/welcome#why-use-swap-api-over-aggregator-solutions):** Read why Trade API is easier to use than traditional aggregator solutions.
-   **[API Reference](https://developer.chrome.com/api-reference/v2/introduction):** Explore the full CDP API v2 documentation.
-   **[Server Wallet v2](https://developer.chrome.com/server-wallets/v2/introduction/quickstart):** Learn more about our new Server Wallet, including account management and transaction signing.
-   **[USDC Rewards](https://developer.chrome.com/server-wallets/v2/introduction/usdc-rewards):** Learn how to earn 3.35% rewards on USDC balances in your CDP wallets.