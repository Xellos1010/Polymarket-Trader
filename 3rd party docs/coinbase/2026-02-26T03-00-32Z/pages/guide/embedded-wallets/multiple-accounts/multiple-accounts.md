# multiple accounts

EVMSolana

## Overview

Users can create multiple blockchain accounts of each type within their Embedded Wallet. This enables organizing funds across different purposes, separating assets, and managing complex workflows without requiring separate wallet authentications. Key capabilities:

-   **Up to 10 EVM EOA accounts** per user
-   **Up to 10 Solana accounts** per user
-   **Up to 10 EVM Smart Accounts** per user (each requires a unique EOA owner)
-   **Account Metadata**: Each account includes creation timestamp and ownership information
-   **Flexible creation**: Create accounts on-demand based on your application’s needs

## Prerequisites

-   A CDP Portal account and project
-   Node.js 22+ and a package manager (npm, pnpm, or yarn)
-   `@coinbase/cdp-core` and `@coinbase/cdp-hooks` installed
-   User must be authenticated

## Account types

### EVM EOA Accounts

Externally Owned Accounts (EOAs) are standard Ethereum accounts controlled by private keys. Users can create up to 10 EOA accounts.

### Solana Accounts

Native Solana accounts for interacting with the Solana blockchain. Users can create up to 10 Solana accounts.

### EVM Smart Accounts

Smart Accounts (ERC-4337) are programmable wallets with advanced features like batch transactions and gas sponsorship. Each Smart Account requires a unique EOA owner, and users can create up to 10 Smart Accounts (one per EOA account).

## Creating additional accounts

### Using React Hooks

React Hooks provide the most convenient way to create accounts in React applications:

### Using Core SDK

For non-React applications, use the core SDK functions:

## Accessing account data

All user accounts are available through the `currentUser` object with metadata including creation timestamps and ownership information.

### List all accounts

```
import { useCurrentUser } from "@coinbase/cdp-hooks";
function AccountList() {
  const { currentUser } = useCurrentUser();
  if (!currentUser) return <div>Please sign in</div>;
  return (
    <div>
      <h2>EVM EOA Accounts</h2>
      {currentUser.evmAccountObjects?.map((account, index) => (
        <div key={account.address}>
          <p>Account {index + 1}: {account.address}</p>
          <p>Created: {new Date(account.createdAt).toLocaleDateString()}</p>
        </div>
      ))}
      <h2>EVM Smart Accounts</h2>
      {currentUser.evmSmartAccountObjects?.map((account, index) => (
        <div key={account.address}>
          <p>Account {index + 1}: {account.address}</p>
          <p>Owners: {account.ownerAddresses.join(", ")}</p>
          <p>Created: {new Date(account.createdAt).toLocaleDateString()}</p>
        </div>
      ))}
      <h2>Solana Accounts</h2>
      {currentUser.solanaAccountObjects?.map((account, index) => (
        <div key={account.address}>
          <p>Account {index + 1}: {account.address}</p>
          <p>Created: {new Date(account.createdAt).toLocaleDateString()}</p>
        </div>
      ))}
    </div>
  );
}

```

### Account object properties

#### EVM EOA

Property

Type

Description

`address`

`string`

The blockchain address of the account

`createdAt`

`string`

ISO 8601 timestamp when the account was created

#### EVM Smart Account

Property

Type

Description

`address`

`string`

The blockchain address of the account

`createdAt`

`string`

ISO 8601 timestamp when the account was created

`ownerAddresses`

`string[]`

Array of EOA addresses that own this Smart Account

#### Solana Account

Property

Type

Description

`address`

`string`

The blockchain address of the account

`createdAt`

`string`

ISO 8601 timestamp when the account was created

## Working with multiple accounts

### Selecting accounts for transactions

When sending transactions, specify which account to use:

## Use cases

Multiple accounts enable various organizational and functional patterns:

-   **Organizing funds by purpose**: Separate accounts for personal, business, and savings to keep funds organized
-   **Separating assets by network**: Different accounts optimized for specific networks or token types
-   **Managing different identities**: Maintain separate blockchain identities for public and private transactions
-   **Separating NFT collections**: Organize NFTs across different accounts by collection, purpose, or value
-   **Different trading strategies**: Separate accounts for different trading approaches or risk profiles
-   **Multi-purpose fund management**: Organize funds across multiple accounts based on liquidity needs
-   **Smart account configurations**: Different Smart Account setups for different use cases (e.g., sponsored vs. non-sponsored flows)
-   **Managing spending policies**: Separate Smart Accounts with different spending permissions and rules

## Reference

Resource

Description

[`useCreateEvmEoaAccount`](https://github.com/coinbase/cdp-web/blob/c7d5e5c5499e30934911d05e26fd2e9921b7e0b3/packages/hooks/src/hooks/useCreateEvmEoaAccount.ts)

React hook to create EVM EOA accounts

[`useCreateEvmSmartAccount`](https://github.com/coinbase/cdp-web/blob/c7d5e5c5499e30934911d05e26fd2e9921b7e0b3/packages/hooks/src/hooks/useCreateEvmSmartAccount.ts)

React hook to create EVM Smart Accounts

[`useCreateSolanaAccount`](https://github.com/coinbase/cdp-web/blob/c7d5e5c5499e30934911d05e26fd2e9921b7e0b3/packages/hooks/src/hooks/useCreateSolanaAccount.ts)

React hook to create Solana accounts

[`createEvmEoaAccount`](https://github.com/coinbase/cdp-web/blob/c7d5e5c5499e30934911d05e26fd2e9921b7e0b3/packages/core/src/actions/createEvmEoaAccount.ts)

Core SDK function to create EVM EOA accounts

[`createEvmSmartAccount`](https://github.com/coinbase/cdp-web/blob/c7d5e5c5499e30934911d05e26fd2e9921b7e0b3/packages/core/src/actions/createEvmSmartAccount.ts)

Core SDK function to create EVM Smart Accounts

[`createSolanaAccount`](https://github.com/coinbase/cdp-web/blob/c7d5e5c5499e30934911d05e26fd2e9921b7e0b3/packages/core/src/actions/createSolanaAccount.ts)

Core SDK function to create Solana accounts

[`useCurrentUser`](https://github.com/coinbase/cdp-web/blob/c7d5e5c5499e30934911d05e26fd2e9921b7e0b3/packages/hooks/src/hooks.ts#L568)

React hook to access current user and all accounts

[User type](https://github.com/coinbase/cdp-web/blob/c7d5e5c5499e30934911d05e26fd2e9921b7e0b3/packages/core/src/types.ts#L189)

TypeScript type definitions for user accounts

## What to read next