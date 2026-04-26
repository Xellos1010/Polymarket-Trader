# sending transactions

## Overview

This guide covers the fundamentals of sending transactions on Solana using the CDP v2 Server Wallet. You will learn how to construct transactions and submit them to the Solana network. Solana transactions benefit from a single API that supports signing and broadcasting. With CDP, signing keys are securely managed in the platform’s infrastructure.

1.  **Create transaction**: Build a transaction with one or more instructions
2.  **Send transaction**: Use CDP to send the serialized transaction
3.  **Confirm transaction**: Wait for network confirmation

## Prerequisites

It is assumed you have:

-   Completed the [Quickstart](https://developer.chrome.com/server-wallets/v2/introduction/quickstart) guide
-   Basic understanding of [Solana accounts](https://developer.chrome.com/server-wallets/v2/introduction/accounts#solana-accounts)
-   Installed dependencies:
    -   For TypeScript: `@solana/web3.js`, `@coinbase/cdp-sdk`, and `dotenv`
    -   For Python: `solana`, `solders`, `cdp-sdk`, and `python-dotenv`

## 1\. Create a Solana account

First, create or retrieve a Solana account using CDP. The below example uses `solana-devnet` and will source `SOL` from CDP faucet to transfer.

## 2\. Build the transaction

Prepare a transaction with one or more instructions. The transaction may contain several instructions, each of which may require signatures from different account keys. Here is a simple SOL transfer.

## 3\. Serialize transaction

Serialize the transaction for signing:

## 4\. Send the transaction

Use CDP to send the serialized transaction:

## 5\. Confirm

Wait for confirmation after you submitted the transaction to the network.

## Complete example

View complete runnable examples

Here’s the complete, runnable example for easy copy and paste:

## Optimizing transaction sends

### Priority Fee

When sending a transaction on Solana, you can set a priority fee, which is an optional fee that increases the likelihood of your transaction being included in the next block by validators. The priority fee is calculated based on the *compute unit price* and *compute unit limit* of the transaction. *Priority Fee* = *Compute Unit Limit* × *Compute Unit Price* Read more about [Solana’s fee model](https://solana.com/docs/core/fees).

To set your own *compute unit limit* and *compute unit price* for a transaction, take a look at the examples below.

#### Compute Unit Price

#### Compute Unit Limit

## Bring your own node

Use a custom node provider to build and broadcast the transaction instead of relying on the CDP SDK. You will still call CDP to sign the transaction, but you can query the recent blockhash and broadcast the transaction using your own node.

How CDP handles transaction signing

When you call `cdp.solana.signTransaction()`, you send an unsigned transaction to CDP’s secure infrastructure. CDP signs it using your managed private key (which never leaves the [Trusted Execution Environment](https://developer.chrome.com/server-wallets/v2/introduction/security#tee-architecture)) and returns the signed transaction.

View a complete example on using your own node

## What to read next

-   [Batching instructions](https://developer.chrome.com/server-wallets/v2/solana-features/batching-instructions): Learn how to batch multiple instructions efficiently
-   [Sponsor transactions](https://developer.chrome.com/server-wallets/v2/solana-features/sponsor-transactions): Implement fee sponsorship for better UX
-   [Solana accounts](https://developer.chrome.com/server-wallets/v2/introduction/accounts#solana-accounts): Understand Solana account management in CDP
-   [API reference](https://developer.chrome.com/api-reference/v2/authentication): Complete API documentation for all Solana operations