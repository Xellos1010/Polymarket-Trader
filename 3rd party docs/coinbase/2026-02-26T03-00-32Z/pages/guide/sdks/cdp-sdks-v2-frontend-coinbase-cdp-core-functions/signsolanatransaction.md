# signsolanatransaction

```
function signSolanaTransaction(options: SignSolanaTransactionOptions): Promise<SignSolanaTransactionResult>;

```

Signs a Solana transaction.

## Parameters

Parameter

Type

Description

`options`

[`SignSolanaTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SignSolanaTransactionOptions)

The options for the signing.

## Returns

`Promise`<[`SignSolanaTransactionResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SignSolanaTransactionResult)\> The result of the signing.

## Example

```
const user = await getCurrentUser();
const solanaAccount = user?.solanaAccountObjects[0]?.address;
const result = await signSolanaTransaction({
  solanaAccount,
  transaction: "base64-encoded-transaction"
});

```