# createcdpembeddedwalletconnector

```
function createCDPEmbeddedWalletConnector(parameters: {
  cdpConfig: Config;
  providerConfig: CDPEmbeddedWalletConfig;
}): CreateConnectorFn<unknown>;

```

Creates a wagmi-compatible connector that wraps our EIP1193 provider. Some internals referenced from [https://github.com/wevm/wagmi/blob/main/packages/connectors/src/coinbaseWallet.ts](https://github.com/wevm/wagmi/blob/main/packages/connectors/src/coinbaseWallet.ts) In order to connect a CDP wallet the user must first sign in via our hooks or prebuilt SignIn component. The connector will automatically emit the connect event when the user is connected.

## Parameters

Parameter

Type

Description

`parameters`

{ `cdpConfig`: `Config`; `providerConfig`: `CDPEmbeddedWalletConfig`; }

Configuration parameters for the connector

`parameters.cdpConfig`

`Config`

\- CDP core SDK configuration

`parameters.providerConfig`

`CDPEmbeddedWalletConfig`

\- Configuration for the EIP1193 provider

## Returns

`CreateConnectorFn`<`unknown`\> A wagmi-compatible connector that wraps the EIP1193 provider

## Examples

```
import { createCDPEmbeddedWalletConnector } from "@coinbase/cdp-wagmi";
import { createConfig, http } from "wagmi";
import { baseSepolia, sepolia } from "viem/chains";
// Create the CDP connector
const cdpConnector = createCDPEmbeddedWalletConnector({
  cdpConfig: {
    projectId: "your-project-id",
  },
  providerConfig: {
    chains: [baseSepolia, sepolia],
    transports: {
      [baseSepolia.id]: http(),
      [sepolia.id]: http(),
    },
    announceProvider: true,
  },
});
// Use with wagmi config
const config = createConfig({
  chains: [baseSepolia, sepolia],
  connectors: [cdpConnector],
  transports: {
    [baseSepolia.id]: http(),
    [sepolia.id]: http(),
  },
});

```

```
import { createCDPEmbeddedWalletConnector } from "@coinbase/cdp-wagmi";
import { useConnect, useAccount, useDisconnect } from "wagmi";
import { baseSepolia } from "viem/chains";
import { SignIn } from "@coinbase/cdp-react";
// Create connector with minimal configuration
const cdpConnector = createCDPEmbeddedWalletConnector({
  cdpConfig: {
    projectId: "your-project-id",
  },
});
function SignInComponent() {
  const { address } = useAccount();
  if (!address) {
    return (
      <SignIn />
    )
  }
  return (
    <WalletComponent />
  )
}
function WalletComponent() {
  const { address } = useAccount();
  return (
    <div>
      Connected with CDP Wallet: {address}
    </div>
  );
}

```