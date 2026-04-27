# smart accounts

## Overview

Smart accounts ([ERC-4337](https://eips.ethereum.org/EIPS/eip-4337)) are programmable wallets that enable advanced features like batching multiple transactions together and sponsoring gas fees for your users. Key benefits of CDP Smart Accounts include:

This guide shows how to enable smart accounts in Embedded Wallets, send user operations, and integrate a paymaster.

## Prerequisites

-   A CDP Portal account and project
-   Node.js 22+ and a package manager (npm, pnpm, or yarn)
-   Basic familiarity with React and TypeScript
-   A CDP project with Embedded Wallets enabled
-   Your app domain [allowlisted](https://developer.chrome.com/embedded-wallets/domains)
-   `@coinbase/cdp-core` and `@coinbase/cdp-hooks` installed

```
# npm
npm install @coinbase/cdp-core @coinbase/cdp-hooks

```

### Enable smart accounts

Configure the `CDPHooksProvider` to create a smart account for new users on sign in.

```
import { CDPHooksProvider } from "@coinbase/cdp-hooks";
function App() {
  return (
    <CDPHooksProvider
      config={{
        projectId: "your-project-id",
        ethereum: {
          createOnLogin: "smart", // Automatically create Smart Accounts
        }
      }}
    >
      <YourApp />
    </CDPHooksProvider>
  );
}

```

-   When `ethereum.createOnLogin` is set to `"smart"`, new users receive both an EOA and a smart account.

Other config keys supported by the SDK provider:

-   `basePath` (API base URL override)
-   `useMock` (local mock mode)
-   `debugging` (verbose logging)

## Access the user’s smart account

Use `useCurrentUser` to access the authenticated user’s accounts. The first smart account is available as `currentUser.evmSmartAccounts?.[0]`.

```
import { useCurrentUser } from "@coinbase/cdp-hooks";
function SmartAccountBadge() {
  const { currentUser } = useCurrentUser();
  const smartAccount = currentUser?.evmSmartAccounts?.[0];
  if (!currentUser) return <div>Please sign in</div>;
  return (
    <div>
      <p>Owner EOA: {currentUser.evmAccounts?.[0]}</p>
      {smartAccount && <p>Smart Account: {smartAccount}</p>}
    </div>
  );
}

```

Alternatively, use `useEvmAddress()` to get the primary EVM address; it returns the smart account if one exists, otherwise the EOA.

## Send a user operation

Smart accounts submit ERC-4337 user operations rather than standard EOA transactions. Use `useSendUserOperation` to submit one or more calls. This hook returns `status`, `data`, and `error` values which you can use to track the status of the user operation.

```
import { useSendUserOperation, useCurrentUser } from "@coinbase/cdp-hooks";
function SendUserOperation() {
  const { sendUserOperation, status, data, error } = useSendUserOperation();
  const { currentUser } = useCurrentUser();
  const handleSendUserOperation = async () => {
    const smartAccount = currentUser?.evmSmartAccounts?.[0];
    if (!smartAccount) return;
    try {
      // This will automatically start tracking the user operation status
      const result = await sendUserOperation({
        evmSmartAccount: smartAccount,
        network: "base-sepolia",
        calls: [{
          to: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
          value: 1000000000000000000n,
          data: "0x",
        }],
        // Optional gas sponsorship
        // useCdpPaymaster: true,
        // or provide your own paymasterUrl
        // paymasterUrl: "https://your-paymaster.example.com",
      });
      console.log("User Operation Hash:", result.userOperationHash);
    } catch (error) {
      console.error("Failed to send user operation:", error);
    }
  };
  return (
    <div>
      {status === "idle" && <p>Ready to send user operation</p>}
      
      {status === "pending" && (
        <div>
          <p>User operation pending...</p>
          {data && <p>User Op Hash: {data.userOpHash}</p>}
        </div>
      )}
      
      {status === "success" && data && (
        <div>
          <p>User operation successful!</p>
          <p>Transaction Hash: {data.transactionHash}</p>
          <p>Status: {data.status}</p>
        </div>
      )}
      
      {status === "error" && (
        <div>
          <p>User operation failed</p>
          <p>Error: {error?.message}</p>
        </div>
      )}
      
      <button onClick={handleSendUserOperation} disabled={status === "pending"}>
        {status === "pending" ? "Sending..." : "Send User Operation"}
      </button>
    </div>
  );
}

```

### Batch calls

Provide multiple `calls` to batch actions in a single user operation. Calls execute in order and revert atomically on failure.

```
const { userOperationHash } = await sendUserOperation({
  evmSmartAccount: smartAccount,
  network: "base-sepolia",
  calls: [
    { to: addr1, value: 100000000000000000n },
    { to: addr2, value: 50000000000000000n },
  ],
});

```

### Encode contract calls

To interact with contracts, pass `data` using an ABI-encoded payload. This example encodes an ERC-20 `transfer` using `viem`:

```
import { encodeFunctionData } from "viem";
const erc20Abi = [
  {
    name: "transfer",
    type: "function",
    stateMutability: "nonpayable",
    inputs: [
      { name: "to", type: "address" },
      { name: "value", type: "uint256" },
    ],
    outputs: [{ name: "", type: "bool" }],
  },
];
const data = encodeFunctionData({
  abi: erc20Abi as const,
  functionName: "transfer",
  args: [recipient, 1_000_000n], // 1 USDC assuming 6 decimals
});
await sendUserOperation({
  evmSmartAccount: smartAccount,
  network: "base-sepolia",
  calls: [
    {
      to: usdcAddress,
      data,
      value: 0n,
    },
  ],
});

```

## Using non-Base networks

Smart accounts can send user operations on the following networks:

Network

Mainnet

Testnet

Base

✓

✓ (Base Sepolia)

Arbitrum

✓

Optimism

✓

Zora

✓

Polygon

✓

BNB Chain

✓

Avalanche

✓

Ethereum

✓

✓ (Sepolia)

Simply specify the `network` parameter for any supported chain. For example:

Use a paymaster to sponsor gas so users can transact without holding ETH.

-   Add `paymasterUrl` to `sendUserOperation`
-   Or set `useCdpPaymaster: true` to use the CDP Paymaster

Learn more about our [Paymaster](https://developer.chrome.com/paymaster/introduction/welcome) or see the [Paymaster Quickstart](https://developer.chrome.com/paymaster/guides/quickstart).

```
await sendUserOperation({
  evmSmartAccount: smartAccount,
  network: "base-sepolia",
  calls: [{ to: recipient, value: 10n ** 15n }],
  paymasterUrl: "https://your-paymaster.example.com",
});

```

## Reference

Resource

Description

[`useCurrentUser`](https://github.com/coinbase/cdp-web/blob/d2bf487d1ba29cfbeec73123a14b104a35819368/packages/hooks/src/hooks.ts#L362)

Get the current authenticated user

[`useEvmAddress`](https://github.com/coinbase/cdp-web/blob/d2bf487d1ba29cfbeec73123a14b104a35819368/packages/hooks/src/hooks.ts#L447)

Primary EVM address (Smart Account if present, else EOA)

[`useSendUserOperation`](https://github.com/coinbase/cdp-web/blob/d2bf487d1ba29cfbeec73123a14b104a35819368/packages/hooks/src/hooks.ts#L1118)

Send a user operation; hook returns status/data

[`sendUserOperationOptions`](https://github.com/coinbase/cdp-web/blob/d2bf487d1ba29cfbeec73123a14b104a35819368/packages/core/src/types.ts#L403)

Options for sending a user operation

[`sendUserOperationResult`](https://github.com/coinbase/cdp-web/blob/d2bf487d1ba29cfbeec73123a14b104a35819368/packages/core/src/types.ts#L419)

Result fields for sendUserOperation

[`ethereum.createOnLogin`](https://github.com/coinbase/cdp-web/blob/d2bf487d1ba29cfbeec73123a14b104a35819368/packages/core/src/types.ts#L47)

Config: “eoa” or “smart”

[User type](https://github.com/coinbase/cdp-web/blob/main/packages/core/src/types.ts#L56)

`user.evmAccounts[]`, `user.evmSmartAccounts[]`

[Hooks README](https://github.com/coinbase/cdp-web/blob/main/packages/hooks/README.md)

Package overview and usage

## What to read next

-   [Spend Permissions](https://developer.chrome.com/embedded-wallets/evm-features/spend-permissions)
-   [React Hooks](https://developer.chrome.com/embedded-wallets/react-hooks)
-   [Paymaster](https://developer.chrome.com/paymaster/introduction/welcome)