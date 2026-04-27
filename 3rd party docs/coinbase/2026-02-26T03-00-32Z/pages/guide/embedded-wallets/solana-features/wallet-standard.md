# wallet standard

## Overview

[Wallet Standard](https://github.com/wallet-standard/wallet-standard) is a universal interface that allows Solana wallets to work seamlessly across different dapps without vendor lock-in. It’s the ecosystem standard for wallet integration, supported by major wallets like Phantom, Solflare, and Backpack. The [@coinbase/cdp-solana-standard-wallet](https://npmjs.com/package/@coinbase/cdp-solana-standard-wallet) package integrates CDP’s embedded Solana wallets into the Wallet Standard ecosystem, allowing you to use the Wallet Standard interface to interact with your user’s wallets. You’ll learn how to install the package, set up the integration, and use CDP wallets through the standard interface.

## Supported features

CDP Solana wallets support all standard Solana wallet features:

Feature

Method

Description

Connect

`standard:connect`

Connect to the wallet

Disconnect

`standard:disconnect`

Disconnect from the wallet

Event listener

`standard:events`

Listen for wallet events

Sign message

`solana:signMessage`

Sign arbitrary messages

Sign transaction

`solana:signTransaction`

Sign transactions for later submission

Sign and send

`solana:signAndSendTransaction`

Sign and broadcast transactions

CDP identifier

`cdp:`

Custom feature flag to identify CDP wallets

## Prerequisites

-   [Node.js 22+](https://nodejs.org/en/download) installed
-   A node package manager installed (i.e., `npm`, `pnpm`, or `yarn`)
-   A free [CDP Portal](https://portal.cdp.coinbase.com/) account
-   Basic familiarity with React and Solana

## 1\. Install packages

Add the required packages to your project using your preferred package manager.

## 2\. Get your CDP Project ID

1.  Sign in to your [CDP Portal](https://portal.cdp.coinbase.com/) account
2.  Select a project from the dropdown menu in the top-left
3.  Click the gear icon to view project settings
4.  Copy the **Project ID** for use in the next step

## 3\. Set up the CDP Provider

Use `CDPReactProvider` or `CDPHooksProvider` to manage CDP initialization and authentication context.

```
import React from 'react';
import ReactDOM from 'react-dom/client';
import { CDPReactProvider } from '@coinbase/cdp-react';
import { App } from './App';
const config = {
  projectId: "your-project-id", // Copy your Project ID here
  ethereum: {
    createOnLogin: "eoa", // Required minimum config
  },
  solana: {
    createOnLogin: true // Automatically create Solana account on login
  }
};
ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <CDPReactProvider config={config}>
      <App />
    </CDPReactProvider>
  </React.StrictMode>
);

```

## 4\. Use the CDP Solana wallet hook

Import and use the `useCdpSolanaStandardWallet` hook to access your CDP wallet. The hook uses the config from your `CDPReactProvider` or `CDPHooksProvider`.

```
import { useCdpSolanaStandardWallet } from '@coinbase/cdp-solana-standard-wallet';
function App() {
  const { ready, wallet } = useCdpSolanaStandardWallet();
  if (!ready || !wallet) {
    return <div>Wallet not ready. Please sign in.</div>;
  }
  return (
    <div>
      <h2>CDP Solana Wallet Ready!</h2>
      <p>Name: {wallet.name}</p>
      <p>Accounts: {wallet.accounts.length}</p>
      <p>Address: {wallet.accounts[0]?.address}</p>
    </div>
  );
}

```

## 5\. Sign in the user

Before using the wallet, users need to authenticate with CDP. **Option 1: Using React Components**

```
import { AuthButton } from "@coinbase/cdp-react";
<AuthButton /> // Users sign in via this button

```

**Option 2: Using React Hooks**

```
import { useSignInWithEmail, useVerifyEmailOTP } from "@coinbase/cdp-hooks";
// Implement your own sign-in flow using these hooks

```

For detailed sign-in implementation examples, see the [React Hooks user sign-in guide](https://docs.cdp.coinbase.com/embedded-wallets/react-hooks#user-sign-in). Once signed in, the wallet is automatically registered with the Wallet Standard and ready to use.

## 6\. Perform wallet operations

### Sign a message

```
import { useCdpSolanaStandardWallet } from '@coinbase/cdp-solana-standard-wallet';
import { useState } from 'react';
export function SignMessage() {
  const { ready, wallet } = useCdpSolanaStandardWallet();
  const [signature, setSignature] = useState<string | null>(null);
  const handleSignMessage = async () => {
    if (!wallet?.accounts.length) return;
    const message = new TextEncoder().encode("Hello from CDP!");
    const result = await wallet.features['solana:signMessage'].signMessage({
      account: wallet.accounts[0],
      message
    });
    if (result.length > 0) {
      const sig = Array.from(result[0].signature)
        .map(b => b.toString(16).padStart(2, '0'))
        .join('');
      setSignature(sig);
    }
  };
  if (!ready || !wallet) {
    return <div>Wallet not ready</div>;
  }
  return (
    <div>
      <button onClick={handleSignMessage}>Sign Message</button>
      {signature && <p>Signature: {signature.slice(0, 20)}...</p>}
    </div>
  );
}

```

### Sign and send a transaction

```
import { useCdpSolanaStandardWallet } from '@coinbase/cdp-solana-standard-wallet';
import { PublicKey, Transaction, SystemProgram, Connection } from '@solana/web3.js';
import { useState } from 'react';
export function SendTransaction() {
  const { ready, wallet } = useCdpSolanaStandardWallet();
  const [txSignature, setTxSignature] = useState<string | null>(null);
  const handleSendTransaction = async () => {
    if (!wallet?.accounts.length) return;
    const account = wallet.accounts[0];
    const fromPubkey = new PublicKey(account.address);
    // Create a transaction
    const transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey,
        toPubkey: new PublicKey('4C7sHifMjsgNZTLHABr7vUqYXLFiGnZhSru712MKAxJf'),
        lamports: 1000 // 0.000001 SOL
      })
    );
    // Get recent blockhash
    const connection = new Connection('https://api.devnet.solana.com');
    const { blockhash } = await connection.getLatestBlockhash();
    transaction.recentBlockhash = blockhash;
    transaction.feePayer = fromPubkey;
    // Serialize transaction
    const serializedTx = transaction.serialize({
      requireAllSignatures: false,
      verifySignatures: false
    });
    // Sign and send
    const result = await wallet.features['solana:signAndSendTransaction']
      .signAndSendTransaction({
        account,
        transaction: serializedTx,
        chain: 'solana:devnet'
      });
    if (result.length > 0) {
      const sig = Array.from(result[0].signature)
        .map(b => b.toString(16).padStart(2, '0'))
        .join('');
      setTxSignature(sig);
    }
  };
  if (!ready || !wallet) {
    return <div>Wallet not ready</div>;
  }
  return (
    <div>
      <button onClick={handleSendTransaction}>Send Transaction</button>
      {txSignature && (
        <div>
          <p>Transaction sent!</p>
          <p>Signature: {txSignature.slice(0, 20)}...</p>
        </div>
      )}
    </div>
  );
}

```

## 7\. Discover all wallets

Use `useSolanaStandardWallets` to see all available wallets in the ecosystem, including CDP, Phantom, Solflare, and others.

```
import { useSolanaStandardWallets } from '@coinbase/cdp-solana-standard-wallet';
export function WalletList() {
  const { wallets } = useSolanaStandardWallets();
  return (
    <div>
      <h3>Available Wallets ({wallets.length})</h3>
      {wallets.map((wallet, index) => (
        <div key={index}>
          <img src={wallet.icon} alt={wallet.name} width="24" height="24" />
          <span>{wallet.name}</span>
          {wallet.features['cdp:'] && <span> (CDP Wallet)</span>}
        </div>
      ))}
    </div>
  );
}

```

What's different from direct CDP integration?

The Wallet Standard integration provides:

1.  **Universal compatibility** - Works with any dapp supporting the Wallet Standard
2.  **Automatic discovery** - Your user’s CDP wallet appears alongside other installed wallets
3.  **Standard interface** - Uses the same methods as Phantom, Solflare, etc.
4.  **No vendor lock-in** - Dapps don’t need CDP-specific code
5.  **Ecosystem benefits** - Wallet adapters, aggregators, and tooling work automatically

The main difference is using standardized feature methods like `wallet.features['solana:signMessage']` instead of CDP-specific APIs.

## Example applications

Explore complete working examples:

These demo applications show real-world implementations including authentication, wallet operations, and transaction handling.

## Supported networks

CDP Solana wallets support both mainnet and devnet. Use these chain identifiers in your transactions:

## What to read next

-   **[CDP Web SDK Documentation](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend)**: Comprehensive API reference for the CDP Web SDK
-   **[React Hooks](https://developer.chrome.com/embedded-wallets/react-hooks)**: Learn about available CDP hooks
-   **[Solana Signing](https://developer.chrome.com/embedded-wallets/solana-features/signing)**: CDP methods for signing operations
-   **[Sending Solana Transactions](https://developer.chrome.com/embedded-wallets/solana-features/sending-transactions)**: CDP methods for sending transactions
-   **[Wallet Standard Documentation](https://github.com/wallet-standard/wallet-standard)**: Explore the Wallet Standard specification