# sending transactions

## Overview

This guide shows how to send Solana transactions using CDP React Hooks or the React Component. Sending a Solana transaction is a two-step process.

## 1\. Build a transaction

Use `@solana/web3.js` to construct Solana transactions. The example below creates a `createAndEncodeTransaction` helper function that builds a simple transfer transaction and returns it as a base64-encoded string:

```
import { PublicKey, Transaction, SystemProgram, SYSVAR_RECENT_BLOCKHASHES_PUBKEY } from "@solana/web3.js";
import { Buffer } from "buffer";
function createAndEncodeTransaction(fromAddress: string, toAddress: string, lamports: number) {
  const fromPubkey = new PublicKey(fromAddress);
  const toPubkey = new PublicKey(toAddress);
  const transaction = new Transaction().add(
    SystemProgram.transfer({
      fromPubkey,
      toPubkey,
      lamports, // 1 lamport = 0.000000001 SOL
    }),
  );
  // Set required fields
  transaction.recentBlockhash = SYSVAR_RECENT_BLOCKHASHES_PUBKEY.toBase58();
  transaction.feePayer = fromPubkey;
  // Serialize and encode to base64
  const serialized = transaction.serialize({
    requireAllSignatures: false,
  });
  return Buffer.from(serialized).toString("base64");
}

```

## 2\. Send a transaction

Once you’ve built your transaction, you can send it using either React hooks or the React component.

### Option A: Using React hooks

Use `useSendSolanaTransaction` for programmatic control. This hook signs the transaction with the user’s embedded wallet and broadcasts it to the Solana network, returning the transaction signature. The example below uses the `createAndEncodeTransaction` helper from Step 1 to build the transaction:

sendTransactionUsingHooks.tsx

```
import { useSendSolanaTransaction, useSolanaAddress } from "@coinbase/cdp-hooks";
function SendTransaction() {
  const { sendSolanaTransaction } = useSendSolanaTransaction();
  const { solanaAddress } = useSolanaAddress();
  const handleSend = async () => {
    if (!solanaAddress) return;
    try {
      // Build the transaction using the helper from step 1
      const transaction = createAndEncodeTransaction(
        solanaAddress,
        "recipient-address",
        1 // Send 1 lamport
      );
      const result = await sendSolanaTransaction({
        transaction,
        solanaAccount: solanaAddress,
        network: "solana-devnet",
      });
      console.log("Transaction signature:", result.transactionSignature);
    } catch (error) {
      console.error("Transaction failed:", error);
    }
  };
  return <button onClick={handleSend}>Send Transaction</button>;
}

```

### Option B: Using React component

Use `SendSolanaTransactionButton` for a pre-built UI button. This component handles the entire flow (signing and broadcasting) with built-in loading states and error handling. The example below uses the `createAndEncodeTransaction` helper from Step 1 to build the transaction. It uses `useMemo` to cache the transaction so it’s only rebuilt when the Solana address changes, improving performance:

sendTransactionUsingComponent.tsx

```
import { useMemo } from "react";
import { useSolanaAddress } from "@coinbase/cdp-hooks";
import { SendSolanaTransactionButton } from "@coinbase/cdp-react";
function SendTransaction() {
  const { solanaAddress } = useSolanaAddress();
  // Build the transaction using the helper from step 1
  const transaction = useMemo(() => {
    if (!solanaAddress) return "";
    return createAndEncodeTransaction(solanaAddress, "recipient-address", 1);
  }, [solanaAddress]);
  if (!solanaAddress) {
    return <p>Wallet not ready yet...</p>;
  }
  return (
    <div>
      <h2>Send Transaction</h2>
      <SendSolanaTransactionButton
        account={solanaAddress}
        network="solana-devnet"
        transaction={transaction}
      />
    </div>
  );
}

```

## What to read next

-   **[Signing](https://developer.chrome.com/embedded-wallets/solana-features/signing)**: Learn how to sign Solana messages and transactions without broadcasting
-   **[React Hooks](https://developer.chrome.com/embedded-wallets/react-hooks)**: Explore all available CDP hooks for embedded wallets
-   **[Quickstart](https://developer.chrome.com/embedded-wallets/quickstart)**: Get started with embedded wallets in under 10 minutes