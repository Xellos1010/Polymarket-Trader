# cdp wagmi

This package provides an embedded wallet connector for [Wagmi](https://wagmi.sh/), which is a commonly used React Hooks library for Ethereum. Developers who use Wagmi in their application should be able to use the provided [createCDPEmbeddedWalletConnector](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-wagmi/Functions/createCDPEmbeddedWalletConnector) out of the box to integrate CDP embedded wallets into their existing environment.

## Quickstart

This guide will help you get started with `@coinbase/cdp-wagmi`. You’ll learn how to install the package, set up the provider, and render your first component.

### Installation

First, add the package to your project using your preferred package manager.

```
# With pnpm
pnpm add @coinbase/cdp-wagmi @coinbase/cdp-core @tanstack/react-query viem wagmi
# With yarn
yarn add @coinbase/cdp-wagmi @coinbase/cdp-core @tanstack/react-query viem wagmi
# With npm
npm install @coinbase/cdp-wagmi @coinbase/cdp-core @tanstack/react-query viem wagmi

```

### Gather your CDP Project information

1.  Sign in or create an account on the [CDP Portal](https://portal.cdp.coinbase.com/)
2.  On your dashboard, select a project from the dropdown at the at the top, and copy the Project ID

### Allowlist your local app

1.  Navigate to the [Embedded Wallet Configuration](https://portal.cdp.coinbase.com/products/embedded-wallets/cors) in CDP Portal, and click Add origin to include your local app
2.  Enter the origin of your locally running app - e.g., `http://localhost:3000`
3.  Click Add origin again to save your changes

### Setup Provider

Next, you must configure your WagmiProvider with the `CDPEmbeddedWalletConnector`. `CDPEmbeddedWalletConnector` provides the necessary context Wagmi to work correctly with the CDP Frontend SDK. The `providerConfig` must be provided and is responsible for configuring the EIP-1193 provider’s transports which are used to broadcast non-Base transactions.

```
import React from 'react';
import ReactDOM from 'react-dom/client';
import { App } from './App'; // Your main App component
import { Config }from '@coinbase/cdp-core';
import { createCDPEmbeddedWalletConector } from '@coinbase/cdp-wagmi';
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { http }from "viem";
import { baseSepolia, base } from 'viem/chains';
import { WagmiProvider, createConfig, http } from 'wagmi';
// Your CDP config
const cdpConfig: Config = {
  projectId: "your-project-id", // Copy your Project ID here.
}
const connector = createCDPEmbeddedWalletConnector({
 cdpConfig: cdpConfig,
 providerConfig:{
  chains: [base, baseSepolia],
  transports: {
    [base.id]: http(),
    [baseSepolia.id]: http()
  }
 }
});
const wagmiConfig = createConfig({
  connectors: [connector],
  chains: [base, baseSepolia],
  transports: {
    [base.id]: http(),
    [baseSepolia.id]: http(),
  },
});
const queryClient = new QueryClient(); // For use with react-query
ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <WagmiProvider config={wagmiConfig} >
      <QueryClientProvider client={ queryClient }>
        <App />
      </QueryClientProvider>
    </WagmiProvider>
  </React.StrictMode>,
);

```

#### Analytics Opt-Out

By default the SDK will emit usage analytics to help us improve the SDK. If you would like to opt-out, you can do so by setting the `disableAnalytics` configuration option to `true`.

```
const cdpConfig: Config = {
  projectId: "your-project-id", // Copy your Project ID here.
  disableAnalytics: true,
}

```

### Sign in and connection

In order to connect the CDP Embedded Wallet, the end application user must first go through the 2 step sign in flow. As a result, the consumer has 3 options:

1.  Use @coinbase/cdp-core’s `signInWithEmail` + `verifyEmailOTP`
2.  Use @coinbase/cdp-hooks’ `useSignInWithEmail` + `useVerifyEmailOTP`
3.  Use @coinbase/cdp-react `<SignIn />` component

After using any of these methods, the CDP embedded wallet’s connector should automatically connect.

### Call Wagmi Hooks

Now, your application should be able to successfully call Wagmi hooks. For example:

```
import { useState } from "react";
import { parseEther } from "viem";
import { useAccount, useSendTransaction, useWaitForTransactionReceipt } from "wagmi";
/**
 * The burn address (0x0000000000000000000000000000000000000000)
 */
const BURN_ADDRESS = "0x0000000000000000000000000000000000000000" as const;
/**
 * The amount to send in ETH (0.00001 ETH)
 */
const AMOUNT_TO_SEND = "0.00001";
/**
 * A component that demonstrates wagmi's useSendTransaction hook
 * by sending 0.00001 ETH to the burn address.
 *
 * @returns A component that allows the user to send a transaction using wagmi.
 */
export default function WagmiTransaction() {
  const { address } = useAccount();
  const [isLoading, setIsLoading] = useState(false);
  const { data: hash, sendTransaction, isPending, error } = useSendTransaction();
  const { isLoading: isConfirming, isSuccess } = useWaitForTransactionReceipt({
    hash,
  });
  const handleSendTransaction = async () => {
    if (!address) return;
    setIsLoading(true);
    try {
      sendTransaction({
        to: BURN_ADDRESS,
        value: parseEther(AMOUNT_TO_SEND),
      });
    } catch (error) {
      console.error("Failed to send transaction:", error);
    } finally {
      setIsLoading(false);
    }
  };
  const handleReset = () => {
    // Reset by refreshing the page or clearing state
    window.location.reload();
  };
  return (
    <div>
      <div>
        <p>
          ⚠️ Warning: This will send {AMOUNT_TO_SEND} ETH to the burn address (0x0000...0000). 
          This transaction cannot be reversed and the ETH will be permanently lost.
        </p>
      </div>
      <div>
        <div>
          <div>Amount: {AMOUNT_TO_SEND} ETH</div>
          <div>To (Burn Address): {BURN_ADDRESS.slice(0, 6)}...{BURN_ADDRESS.slice(-4)}</div>
          <div>From: {address?.slice(0, 6)}...{address?.slice(-4)}</div>
        </div>
      </div>
      {error && (
        <div>
          <strong>Error:</strong> {error.message}
        </div>
      )}
      {!hash && !isPending && !isLoading && (
        <button disabled={!address} onClick={handleSendTransaction}>
          Send {AMOUNT_TO_SEND} ETH to Burn Address
        </button>
      )}
      {(isPending || isConfirming) && (
        <div>
          <div>Sending transaction...</div>
          {hash && (
            <div>
              Hash: {hash.slice(0, 10)}...{hash.slice(-8)}
            </div>
          )}
        </div>
      )}
      {isSuccess && hash && (
        <div>
          <div>
            <div>✅</div>
          </div>
          <div>
            <div>Transaction Confirmed!</div>
            <div>Your transaction has been successfully sent to the burn address</div>
          </div>
          <div>
            <div>Amount: {AMOUNT_TO_SEND} ETH</div>
            <div>To: {BURN_ADDRESS.slice(0, 6)}...{BURN_ADDRESS.slice(-4)}</div>
            <div>
              Block Explorer:{" "}
              <a
                href={`https://sepolia.basescan.org/tx/${hash}`}
                target="_blank"
                rel="noopener noreferrer"
              >
                {hash.slice(0, 10)}...{hash.slice(-8)}
              </a>
            </div>
          </div>
          <button onClick={handleReset}>
            Send Another Transaction →
          </button>
        </div>
      )}
    </div>
  );
}

