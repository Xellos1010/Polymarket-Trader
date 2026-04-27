# aave lending

DeFiYieldEmbedded Wallet

Integrate [Aave V3](https://aave.com/) lending into your app using CDP Embedded Wallets. Users can deposit tokens to earn yield, all with familiar sign-in methods and optional gasless transactions.

## Why CDP for DeFi

Feature

Benefit

**Enterprise-grade**

Same infrastructure powering DeFi at massive scale in the Coinbase app

**Gasless transactions**

[Smart accounts](https://developer.chrome.com/embedded-wallets/evm-features/smart-accounts) + [CDP Paymaster](https://developer.chrome.com/paymaster/introduction/welcome) on supported networks — users don’t need ETH

**Built-in onramp**

Users [buy crypto](https://developer.chrome.com/embedded-wallets/onramp/cross-platform) directly in your app

**Familiar auth**

Email, SMS, Google, Apple, X, Custom — no seed phrases

## Integration options

There are two ways to integrate Aave:

-   **Supply directly**: Users deposit tokens into Aave pools and earn interest
-   **Managed vaults**: Deploy ERC-4626 vaults that hold [aTokens](https://aave.com/docs/aave-v3/smart-contracts/tokenization#atoken), allowing you to manage deposits and earn fees on yield

This guide demonstrates both approaches on Base Sepolia. For production, use Base mainnet or other networks where both Aave V3 and [CDP smart accounts](https://developer.chrome.com/embedded-wallets/evm-features/smart-accounts) are supported.

## Setup

### Install packages

```
npm install @aave/react@latest @coinbase/cdp-core @coinbase/cdp-hooks

```

### Configure providers

```
import { CDPHooksProvider } from "@coinbase/cdp-hooks";
import { AaveProvider, AaveClient } from "@aave/react";
// Configure CDP Embedded Wallets
const cdpConfig = {
  projectId: "your-project-id",
  ethereum: {
    // Use "smart" for gasless transactions via CDP Paymaster
    // Use "eoa" for standard EOA wallets
    createOnLogin: "smart"
  }
};
// Initialize Aave SDK client
const aaveClient = AaveClient.create();
export function App() {
  return (
    // Wrap your app with both providers
    <CDPHooksProvider config={cdpConfig}>
      <AaveProvider client={aaveClient}>
        <YourApp />
      </AaveProvider>
    </CDPHooksProvider>
  );
}

```

## Supply directly to Aave

The simplest integration — users deposit tokens and earn interest immediately.

```
import { useState } from "react";
import { useSupply, bigDecimal } from "@aave/react";
import { useSendEvmTransaction, useEvmAddress } from "@coinbase/cdp-hooks";
// Aave V3 Pool address on Base Sepolia testnet
const AAVE_POOL = "0x8bAB6d1b75f19e9eD9fCe8b9BD338844fF79aE27";
export function SupplyToAave() {
  // Get the user's wallet address from CDP
  const { evmAddress } = useEvmAddress();
  // Hook to send transactions via CDP Embedded Wallet
  const { sendEvmTransaction } = useSendEvmTransaction();
  // Aave SDK hook to prepare supply transactions
  const [supply] = useSupply();
  const [amount, setAmount] = useState("0.01");
  // Helper to convert Aave SDK transaction data to CDP format and execute
  const executeTransaction = (tx: { to: string; data: string; value: string; chainId: number }) =>
    sendEvmTransaction({
      transaction: { to: tx.to, data: tx.data, value: BigInt(tx.value), chainId: tx.chainId },
      evmAccount: evmAddress!,
      network: "base-sepolia",
    });
  const handleSupply = async () => {
    if (!evmAddress) return;
    // Prepare the supply transaction via Aave SDK
    const prepareResult = await supply({
      market: AAVE_POOL,
      amount: { native: bigDecimal(parseFloat(amount)) },
      sender: evmAddress,
      chainId: 84532 // Base Sepolia
    });
    if (prepareResult.isErr()) {
      console.error("Supply failed:", prepareResult.error);
      return;
    }
    const preparedTx = prepareResult.value;
    // Aave SDK returns different types based on token allowance
    if (preparedTx.__typename === "ApprovalRequired") {
      // Token needs approval first, then supply
      await executeTransaction(preparedTx.approval);
      await executeTransaction(preparedTx.originalTransaction);
    } else if (preparedTx.__typename === "TransactionRequest") {
      // Token already approved, supply directly
      await executeTransaction(preparedTx);
    }
  };
  return (
    <div>
      <input value={amount} onChange={(e) => setAmount(e.target.value)} />
      <button onClick={handleSupply}>Supply {amount} ETH</button>
    </div>
  );
}

```

### Withdraw from Aave

```
import { useState } from "react";
import { useWithdraw, bigDecimal } from "@aave/react";
import { useSendEvmTransaction, useEvmAddress } from "@coinbase/cdp-hooks";
// Aave V3 Pool address on Base Sepolia testnet
const AAVE_POOL = "0x8bAB6d1b75f19e9eD9fCe8b9BD338844fF79aE27";
export function WithdrawFromAave() {
  const { evmAddress } = useEvmAddress();
  const { sendEvmTransaction } = useSendEvmTransaction();
  // Aave SDK hook to prepare withdraw transactions
  const [withdraw] = useWithdraw();
  const [amount, setAmount] = useState("0.01");
  // Helper to convert Aave SDK transaction data to CDP format and execute
  const executeTransaction = (tx: { to: string; data: string; value: string; chainId: number }) =>
    sendEvmTransaction({
      transaction: { to: tx.to, data: tx.data, value: BigInt(tx.value), chainId: tx.chainId },
      evmAccount: evmAddress!,
      network: "base-sepolia",
    });
  const handleWithdraw = async () => {
    if (!evmAddress) return;
    // Prepare the withdraw transaction via Aave SDK
    const prepareResult = await withdraw({
      market: AAVE_POOL,
      amount: { native: bigDecimal(parseFloat(amount)) },
      sender: evmAddress,
      chainId: 84532 // Base Sepolia
    });
    if (prepareResult.isErr()) {
      console.error("Withdraw failed:", prepareResult.error);
      return;
    }
    const preparedTx = prepareResult.value;
    // Withdraw typically doesn't require approval (you own the aTokens)
    if (preparedTx.__typename === "TransactionRequest") {
      await executeTransaction(preparedTx);
    }
  };
  return (
    <div>
      <input value={amount} onChange={(e) => setAmount(e.target.value)} />
      <button onClick={handleWithdraw}>Withdraw {amount} ETH</button>
    </div>
  );
}

```

## Create a managed vault

Deploy an ERC-4626 vault to manage user deposits and earn fees on yield generated.

```
import { useAaveReserve, useVaultDeploy, bigDecimal } from "@aave/react";
import { useSendEvmTransaction, useEvmAddress } from "@coinbase/cdp-hooks";
// Contract addresses for Base Sepolia
const AAVE_POOL = "0x8bAB6d1b75f19e9eD9fCe8b9BD338844fF79aE27";
const WETH = "0x4200000000000000000000000000000000000006";
export function DeployVault() {
  const { evmAddress } = useEvmAddress();
  const { sendEvmTransaction } = useSendEvmTransaction();
  // Aave SDK hook to deploy ERC-4626 vaults
  const [deployVault] = useVaultDeploy();
  // Fetch reserve data for the underlying token (needed for vault config)
  const { data: reserve } = useAaveReserve({
    market: AAVE_POOL,
    underlyingToken: WETH,
    chainId: 84532,
    suspense: true
  });
  // Helper to convert Aave SDK transaction data to CDP format and execute
  const executeTransaction = (tx: { to: string; data: string; value: string; chainId: number }) =>
    sendEvmTransaction({
      transaction: { to: tx.to, data: tx.data, value: BigInt(tx.value), chainId: tx.chainId },
      evmAccount: evmAddress!,
      network: "base-sepolia",
    });
  const handleDeploy = async () => {
    if (!evmAddress || !reserve) return;
    // Configure and deploy the vault
    const prepareResult = await deployVault({
      market: reserve.market.address,
      chainId: 84532,
      underlyingToken: reserve.underlyingToken.address,
      deployer: evmAddress,
      initialFee: bigDecimal(3),           // 3% fee on yield earned
      shareName: "My WETH Vault",          // ERC-20 name for vault shares
      shareSymbol: "myWETH",               // ERC-20 symbol for vault shares
      initialLockDeposit: bigDecimal(0.01) // Required initial deposit
    });
    if (prepareResult.isErr()) {
      console.error("Deploy failed:", prepareResult.error);
      return;
    }
    const preparedTx = prepareResult.value;
    // Handle approval + deploy, or direct deploy
    if (preparedTx.__typename === "ApprovalRequired") {
      await executeTransaction(preparedTx.approval);
      await executeTransaction(preparedTx.originalTransaction);
    } else if (preparedTx.__typename === "TransactionRequest") {
      await executeTransaction(preparedTx);
    }
  };
  return <button onClick={handleDeploy}>Deploy Vault</button>;
}

```

### Deposit into a vault

```
import { useVault, useVaultDeposit, bigDecimal } from "@aave/react";
import { useSendEvmTransaction, useEvmAddress } from "@coinbase/cdp-hooks";
export function DepositToVault({ vaultAddress }: { vaultAddress: string }) {
  const { evmAddress } = useEvmAddress();
  const { sendEvmTransaction } = useSendEvmTransaction();
  // Aave SDK hook to deposit into existing vaults
  const [deposit] = useVaultDeposit();
  // Fetch vault details (underlying token, reserve info, etc.)
  const { data: vault } = useVault({
    by: { address: vaultAddress },
    chainId: 84532
  });
  // Helper to convert Aave SDK transaction data to CDP format and execute
  const executeTransaction = (tx: { to: string; data: string; value: string; chainId: number }) =>
    sendEvmTransaction({
      transaction: { to: tx.to, data: tx.data, value: BigInt(tx.value), chainId: tx.chainId },
      evmAccount: evmAddress!,
      network: "base-sepolia",
    });
  const handleDeposit = async () => {
    if (!evmAddress || !vault) return;
    // Prepare deposit transaction
    const prepareResult = await deposit({
      chainId: vault.chainId,
      vault: vault.address,
      amount: {
        currency: vault.usedReserve.underlyingToken.address,
        value: bigDecimal(0.1) // Amount of underlying token to deposit
      },
      depositor: evmAddress
    });
    if (prepareResult.isErr()) {
      console.error("Deposit failed:", prepareResult.error);
      return;
    }
    const preparedTx = prepareResult.value;
    // Handle approval + deposit, or direct deposit
    if (preparedTx.__typename === "ApprovalRequired") {
      await executeTransaction(preparedTx.approval);
      await executeTransaction(preparedTx.originalTransaction);
    } else if (preparedTx.__typename === "TransactionRequest") {
      await executeTransaction(preparedTx);
    }
  };
  return <button onClick={handleDeposit}>Deposit 0.1 WETH</button>;
}

```

## Add onramp (optional)

Let users buy crypto before supplying to Aave:

```
import { Fund } from "@coinbase/cdp-react";
// One-click fiat-to-crypto onramp — users can buy ETH with card/bank
<Fund asset="ETH" network="base" onSuccess={() => console.log("Funded!")} />

```

## Key concepts

Concept

Description

**[aTokens](https://aave.com/docs/aave-v3/smart-contracts/tokenization#atoken)**

Interest-bearing tokens representing your deposit. Balance grows automatically as yield accrues.

**Vaults**

ERC-4626 wrappers that let apps manage deposits on behalf of users and earn fees on yield.

## Production tips

-   For mainnet, replace Base Sepolia addresses with Base mainnet Pool and token addresses
-   Query `getReserveData()` to display current APY to users
-   Add error handling for failed transactions and insufficient balances

## Resources