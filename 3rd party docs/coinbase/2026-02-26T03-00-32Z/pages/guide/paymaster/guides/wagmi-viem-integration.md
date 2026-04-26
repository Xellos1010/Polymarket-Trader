# wagmi viem integration

This guide covers how to integrate CDP Paymaster for gasless transactions in a Wagmi project using the Base Account SDK. You’ll configure Wagmi with the `baseAccount` connector and use Wagmi’s hooks to execute sponsored transactions.

## Prerequisites

-   A [CDP account](https://portal.cdp.coinbase.com/) with Paymaster enabled
-   Node.js 22+ and a package manager (npm, pnpm, yarn, or bun)
-   Basic familiarity with React and Wagmi

## Step 1: Set Up CDP Paymaster

1.  [Sign in](https://portal.cdp.coinbase.com/) to your CDP account
2.  Navigate to [**Onchain Tools > Paymaster**](https://portal.cdp.coinbase.com/products/bundler-and-paymaster)
3.  Select your network (**Base** or **Base Sepolia**) in the top right
4.  Copy your **Paymaster & Bundler endpoint** URL
5.  Under **Configuration**, add any contracts you want to sponsor to the allowlist

Add the endpoint to your environment:

```
CDP_PAYMASTER_URL=https://api.developer.coinbase.com/rpc/v1/base-sepolia/YOUR_API_KEY

```

## Step 2: Install Dependencies

### New Wagmi Project

Create a new Wagmi project and override the Base Account SDK version:

```
# Create new project
npm create wagmi@latest
cd your-project
# Override Base Account SDK to latest
npm pkg set overrides.@base-org/account="latest"
# Install dependencies
npm install

```

### Existing Project

Add the required packages and override:

```
# Add override for Base Account SDK
npm pkg set overrides.@base-org/account="latest"
# Install dependencies
npm install viem wagmi @tanstack/react-query

```

## Step 3: Configure Wagmi with Base Account

Create your Wagmi configuration using the `baseAccount` connector:

```
import { cookieStorage, createConfig, createStorage, http } from "wagmi";
import { base, baseSepolia } from "wagmi/chains";
import { baseAccount } from "wagmi/connectors";
export function getConfig() {
  return createConfig({
    chains: [base, baseSepolia],
    multiInjectedProviderDiscovery: false,
    connectors: [
      baseAccount({
        appName: "My Wagmi App",
      }),
    ],
    storage: createStorage({
      storage: cookieStorage,
    }),
    ssr: true,
    transports: {
      [base.id]: http(),
      [baseSepolia.id]: http(),
    },
  });
}
declare module "wagmi" {
  interface Register {
    config: ReturnType<typeof getConfig>;
  }
}

```

## Step 4: Set Up Providers

Wrap your application with the Wagmi and QueryClient providers:

```
'use client'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { type ReactNode, useState } from 'react'
import { type State, WagmiProvider } from 'wagmi'
import { getConfig } from './wagmi'
export function Providers(props: {
  children: ReactNode
  initialState?: State
}) {
  const [config] = useState(() => getConfig())
  const [queryClient] = useState(() => new QueryClient())
  return (
    <WagmiProvider config={config} initialState={props.initialState}>
      <QueryClientProvider client={queryClient}>
        {props.children}
      </QueryClientProvider>
    </WagmiProvider>
  )
}

```

Use Wagmi’s `useWriteContracts` hook with paymaster capabilities to execute gasless transactions.

### Check Paymaster Support

First, check if the connected wallet supports paymaster services:

usePaymasterCapabilities.ts

```
import { useMemo } from 'react';
import { useAccount } from 'wagmi';
import { useCapabilities } from 'wagmi/experimental';
export function usePaymasterCapabilities() {
  const { address, chainId } = useAccount();
  
  const { data: availableCapabilities } = useCapabilities({
    account: address,
  });
  const capabilities = useMemo(() => {
    if (!availableCapabilities || !chainId) return {};
    
    const chainCapabilities = availableCapabilities[chainId];
    if (chainCapabilities?.paymasterService?.supported) {
      return {
        paymasterService: {
          // Use your proxy URL in production
          url: process.env.NEXT_PUBLIC_PAYMASTER_PROXY_URL,
        },
      };
    }
    return {};
  }, [availableCapabilities, chainId]);
  const isPaymasterSupported = Object.keys(capabilities).length > 0;
  return { capabilities, isPaymasterSupported };
}

```

Use the capabilities with `useWriteContracts` to send gasless transactions:

```
'use client';
import { useAccount, useConnect, useDisconnect } from 'wagmi';
import { useState } from 'react';
import { useWriteContracts } from 'wagmi/experimental';
import { usePaymasterCapabilities } from './usePaymasterCapabilities';
// Your contract details
const NFT_CONTRACT = '0xYourContractAddress';
const NFT_ABI = [
  {
    name: 'mintTo',
    type: 'function',
    stateMutability: 'nonpayable',
    inputs: [{ name: 'to', type: 'address' }],
    outputs: [],
  },
] as const;
export default function MintPage() {
  const { address, isConnected } = useAccount();
  const { connect, connectors } = useConnect();
  const { disconnect } = useDisconnect();
  const [status, setStatus] = useState<'idle' | 'minting' | 'success' | 'error'>('idle');
  
  const { capabilities, isPaymasterSupported } = usePaymasterCapabilities();
  const { writeContracts } = useWriteContracts({
    mutation: {
      onSuccess: () => setStatus('success'),
      onError: () => setStatus('error'),
    },
  });
  const handleMint = async () => {
    if (!address) return;
    
    setStatus('minting');
    try {
      writeContracts({
        contracts: [
          {
            address: NFT_CONTRACT,
            abi: NFT_ABI,
            functionName: 'mintTo',
            args: [address],
          },
        ],
        capabilities, // Includes paymaster if supported
      });
    } catch (error) {
      console.error('Minting failed:', error);
      setStatus('error');
    }
  };
  // Find the Base Account connector
  const baseAccountConnector = connectors.find(c => c.name === 'Base Account');
  if (!isConnected) {
    return (
      <button onClick={() => baseAccountConnector && connect({ connector: baseAccountConnector })}>
        Connect with Base Account
      </button>
    );
  }
  return (
    <div>
      <p>Connected: {address}</p>
      {isPaymasterSupported && <p>✓ Gas sponsorship available</p>}
      
      <button onClick={handleMint} disabled={status === 'minting'}>
        {status === 'minting' ? 'Minting...' : 'Mint NFT (Gasless)'}
      </button>
      
      {status === 'success' && <p>Mint successful!</p>}
      {status === 'error' && <p>Mint failed. Check console for details.</p>}
      
      <button onClick={() => disconnect()}>Disconnect</button>
    </div>
  );
}

```

## Batch Multiple Calls

You can batch multiple contract calls into a single sponsored transaction:

```
writeContracts({
  contracts: [
    {
      address: NFT_CONTRACT,
      abi: NFT_ABI,
      functionName: 'mintTo',
      args: [address],
    },
    {
      address: TOKEN_CONTRACT,
      abi: TOKEN_ABI,
      functionName: 'transfer',
      args: [recipientAddress, amount],
    },
  ],
  capabilities,
});

```

All calls execute atomically — if any call fails, the entire transaction reverts.

## Production Considerations

### Use a Paymaster Proxy

Never expose your Paymaster URL in client-side code. Create a backend proxy:

```
import { NextRequest, NextResponse } from "next/server";
export async function POST(request: NextRequest) {
  const body = await request.json();
  
  const response = await fetch(process.env.CDP_PAYMASTER_URL!, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });
  
  return NextResponse.json(await response.json());
}

```

Then use your proxy URL in the capabilities:

```
paymasterService: {
  url: '/api/paymaster',
}

```

See the [Paymaster Proxy Guide](https://developer.chrome.com/paymaster/guides/paymaster-proxy) for more details.

## Resources

-   [Base Account Wagmi Template](https://github.com/base/demos/tree/master/base-account/base-account-wagmi-template) — Complete example project
-   [Wagmi Experimental Hooks](https://wagmi.sh/react/api/hooks/useCallsStatus) — Documentation for `useWriteContracts` and `useCapabilities`
-   [Base Account SDK](https://docs.base.org/base-account) — Full SDK documentation

## Troubleshooting

If you run into errors, check:

-   [Paymaster Errors](https://developer.chrome.com/paymaster/reference-troubleshooting/errors) — Common error codes and solutions
-   [Troubleshooting Guide](https://developer.chrome.com/paymaster/reference-troubleshooting/troubleshooting) — Debugging tips for userOperations