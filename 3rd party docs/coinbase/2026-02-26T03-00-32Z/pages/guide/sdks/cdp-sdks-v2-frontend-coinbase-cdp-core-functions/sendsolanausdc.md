# sendsolanausdc

```
function sendSolanaUsdc(options: SendSolanaUsdcOptions): Promise<SendSolanaUsdcResult>;

```

Sends USDC on the Solana network.

## Parameters

Parameter

Type

Description

`options`

[`SendSolanaUsdcOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SendSolanaUsdcOptions)

The options for sending USDC.

## Returns

`Promise`<[`SendSolanaUsdcResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SendSolanaUsdcResult)\> The result of sending USDC.

## Example

```
const user = await getCurrentUser();
const solanaAccount = user?.solanaAccountObjects[0]?.address;
// Send to recipient with existing USDC account (default behavior)
const result = await sendSolanaUsdc({
  solanaAccount,
  to: "ExXhNkgYf6efh7YyqDRVxPZuzafobao1A74drUdp8trd",
  amount: "10.00",
  network: "solana-devnet",
});
// Or explicitly create recipient's ATA if needed (~0.002 SOL cost)
const resultWithAta = await sendSolanaUsdc({
  solanaAccount,
  to: "ExXhNkgYf6efh7YyqDRVxPZuzafobao1A74drUdp8trd",
  amount: "10.00",
  network: "solana-devnet",
  createRecipientAta: true, // Sender pays for recipient's ATA creation
});
console.log("Transaction Signature:", result.transactionSignature);

```