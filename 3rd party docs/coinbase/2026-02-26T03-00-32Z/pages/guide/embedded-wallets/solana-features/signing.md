# signing

## Overview

Signing is a cryptographic operation that proves ownership of a Solana account without broadcasting anything to the blockchain. You can sign arbitrary data (like authentication challenges or terms of service) to prove account ownership without sending the message to the blockchain. You can also sign transactions locally without broadcasting them. This is useful when you want to batch multiple transactions, use a custom RPC endpoint, or implement advanced transaction flows before broadcasting. This guide shows how to sign both messages and transactions using CDP React Hooks.

## Sign messages

Use `useSignSolanaMessage` to sign arbitrary messages. The hook takes a base64-encoded message and returns an Ed25519 signature. This is commonly used for authentication (proving you own an account) or signing terms of service. The message is never broadcast to the blockchain.

```
import { useSignSolanaMessage, useSolanaAddress } from "@coinbase/cdp-hooks";
function SignData() {
  const { signSolanaMessage } = useSignSolanaMessage();
  const { solanaAddress } = useSolanaAddress();
  const handleSignMessage = async () => {
    if (!solanaAddress) return;
    const result = await signSolanaMessage({
      solanaAccount: solanaAddress,
      message: "base64-message"
    });
    console.log("Message signature:", result.signature);
  };
  return (
    <button onClick={handleSignMessage}>Sign Message</button>
  );
}

```

## Sign transactions (without broadcasting)

Use `useSignSolanaTransaction` to sign a transaction locally without broadcasting it. The hook takes a base64-encoded transaction and returns a signed transaction that you can broadcast yourself. This provides control over how transactions are broadcast and allows the use of a custom RPC endpoint.

```
import { useSignSolanaTransaction, useSolanaAddress } from "@coinbase/cdp-hooks";
function SignTransaction() {
  const { signSolanaTransaction } = useSignSolanaTransaction();
  const { solanaAddress } = useSolanaAddress();
  const handleSignTransaction = async () => {
    if (!solanaAddress) return;
    try {
      const result = await signSolanaTransaction({
        solanaAccount: solanaAddress,
        transaction: "base64-solana-transaction"
      });
      console.log("Signed Transaction:", result.signedTransaction);
      // You can now broadcast this signed transaction yourself.
    } catch (error) {
      console.error("Failed to sign transaction:", error);
    }
  };
  return <button onClick={handleSignTransaction}>Sign Transaction</button>;
}

```

## What to read next

-   **[Sending Transactions](https://developer.chrome.com/embedded-wallets/solana-features/sending-transactions)**: Learn how to build and send Solana transactions with hooks and components
-   **[React Hooks](https://developer.chrome.com/embedded-wallets/react-hooks)**: Explore all available CDP hooks for embedded wallets