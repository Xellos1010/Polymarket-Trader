# faqs

## Basics

### What is Account Abstraction (ERC-4337)?

Account Abstraction gives onchain accounts programmable logic, enabling features like batched transactions, gas sponsorship, and passkey authentication. Operations are packaged as “userOperations” and processed through specialized components:

-   **Bundler**: Collects userOperations and submits them to the network
-   **Entrypoint**: The contract that orchestrates the transaction flow
-   **Paymaster**: A contract that covers gas fees on behalf of the user
-   **Smart Account**: A programmable wallet that validates signatures and executes transactions

No. CDP Paymaster combines Paymaster and Bundler into a single endpoint. You just use your Paymaster URL:

```
const hash = await bundlerClient.sendUserOperation({
  calls: [{
    abi: ContractAbi,
    functionName: 'mint',
    to: '0xContractAddress',
  }],
});

```

Gas sponsorship only works with **smart accounts**, not traditional EOAs. With [EIP-7702](#eip-7702), EOAs can be upgraded to smart accounts, enabling gas sponsorship.

### What’s the difference between an EOA and a smart account?

Feature

EOA

Smart Account

**Control**

Single private key

Onchain contract code

**Creation**

Instant (no deployment)

Requires contract deployment

**Functionality**

Basic transfers only

Batching, custom auth, programmable logic

**Gas & Fees**

Must hold ETH

Gas sponsorship, stablecoin payments

**Recovery**

No built-in recovery

Guardian-based recovery possible

### Which SDKs can I use with CDP Paymaster?

-   **CDP Embedded Wallets** — Use `useSendUserOperation` with `useCdpPaymaster: true`
-   **CDP Server Wallets** — Use [Managed Mode](https://developer.chrome.com/server-wallets/v2/evm-features/managed-mode#paymaster-integration) for automatic Paymaster integration
-   **Viem + Permissionless** — Direct bundler/paymaster client integration
-   **Any ERC-7677 compatible SDK**

See the [examples repository](https://github.com/coinbase/paymaster-bundler-examples) for implementations.

### Can I use the Paymaster on testnet?

Yes. CDP Paymaster supports Base Mainnet and Base Sepolia. Select your network in [CDP Portal](https://portal.cdp.coinbase.com/products/bundler-and-paymaster).

### Can I use third-party bundlers with CDP Paymaster?

No. CDP Paymaster works exclusively with CDP’s Bundler—they’re combined into a single endpoint.

* * *

## Billing

### How is billing calculated?

```
actualGasUsed × ethPriceUsd × 1.07

```

You’re charged based on actual gas used (from the `UserOperationEvent`) multiplied by ETH price in USD, plus a 7% markup. You’re billed monthly in USD via your CDP account.

### Where can I see my bill and usage?

-   **Billing tab** in [CDP Portal](https://portal.cdp.coinbase.com/) — View and download invoices
-   **Logs tab** in [Paymaster tool](https://portal.cdp.coinbase.com/products/bundler-and-paymaster) — Export detailed userOp logs

### How can I get Paymaster credits?

Developers can apply for up to **$15,000** in gas credits, with additional bonuses available through the Base Gasless Campaign. [Apply for credits](https://docs.google.com/forms/d/1yPnBFW0bVUNLUN_w3ctCqYM9sjdIQO3Typ53KXlsS5g/viewform?edit_requested=true&pli=1)

* * *

## EIP-7702

### What is EIP-7702?

[EIP-7702](https://eips.ethereum.org/EIPS/eip-7702) (part of Ethereum’s Pectra upgrade) lets you upgrade an EOA into a smart account **at the same address** by delegating to a smart contract’s code.

### What’s the difference between EIP-7702 and ERC-4337?

-   **EIP-7702**: Upgrades EOAs to smart accounts via code delegation
-   **ERC-4337**: The Account Abstraction infrastructure (userOps, paymasters, bundlers)

They work together — EIP-7702 lets you add ERC-4337 support to existing EOAs.

### Does CDP Paymaster support EIP-7702?

Yes. CDP Paymaster can sponsor transactions for EOAs that have been upgraded to support ERC-4337 validation.

### How do I upgrade an EOA with EIP-7702?

See the [Security page](https://developer.chrome.com/paymaster/reference-troubleshooting/security#eip-7702-security) for secure upgrade patterns and resources. The key points:

-   Don’t delegate directly to a wallet implementation (front-running risk)
-   Use `EIP7702Proxy` and call `setImplementation` atomically

### How can I tell if a wallet is upgraded?

Call `eth_getCode(address)`. If the result is not `0x`, the account is a smart account.