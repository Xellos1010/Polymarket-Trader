# smart accounts

## Overview

[Smart accounts](https://developer.chrome.com/server-wallets/v2/introduction/accounts#smart-accounts) are a type of account that can be used to execute user operations onchain. In this guide, you will learn how to:

-   Create an EVM smart account
-   Send a user operation from the smart account
-   Batch calls within a single user operation

## Prerequisites

It is assumed you have already completed the [Quickstart](https://developer.chrome.com/server-wallets/v2/introduction/quickstart) guide.

## 1\. Create a smart account

An **EVM smart account** is a smart contract account deployed on an EVM compatible network that provides the ability to batch transactions, sponsor gas, and manage spend permissions. Smart accounts require an **owner** account to sign on its behalf. In this example, we will only create the smart account, and use a CDP EVM account as the owner. Note that the smart contract is not deployed until the following step when you submit the first user operation.

After running the above snippet, you should see output similar to the following:

```
Created smart account: 0x7a3D84055994c3062819Ce8730869D0aDeA4c3Bf

```

## 2\. Send a [user operation](https://www.erc4337.io/docs/understanding-ERC-4337/user-operation)

A user operation is a transaction that is executed by a smart account. In this example, we will:

-   Create an [externally owned account](https://developer.chrome.com/server-wallets/v2/introduction/accounts#externally-owned-accounts)
-   Create a smart account with the EOA as the owner
-   Submit a user operation on Base Sepolia from the smart account which transfers 0 ETH to the EOA

After running the above snippet, you should see similar output:

```
Created owner account: 0x088a49cAf927B8DacEFc4ccFD0D5EAdeC06F19A2
Created smart account: 0x929444AFfd714c260bb6695c921bEB99d1D31ff7
User operation status: broadcast
Waiting for user operation to be confirmed...
User operation confirmed. Block explorer link: https://basescan.org/tx/0x8e66c974c8d1b2a75fee35e097fe9171d28c48066472bb6ed81ca81a10d3c321```

```

## 3\. Batch calls within a single user operation

A smart account can batch multiple calls in a single user operation through the `calls` field. In this example, we will:

-   Create an [externally owned account](https://developer.chrome.com/server-wallets/v2/introduction/accounts#externally-owned-accounts)
-   Create a smart account with the EOA as the owner
-   Fund the smart account using a faucet
-   Submit a batch transaction with 3 calls

After running the above snippet, you should see output similar to the following:

```
Created smart account: 0xA557E90004ba5406A3553897e99D1FC5A2685F6d
Faucet transaction confirmed: 0xa691fcfd1dcacad1ef144461e9c2f1fc110172f0fcfe9a10cbc83e5ca2b6b610
Sending user operation to three destinations...
Waiting for user operation to be confirmed...
User operation confirmed. Block explorer link: https://sepolia.basescan.org/tx/0xd01b2089fd6d4673eae0d7629bcdf5488ff950dba2b7741b4725632f29e9f1ab

```

## Debugging user operation failure

When a user operation reverts onchain, the reason for the revert is included in its receipt, if it can be decoded. You can use the SDK to fetch the user operation and inspect its receipt as shown below.

## What to read next

-   [**Spend Permissions**](https://developer.chrome.com/server-wallets/v2/evm-features/spend-permissions): Learn more about spend permissions
-   [**Gas Sponsorship**](https://developer.chrome.com/server-wallets/v2/evm-features/gas-sponsorship): Sponsor gas fees for your users’ transactions