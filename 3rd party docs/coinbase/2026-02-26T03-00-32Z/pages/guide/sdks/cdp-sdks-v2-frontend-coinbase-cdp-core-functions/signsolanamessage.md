# signsolanamessage

```
function signSolanaMessage(options: SignSolanaMessageOptions): Promise<SignSolanaMessageResult>;

```

Signs a message with a Solana account.

## Parameters

Parameter

Type

Description

`options`

[`SignSolanaMessageOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SignSolanaMessageOptions)

The options for the signing.

## Returns

`Promise`<[`SignSolanaMessageResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SignSolanaMessageResult)\> The result of the signing.

## Example

```
const user = await getCurrentUser();
const solanaAccount = user?.solanaAccountObjects[0]?.address;
const message = Buffer.from("Hello, Solana!", "utf8").toString("base64");
const result = await signSolanaMessage({
  solanaAccount,
  message // Base64 encoded message to sign
});

```