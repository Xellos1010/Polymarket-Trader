# batching instructions

## Overview

A Solana transaction is a list of instructions that are executed in order. This allows developers to batch multiple instructions into a single transaction, reducing the number of transactions required to complete a complex multi-step process.

## Prerequisites

It is assumed you have already completed the [Quickstart](https://developer.chrome.com/server-wallets/v2/introduction/quickstart) guide.

## Create and send transaction with multiple instructions

In this example, we will:

-   Create a Solana account
-   Construct multiple instructions to be executed in the transaction
-   Sign the transaction with the Solana account
-   Send the transaction to the Solana network

After running the above snippet, you should see output similar to the following:

```
Created account: Af8cVHK2DZXcT4WhK6VDZ3h2zFxbEfgamsRkrB7dUcfF
Waiting for funds...
Account funded with 0.00125 SOL (1250000 lamports)
Sending transaction...
Waiting for transaction to be confirmed...
Sent SOL: https://explorer.solana.com/tx/56oRrY2nHSbncysmrW6vtBaUoyvWnRrMqN1joGNzaY3TNmPSTM653skDjbj2jDEdMA4QqFo9c4GY4hTnRhScgJk5?cluster=devnet

```

## What to read next

-   [v2 API Reference documentation](https://developer.chrome.com/api-reference/v2/authentication): Learn how to use Wallet Secrets to authenticate requests to the v2 Server Wallet.
-   [Sponsor Solana Transactions](https://developer.chrome.com/server-wallets/v2/solana-features/sponsor-transactions): Learn how to sponsor gas on Solana transactions.