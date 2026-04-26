# managed mode

## Overview

The CDP SDK offers a simple and flexible API to enhance EVM Accounts with production-ready infrastructure. This feature offers:

1.  **Automatic CDP Node Integration**: Accounts automatically use CDP’s managed RPC endpoints and Paymaster services
2.  **Network-Scoped Account Instances**: Create network-scoped account instances with `useNetwork()` for improved developer experience
3.  **Bring Your Own Node**: Flexibility to use custom RPC endpoints and Paymasters when needed
4.  **Enhanced Type Safety**: Network-scoped method availability with TypeScript support

## Network-Scoped Account Instances

### Managing Network-Scoped Accounts

The `useNetwork()` method lets you initialize network-scoped account instances, helping you avoid repeatedly specifying the network in requests. This provides a more intuitive developer experience by scoping account operations to a specific network.

```
import { CdpClient } from "@coinbase/cdp-sdk";
import "dotenv/config";
const cdp = new CdpClient();
const account = await cdp.evm.getOrCreateAccount({
  name: "My-Account",
});
// Scope an account instance to Base.
const baseAccount = await account.useNetwork("base");
// All subsequent operations automatically use "base" as the network.
const transaction = await baseAccount.sendTransaction({
  transaction: {
    to: "0x0000000000000000000000000000000000000000",
    value: 0n,
  },
});
// Wait for transaction receipt will automatically use your CDP Node RPC.
const receipt = await baseAccount.waitForTransactionReceipt(transaction);
console.log("Transaction confirmed:", receipt.transactionHash);

```

### Network-Scoped Method Availability

The SDK enhances type safety by exposing network-specific methods based on the selected network. For example, faucet functionality is only available on testnets, and attempting to use it on a mainnet will result in a type error.

```
import { CdpClient } from "@coinbase/cdp-sdk";
import "dotenv/config";
const cdp = new CdpClient();
const account = await cdp.evm.createAccount();
// Base Sepolia testnet - supports faucet requests.
const baseSepoliaAccount = await account.useNetwork("base-sepolia");
// TypeScript knows these methods are available.
await baseSepoliaAccount.listTokenBalances({});
await baseSepoliaAccount.requestFaucet({ token: "eth" }); // ✅ Available on testnet
// Base mainnet - does not support faucet requests.
const baseAccount = await account.useNetwork("base");
// TypeScript knows this method is available.
await baseAccount.listTokenBalances({});
// TypeScript would error on this: 
// await baseAccount.requestFaucet({ token: "eth" }); // ❌ TypeScript error

```

## Bundle CDP Node and Paymaster with EVM Accounts

### Default CDP Node Usage

When you use network-specific account instances with Base or Base Sepolia, the SDK automatically integrates with the free [CDP Base Node](https://www.coinbase.com/developer-platform/products/base-node) offering, providing:

-   **Managed RPC Endpoints**: Automatic connection to CDP’s optimized RPC infrastructure
-   **Paymaster Integration**: Automatic gas sponsorship for smart accounts

#### Managed RPC Endpoints

The [Managing Network-Scoped Accounts](https://developer.chrome.com/server-wallets/v2/evm-features/managed-mode#managing-network-scoped-accounts) section above demonstrates how network-scoped accounts automatically use CDP Node RPC. The important bits are displayed below:

```
// Scope an account instance to Base.
const baseAccount = await account.useNetwork("base");
// ... send a transaction ...
// Wait for transaction receipt will automatically use your CDP Node RPC.
const receipt = await baseAccount.waitForTransactionReceipt(transaction);

```

When using accounts scoped to Base or Base Sepolia (i.e. calling `useNetwork("base")` or `useNetwork("base-sepolia")`), the SDK automatically uses the CDP Base Node RPC.

#### Paymaster Integration

The following code shows how Smart Accounts automatically use CDP Paymaster for gas sponsorship.

```
import { CdpClient } from "@coinbase/cdp-sdk";
import "dotenv/config";
const cdp = new CdpClient();
const account = await cdp.evm.getOrCreateAccount({
  name: "My-Account",
});
const smartAccount = await cdp.evm.getOrCreateSmartAccount({
  name: "My-Smart-Account",
  owner: account,
});
// Scope an account instance to Base.
const baseSmartAccount = await smartAccount.useNetwork("base");
// Automatically uses CDP Paymaster for gas sponsorship.
const userOperation = await baseSmartAccount.sendUserOperation({
  calls: [
    {
      to: "0x0000000000000000000000000000000000000000",
      value: 0n,
      data: "0x",
    },
  ],
});
const receipt = await baseSmartAccount.waitForUserOperation(userOperation);
console.log("User operation confirmed:", receipt);

```

Note that even generic accounts can use CDP Paymaster for gas sponsorship, just by specifying Base as the network.

```
import { CdpClient } from "@coinbase/cdp-sdk";
import "dotenv/config";
const cdp = new CdpClient();
const account = await cdp.evm.getOrCreateAccount({
  name: "My-Account",
});
const smartAccount = await cdp.evm.getOrCreateSmartAccount({
  name: "My-Smart-Account",
  owner: account,
});
// Automatically uses CDP Paymaster for gas sponsorship.
const userOperation = await smartAccount.sendUserOperation({
  network: "base",
  calls: [
    {
      to: "0x0000000000000000000000000000000000000000",
      value: 0n,
      data: "0x",
    },
  ],
});
const receipt = await smartAccount.waitForUserOperation(userOperation);
console.log("User operation confirmed:", receipt);

```

## Bring Your Own Node

### Using Custom RPC URLs

For greater control or to use networks not natively supported by the CDP API, you can specify custom RPC URLs when creating network-scoped account instances.

```
import { CdpClient } from "@coinbase/cdp-sdk";
import "dotenv/config";
const cdp = new CdpClient();
const account = await cdp.evm.getOrCreateAccount({
  name: "My-Account",
});
// Use a custom RPC URL for Polygon.
const polygonAccount = await account.useNetwork(
  "https://polygon-mainnet.rpc-provider.com/YOUR_API_KEY"
);
// Uses the provided RPC URL instead of CDP Node.
const transaction = await polygonAccount.sendTransaction({
  transaction: {
    to: "0x0000000000000000000000000000000000000000",
    value: 0n,
  },
});
// Transaction monitoring also uses the provided RPC URL.
const receipt = await polygonAccount.waitForTransactionReceipt(transaction);
console.log("Transaction confirmed on custom RPC:", receipt.transactionHash);

```

## What to read next

-   [**Smart Accounts**](https://developer.chrome.com/server-wallets/v2/evm-features/smart-accounts): Learn about EIP-4337 smart accounts and gas sponsorship
-   [**Gas Sponsorship**](https://developer.chrome.com/server-wallets/v2/evm-features/gas-sponsorship): Understand how CDP’s Paymaster works
-   [**Sending Transactions**](https://developer.chrome.com/server-wallets/v2/evm-features/sending-transactions): Basic transaction sending guide