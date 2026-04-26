# accounts

## Overview

**Accounts** refer to an address on a blockchain that has the ability to sign transactions on behalf of the address, allowing you to not only send and receive funds, but also interact with smart contracts. Cryptographically, an account corresponds to a **private/public key pair**.

The v2 Server Wallet supports the following account types:

-   **EVM Compatible Accounts**:
    -   **EOAs**: [Externally Owned Accounts](https://ethereum.org/en/developers/docs/accounts/) on any EVM-compatible blockchain that have the ability to sign transactions on behalf of an account’s address (i.e., when using a Smart Account).
    -   **Smart Account**: A smart contract-based account that can provide advanced functionality such as gas sponsorships and spend permissions.
-   **Solana Accounts**: An account on the Solana blockchain.

## EVM accounts

When using the v2 Server Wallet, ensure you understand the differences between our two offered account types, Externally Owned Accounts (EOAs) and Smart Accounts so that you select the proper type for your application. The v2 Server Wallet supports EOAs on **all EVM-compatible networks** and Smart Accounts on \*\*Base, Arbitrum, Optimism, Zora, Polygon, BNB, Avalanche and Ethereum Mainnet \*\*.

### EOA vs Smart Accounts

While both account types enable blockchain interactions, they differ significantly in their architecture, capabilities, and constraints:

Feature

EOA

Smart Account

**Control**

Private key generated and secured in CDP’s TEE

Controlled by smart contract code with an owner account (can be a CDP-managed EOA or bring your own)

**Creation**

Generated new or imported from existing private key

Created with CREATE2 opcode, deployed on first operation

**Transaction type**

Direct, signed blockchain transactions

Bundled transactions (user operations)

**Gas payment**

Must pay gas fees directly

Gas sponsorship available via paymaster (subsidized on Base Sepolia)

**Batch operations**

Single operation at a time

Multiple calls in a single user operation

**Owner requirements**

None required

Requires an owner account (CDP EOA or external)

**CDP limitations**

None

One smart account per owner, one owner per smart account

**Network support**

All EVM networks supported by CDP

Base, Arbitrum, Optimism, Zora, Polygon, BNB, Avalanche and Ethereum Mainnet

**Concurrent operations**

Can have multiple pending transactions

Support for concurrent userOperations

**viem compatibility**

Works seamlessly with viem for all onchain actions

Smart account owners work seamlessly with viem for all onchain actions

**web3/eth-account compatibility**

Works seamlessly with web3.py and [eth-account](https://web3py.readthedocs.io/en/stable/web3.eth.account.html) libraries for all onchain actions

Smart account owners work seamlessly with web3.py and [eth-account](https://web3py.readthedocs.io/en/stable/web3.eth.account.html) libraries for all onchain actions

**Faucet support**

Base, Ethereum, Solana

Base, Ethereum, Solana

### Use cases

**Use EOAs when:**

-   You need support across all EVM networks
-   You require simple wallet functionality
-   You don’t need gas sponsorship features

**Use Smart Accounts when:**

-   You’re building on Base Sepolia or Base Mainnet
-   You need to batch multiple operations in one transaction
-   You want to sponsor gas fees for users
-   You need EIP-4337 account abstraction features

### Implementation

EOAs are controlled directly by a private key.

#### EOAs

EOAs can be created new or imported from existing private keys. The following example shows both methods:

```
// Create a new EOA
const newAccount = await cdp.evm.createAccount();
// Import an existing EOA from private key
const importedAccount = await cdp.evm.importAccount({
  privateKey: "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
  name: "imported-account"
});

```

Here’s how to create an EOA and send a simple transaction:

For a complete example of creating and using EOAs, see the [quickstart guide](https://developer.chrome.com/server-wallets/v2/introduction/quickstart#evm).

#### Smart Accounts

Smart Accounts operate through deployed smart contracts, enabling advanced features through [EIP-4337 Account Abstraction](https://eips.ethereum.org/EIPS/eip-4337). When creating a Smart Account, an EOA must be provided as the owner (either a CDP-managed EOA or an external EOA). A Smart Account is not deployed until its first user operation:

```
const smartAccount = await cdp.evm.createSmartAccount({
  owner: evmAccount,
});
// Contract address is deterministic but not yet deployed
// Contract is deployed with the first user operation
const sendResult = await cdp.evm.sendUserOperation({
  smartAccount,
  network: "base-sepolia",
  calls: [/* ... */],
});

```

For detailed implementation examples including batch operations and gas sponsorship, see the [Smart Accounts guide](https://developer.chrome.com/server-wallets/v2/evm-features/smart-accounts).

## Solana accounts

Solana accounts represent addresses on the Solana blockchain that can hold SOL and other tokens. They function similarly to EOAs on EVM chains but with some key differences in capabilities and architecture.

### Features

Here’s what’s available for Solana accounts in the v2 Server Wallet:

Feature

Solana Account Support

Notes

**Control**

Private key secured in CDP’s [TEE](https://developer.chrome.com/server-wallets/v2/introduction/security)

Similar to EVM EOAs

**Creation**

Generate new or [import existing](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/import-accounts#solana-accounts-import-from-external-wallet-providers)

From base58 or raw private keys

**Transaction signing**

Native Solana transactions

See [sending transactions](https://developer.chrome.com/server-wallets/v2/solana-features/sending-transactions)

**Message signing**

Off-chain message signing

See [message signing](https://developer.chrome.com/server-wallets/v2/solana-features/message-signing)

**Gas payment**

Must pay fees directly

Sender pays transaction fees

**Fee sponsorship**

Via `feePayer` property

See [sponsoring transactions](https://developer.chrome.com/server-wallets/v2/solana-features/sponsor-transactions)

**Batch operations**

Multiple instructions per transaction

See [batching instructions](https://developer.chrome.com/server-wallets/v2/solana-features/batching-instructions)

**Token transfers**

SOL and SPL tokens

See [transfers](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/transfers)

**Program interactions**

Any Solana program

Full support

**Faucet support**

Solana devnet only

See [quickstart](https://developer.chrome.com/server-wallets/v2/introduction/quickstart#solana)

**Network support**

Mainnet and Devnet

`solana` and `solana-devnet`

### Implementation

Creating and using Solana accounts with the CDP Server Wallet is straightforward. This example demonstrates creating an account, funding it via faucet, and signing a message:

### Transaction signing

Beyond basic account operations, you’ll often need to sign and send **transactions**. While message signing, demonstrated above, is used to verify account ownership (e.g., for authentication or off-chain verification), transaction signing is used to authorize actual on-chain actions, such as transferring SOL or interacting with a program. The CDP Server Wallet integrates seamlessly with the Solana Web3.js library for transaction handling. For complete examples of creating Solana accounts and sending transactions, see:

-   [Quickstart guide](https://developer.chrome.com/server-wallets/v2/introduction/quickstart#solana): Basic Solana account creation and transactions using CDP with Solana’s Web3 library
-   [Sending transactions](https://developer.chrome.com/server-wallets/v2/solana-features/sending-transactions): How to send transactions using the CDP Server Wallet
-   [Batching Instructions](https://developer.chrome.com/server-wallets/v2/solana-features/batching-instructions): Execute multiple Solana instructions in a single transaction
-   [Sponsor Transactions](https://developer.chrome.com/server-wallets/v2/solana-features/sponsor-transactions): Learn about fee sponsorship on Solana

## What to read next

-   [**v2 Security**](https://developer.chrome.com/server-wallets/v2/introduction/security): Learn about the security features of v2 Server Wallet.
-   [**API Reference**](https://developer.chrome.com/api-reference/v2/introduction): Explore the complete API reference for v2 Server Wallet.