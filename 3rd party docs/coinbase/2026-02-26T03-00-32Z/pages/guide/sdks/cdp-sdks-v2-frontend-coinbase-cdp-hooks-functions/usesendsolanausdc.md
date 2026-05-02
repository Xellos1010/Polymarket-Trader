# usesendsolanausdc

```
function useSendSolanaUsdc(): {
  sendSolanaUsdc: (options: SendSolanaUsdcOptions) => Promise<SendSolanaUsdcResult>;
};

```

Hook to send USDC on the Solana network.

## Returns

```
{
  sendSolanaUsdc: (options: SendSolanaUsdcOptions) => Promise<SendSolanaUsdcResult>;
}

```

### sendSolanaUsdc()

```
sendSolanaUsdc: (options: SendSolanaUsdcOptions) => Promise<SendSolanaUsdcResult>;

```

#### Parameters

Parameter

Type

`options`

`SendSolanaUsdcOptions`

#### Returns

`Promise`<`SendSolanaUsdcResult`\>

## Example

```
import { useSendSolanaUsdc, useSolanaAddress } from "@coinbase/cdp-hooks";
function MyComponent() {
  const { sendSolanaUsdc } = useSendSolanaUsdc();
  const { solanaAddress } = useSolanaAddress();
  const handleSendUsdc = async () => {
    if (!solanaAddress) return;
    try {
      const result = await sendSolanaUsdc({
        solanaAccount: solanaAddress,
        to: "ExXhNkgYf6efh7YyqDRVxPZuzafobao1A74drUdp8trd",
        amount: "10.00",
        network: "solana-devnet",
        createRecipientAta: true,
      });
      console.log("Transaction Signature:", result.transactionSignature);
    } catch (error) {
      console.error("Failed to send USDC:", error);
    }
  };
  return <button onClick={handleSendUsdc}>Send USDC on Solana</button>;
}

```