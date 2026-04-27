# morpho integration

DeFiYieldEmbedded Wallet

## Overview

This guide shows how to integrate a [Morpho](https://morpho.org/) vault with CDP embedded wallets so your users can deposit tokens to earn yield and withdraw them, without leaving your application. [Morpho vaults](https://docs.morpho.org/build/earn/get-started/) are ERC-4626 tokenized wrappers that automatically allocate deposits across multiple Morpho Blue lending markets. This guide uses the [Steakhouse USDC](https://app.morpho.org/vault?vault=0xbeeF010f9cb27031ad51e3333f9aF9C6B1228183&network=base) vault on Base.

### What you’ll build

-   Authenticate users and create embedded wallets with CDP hooks
-   Deposit USDC into a Morpho vault to earn yield
-   Withdraw USDC from the vault
-   Read a user’s vault position

## Prerequisites

-   A [CDP project](https://portal.cdp.coinbase.com/) with your domain configured
-   Node.js 24+

## Installation

Create a React project and install dependencies:

Install CDP packages and viem:

## Provider setup

Wrap your app with `CDPReactProvider`:

```
import { CDPReactProvider } from "@coinbase/cdp-react";
createRoot(document.getElementById("root")!).render(
  <CDPReactProvider
    config={{
      projectId: "YOUR_CDP_PROJECT_ID",
      // Use "smart" for gasless transactions via CDP Paymaster
      // Use "eoa" for standard EOA wallets
      ethereum: { createOnLogin: "smart" },
      appName: "Morpho Vault guide",
    }}
  >
    <App />
  </CDPReactProvider>
);

```

## Configuration

Define the vault and token addresses, and a public client for reading onchain data:

```
import {
  createPublicClient,
  http,
  encodeFunctionData,
  parseAbi,
  parseUnits,
  formatUnits,
  maxUint256,
  type Address,
} from "viem";
import { base } from "viem/chains";
// Steakhouse USDC vault on Base
const VAULT_ADDRESS: Address = "0xbeeF010f9cb27031ad51e3333f9aF9C6B1228183";
// USDC on Base
const USDC_ADDRESS: Address = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913";
const publicClient = createPublicClient({
  chain: base,
  transport: http(),
});

```

## Deposit

Before depositing, the user must approve the vault to spend their USDC. Then call `deposit` with the amount and receiver address.

```
import { useEvmAddress, useSendEvmTransaction } from "@coinbase/cdp-hooks";
const { evmAddress } = useEvmAddress();
const { sendEvmTransaction } = useSendEvmTransaction();
// Step 1: Approve the vault to spend USDC (one-time)
const approveData = encodeFunctionData({
  abi: parseAbi(["function approve(address spender, uint256 amount) returns (bool)"]),
  functionName: "approve",
  args: [VAULT_ADDRESS, maxUint256],
});
await sendEvmTransaction({
  evmAccount: evmAddress,
  network: "base",
  transaction: {
    to: USDC_ADDRESS,
    data: approveData,
    chainId: 8453,
  },
});
// Step 2: Deposit USDC into the vault
const amount = parseUnits("100", 6); // 100 USDC
const depositData = encodeFunctionData({
  abi: parseAbi(["function deposit(uint256 assets, address receiver) returns (uint256 shares)"]),
  functionName: "deposit",
  args: [amount, evmAddress],
});
const { transactionHash } = await sendEvmTransaction({
  evmAccount: evmAddress,
  network: "base",
  transaction: {
    to: VAULT_ADDRESS,
    data: depositData,
    chainId: 8453,
  },
});

```

## Withdraw

To withdraw, call `withdraw` with the amount of underlying USDC to receive:

```
const amount = parseUnits("10", 6); // 10 USDC
const withdrawData = encodeFunctionData({
  abi: parseAbi([
    "function withdraw(uint256 assets, address receiver, address owner) returns (uint256 shares)",
  ]),
  functionName: "withdraw",
  args: [amount, evmAddress, evmAddress],
});
const { transactionHash } = await sendEvmTransaction({
  evmAccount: evmAddress,
  network: "base",
  transaction: {
    to: VAULT_ADDRESS,
    data: withdrawData,
    chainId: 8453,
  },
});

```

To withdraw the entire position, use `redeem` with all vault shares instead:

```
const shares = await publicClient.readContract({
  address: VAULT_ADDRESS,
  abi: parseAbi(["function balanceOf(address owner) view returns (uint256)"]),
  functionName: "balanceOf",
  args: [evmAddress],
});
const redeemData = encodeFunctionData({
  abi: parseAbi([
    "function redeem(uint256 shares, address receiver, address owner) returns (uint256 assets)",
  ]),
  functionName: "redeem",
  args: [shares, evmAddress, evmAddress],
});

```

## Check position

Read the user’s vault share balance and convert it to the underlying USDC value:

```
async function getPosition(userAddress: Address) {
  const shares = await publicClient.readContract({
    address: VAULT_ADDRESS,
    abi: parseAbi(["function balanceOf(address owner) view returns (uint256)"]),
    functionName: "balanceOf",
    args: [userAddress],
  });
  const assets = await publicClient.readContract({
    address: VAULT_ADDRESS,
    abi: parseAbi(["function convertToAssets(uint256 shares) view returns (uint256)"]),
    functionName: "convertToAssets",
    args: [shares],
  });
  return {
    shares,
    assets,
    formatted: formatUnits(assets, 6),
  };
}

```

## Using other vaults

This code works with any ERC-4626 Morpho vault. To use a different vault:

1.  Replace `VAULT_ADDRESS` with the target vault’s address
2.  Replace `USDC_ADDRESS` with the vault’s underlying token
3.  Update the `parseUnits`/`formatUnits` decimals to match the token.

Browse available vaults [here](https://morpho.org/vaults/).