```

### Smart Accounts

This package has Smart Account support for the Wagmi `useSendCalls`, `useCallsStatus` and `useCapabilities` hooks. It requires a different CDP Config than for EOA wallets:

```
const cdpConfig: Config = {
  projectId: "your-project-id", // Copy your Project ID here.
  ethereum: {
    createOnLogin: "smart" // Create a smart account by default when a user logs in and does not yet have one
  }
}

```

After signing in, you can send a User Operation using Wagmi hooks:

```
import { useState } from "react";
import { parseEther } from "viem";
import { useAccount, useSendCalls, useSendCalls, useCallsStatus, useCapabilities } from "wagmi";
/**
 * The burn address (0x0000000000000000000000000000000000000000)
 */
const BURN_ADDRESS = "0x0000000000000000000000000000000000000000" as const;
/**
 * The amount to send in ETH (0.00001 ETH)
 */
const AMOUNT_TO_SEND = "0.00001";
/**
 * SendCalls component that allows users with Smart Accounts to send ETH to the burn address
 *
 * @returns JSX element with transaction sending functionality
 */
export function WagmiSendCalls() {
  const { address, isConnected } = useAccount();
  const { data: userOpHash, sendCalls, isPending, error } = useSendCalls();
  // Check the status of a sent user operation
  const { isLoading: isConfirming, isSuccess } = useCallsStatus({
    id: userOpHash as Hex,
    query: {
      enabled: !!userOpHash,
    },
  });
  const chainId = useChainId();
  const { data: walletCapabilities } = useCapabilities({
    chainId,
  });
  // Check the capabilities of the wallet to determine if you can use user operations
  const isSendCallsSupported = useMemo(() => {
    return walletCapabilities?.atomic?.status === "supported";
  }, [walletCapabilities]);
  const handleSendCalls = async () => {
    if (!isConnected || !address) return;
    try {
      sendCalls(
        {
          calls: [
            {
              to: BURN_ADDRESS,
              value: parseEther(AMOUNT_TO_SEND),
            },
          ],
        }
      );
    } catch (err) {
      console.log("Failed to send user operation: ", err)
    }
  };
  if (!isSendCallsSupported) {
    return (
      <div>
        <p>
          This wallet does not support sending calls on chain {chainId}. Ensure your wallet has a
          smart account, and is on a supported chain.
        </p>
      </div>
    );
  }
  return (
    <div>
      <div>
        <p>
          ⚠️ Warning: This will send {AMOUNT_TO_SEND} ETH to the burn address (0x0000...0000). 
          This operation cannot be reversed and the ETH will be permanently lost.
        </p>
      </div>
      <div>
        <div>
          <div>Amount: {AMOUNT_TO_SEND} ETH</div>
          <div>To (Burn Address): {BURN_ADDRESS.slice(0, 6)}...{BURN_ADDRESS.slice(-4)}</div>
          <div>From: {address?.slice(0, 6)}...{address?.slice(-4)}</div>
        </div>
      </div>
      {error && (
        <div>
          <strong>Error:</strong> {error.message}
        </div>
      )}
      {!userOpHash && !isPending && (
        <button disabled={!address} onClick={handleSendCalls}>
          Send {AMOUNT_TO_SEND} ETH to Burn Address
        </button>
      )}
      {(isPending || isConfirming) && (
        <div>
          <div>Sending transaction...</div>
          {hash && (
            <div>
              Hash: {hash.slice(0, 10)}...{hash.slice(-8)}
            </div>
          )}
        </div>
      )}
      {isSuccess && userOpHash && (
        <div>
          <div>
            <div>✅</div>
          </div>
          <div>
            <div>Operation Sent!</div>
            <div>Your operation has been successfully sent to the burn address</div>
          </div>
          <div>
            <div>Amount: {AMOUNT_TO_SEND} ETH</div>
            <div>To: {BURN_ADDRESS.slice(0, 6)}...{BURN_ADDRESS.slice(-4)}</div>
            <div>
              Block Explorer:{" "}
              <a
                href={`https://sepolia.basescan.org/tx/${userOpHash}`}
                target="_blank"
                rel="noopener noreferrer"
              >
                {hash.slice(0, 10)}...{hash.slice(-8)}
              </a>
            </div>
          </div>
          <button onClick={handleReset}>
            Send Another Operation →
          </button>
        </div>
      )}
    </div>
  );
}

