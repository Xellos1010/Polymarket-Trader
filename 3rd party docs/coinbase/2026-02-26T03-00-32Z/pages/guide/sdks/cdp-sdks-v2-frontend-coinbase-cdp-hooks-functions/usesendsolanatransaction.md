# usesendsolanatransaction

```
function useSendSolanaTransaction(): {
  sendSolanaTransaction: (options: SendSolanaTransactionOptions) => Promise<SendSolanaTransactionResult>;
};

```

Hook that provides a wrapped function to send Solana transactions with authentication checks. This hook uses useEnforceAuthenticated to ensure the user is signed in before attempting to send.

## Returns

```
{
  sendSolanaTransaction: (options: SendSolanaTransactionOptions) => Promise<SendSolanaTransactionResult>;
}

```

### sendSolanaTransaction()

```
sendSolanaTransaction: (options: SendSolanaTransactionOptions) => Promise<SendSolanaTransactionResult>;

```

#### Parameters

#### Returns

`Promise`<[`SendSolanaTransactionResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/SendSolanaTransactionResult)\>

## Example

```
import { useSendSolanaTransaction, useSolanaAddress } from "@coinbase/cdp-hooks";
function MyComponent() {
  const { sendSolanaTransaction } = useSendSolanaTransaction();
  const { data: solanaAddress } = useSolanaAddress();
  const handleSend = async () => {
    if (solanaAddress) {
      const result = await sendSolanaTransaction({
        solanaAccount: solanaAddress,
        network: "solana-devnet",
        transaction: "base64EncodedTransaction..."
      });
      console.log("Transaction signature:", result.transactionSignature);
    }
  };
  return <button onClick={handleSend}>Send Transaction</button>;
}

```