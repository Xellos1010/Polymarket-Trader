# transfers

## Overview

A **transfer** refers to sending tokens from one account to another on a blockchain network. Token transfers are supported on EVM Accounts, EVM Smart Accounts and Solana Accounts. EVM Accounts can transfer native ETH or ERC-20 tokens, and Solana Accounts can transfer native SOL or SPL tokens.

## Prerequisites

-   Python 3.10+ or Node.js 20+
-   CDP API credentials (See the [Quickstart](https://developer.chrome.com/server-wallets/v2/introduction/quickstart#create-keys) guide for more information)
-   Required packages:

If you’re using Solana, you’ll want to install `@solana/web3.js` instead of `viem` on TypeScript, and `solana` instead of `web3` on Python:

## EVM Account transfer

This section will walk you through transferring funds from an externally owned account (EOA).

### 1\. Create accounts

First, create two EOAs:

### 2\. Transfer ETH to account

Next, transfer ETH using the accounts you created earlier.

### 3\. Wait for transfer result

Finally, wait for the transfer to be confirmed.

## Smart Account transfer

This section will walk you through transferring funds from a Smart Account to an EOA.

### 1\. Create Smart Account

First, we create an owner account to assign to our newly created Smart Account:

### 2\. Create receiver account

Create a receiver account:

### 3\. Transfer ETH to account

Perform the transfer using the smart account:

### 4\. Wait for transfer result

Finally, wait for the transfer to be confirmed.

## Solana Account transfer

This section will walk you through transferring funds from a Solana Account.

### 1\. Create accounts

First, create two Solana Accounts:

### 2\. Transfer SOL to account

Perform the transfer using the Solana Account:

### 3\. Wait for transfer result

Finally, wait for the transfer to be confirmed.

## Complete example

A complete, runnable example can be found below. In the code below, we:

1.  Create two EVM EOAs
2.  Transfer ETH from one EOA to another
3.  Wait for the transfer to be confirmed
4.  Create a Smart Account
5.  Transfer ETH from the Smart Account to an EOA
6.  Wait for the transfer to be confirmed
7.  Create two Solana Accounts
8.  Transfer SOL from one Solana Account to another
9.  Wait for the transfer to be confirmed

After running the complete example, you should see the following:

```
=== EOA Example ===
Transferring 0.00001 ETH from 0x689c59617D8Ec93a114E2304cC038bB8678775C7 to 0xe4026d8D0fA814379042f1E245096F0551931d14...
Transfer status: success
Explorer link: https://sepolia.basescan.org/tx/0xdaa53830fb62f407dfe65c6e10bac6c9af1fb2014551d107f7d9de2055914985
=== Smart Account Example ===
Transferring 0.00001 ETH from 0xC3c2D7879B31Aca4e26D16AD57D07422E4a23A67 to 0xe4026d8D0fA814379042f1E245096F0551931d14...
Transfer status: complete
Explorer link: https://sepolia.basescan.org/tx/0x045e29f40897dc01b50a6ba0d7c3d0a424fbca69792869f56e1321f1a899419f
=== Solana Account Example ===
Transferring 0.0001 SOL from DYjMQTJCcqmtdMvagJvW17U7teg8caUQ92nSrcXdqSZG to 32gPVc5gDvn2T5EgV81NFavDMpf5HU4oLUbmuipKpV8C...
Transaction confirmed! Link: https://explorer.solana.com/tx/4KEPbhkRLTg2FJNqV5bbUd6zv1TNkksxF9PDHw2FodrTha3jq2Cojn4hSKtjPWdrZiRDuYp7okRuc1oYvh3JkLuE?cluster=devnet

```

## Transferring ERC-20 Tokens or SPL Tokens

In the examples above, we transferred native tokens (`eth` and `sol`). Using the `transfer` method, you can also easily transfer ERC-20 tokens or SPL tokens, like USDC. The `transfer` method itself can receive `usdc` as the value for the `token` parameter:

Notice the value passed to the `amount` parameter is specified as `10000` for USDC. This is because USDC has 6 decimal places, so `10000` is equivalent to `0.01 USDC`. To work with more human-readable values, you can use the `parseUnits` function from `viem` if using TypeScript, or CDP’s `parse_units` function if using Python:

The `transfer` method can also receive an arbitrary token contract address as the value for the `token` parameter:

## Video: Watch and learn

**Watch this video for a walkthrough of the transfer process:**

## What to read next

-   [Managing Accounts](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/managing-accounts): Learn tips and best practices for managing your accounts
-   [Accounts](https://developer.chrome.com/server-wallets/v2/introduction/accounts): More information on the different types of accounts available using the v2 Server Wallet