```

**Transaction Attribution with EIP-8021:** You can add attribution data to Smart Account operations for tracking app usage and revenue sharing using the `dataSuffix` capability:

```
import { useSendCalls, useChainId } from "wagmi;
import { parseEther, type Hex } from "viem";
function SendCallsWithAttribution() {
  const { sendCalls, isPending } = useSendCalls();
  const chainId = useChainId();
  
  const handleSendWithAttribution = async () => {
    // EIP-8021 data suffix for "baseapp" attribution (Schema ID 0)
    const dataSuffix = "0xdddddddd62617365617070070080218021802180218021802180218021" as Hex;
    
    await sendCalls({
      calls: [
        {
          to: "0xTargetContract",
          value: parseEther("0.001"),
        }
      ],
      capabilities: {
        dataSuffix: {
          value: dataSuffix  // Attribution data appended to callData
        }
      }
    });
  };
  return (
    <button onClick={handleSendWithAttribution} disabled={isPending}>
      Send with Attribution
    </button>
  );
}

```

The `dataSuffix` capability accepts a hex-encoded string following the [EIP-8021 standard](https://eip.tools/eip/8021). The suffix is automatically appended to the user operation’s `callData` for onchain attribution, enabling usage tracking and revenue sharing mechanisms. **Checking Data Suffix Support:** Use `useCapabilities` to check if the wallet supports data suffix:

```
import { useCapabilities, useChainId } from "wagmi/experimental";
function CheckDataSuffixSupport() {
  const { data: capabilities } = useCapabilities();
  const chainId = useChainId();
  
  const supportsDataSuffix = capabilities?.[chainId.toString()]?.dataSuffix?.supported;
  
  return (
    <div>
      Data Suffix Support: {supportsDataSuffix ? "✅ Supported" : "❌ Not Supported"}
    </div>
  );
}

```