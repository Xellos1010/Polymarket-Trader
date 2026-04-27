# createcdpembeddedwallet

```
function createCDPEmbeddedWallet<chains>(_parameters: CDPEmbeddedWalletConfig<chains>): CDPEmbeddedWallet;

```

Creates the CDP embedded wallet’s 1193 provider. Note: The transports are currently only used for non-Base transactions. For non-Base transactions, the provider internally signs the transaction via the CDP APIs and broadcasts it via the provided transports, whereas for Base transactions the CDP API both signs and broadcasts the transaction. For more information on transports, see [Wagmi’s `createConfig` setup](https://wagmi.sh/react/api/createConfig).

## Type Parameters

Type Parameter

`chains` *extends* readonly \[`Chain`, `Chain`\]

## Parameters

Parameter

Type

Description

`_parameters`

[`CDPEmbeddedWalletConfig`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/CDPEmbeddedWalletConfig)<`chains`\>

Configuration parameters for the connector - see [CDPEmbeddedWalletConfig](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/CDPEmbeddedWalletConfig)

## Returns

[`CDPEmbeddedWallet`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/CDPEmbeddedWallet) A CDP embedded wallet instance

## Examples

```
import { createCDPEmbeddedWallet, initialize } from "@coinbase/cdp-core";
import { http } from "viem";
import { baseSepolia, sepolia } from "viem/chains";
// SDK core must be initialized before creating the wallet
await initialize({
  projectId: "your-project-id"
})
// Create a wallet with multiple chains
const wallet = createCDPEmbeddedWallet({
  chains: [baseSepolia, sepolia],
  transports: {
    [baseSepolia.id]: http(),
    [sepolia.id]: http(),
  },
  announceProvider: true, // Announce the provider to window.ethereum
});
// The provider can be accessed via the provider property
const provider = wallet.provider;
// The provider implements the EIP-1193 interface
await provider.request({ method: "eth_requestAccounts" });

```

```
// Basic usage with default configuration
const wallet = createCDPEmbeddedWallet();
const provider = wallet.provider;
// Request account access
const accounts = await provider.request({
  method: "eth_requestAccounts"
});
// Sign a message
const signature = await provider.request({
  method: "personal_sign",
  params: ["Hello, World!", accounts[0]]
});
// Send a transaction
const txHash = await provider.request({
  method: "eth_sendTransaction",
  params: [{
    from: accounts[0],
    to: "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6",
    value: "0x1000000000000000000" // 1 ETH
  }]
});
// Listen for connection events
provider.on("connect", (connectInfo) => {
  console.log("Connected to chain:", connectInfo.chainId);
});
provider.on("disconnect", () => {
  console.log("Disconnected from wallet");
});

```