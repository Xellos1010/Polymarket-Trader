# senduseroperation

```
function sendUserOperation(options: SendUserOperationOptions): Promise<SendUserOperationResult>;

```

Sends a user operation from a smart account.

## Parameters

Parameter

Type

Description

`options`

[`SendUserOperationOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SendUserOperationOptions)

The options for sending the user operation.

## Returns

`Promise`<[`SendUserOperationResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SendUserOperationResult)\> Promise that resolves to the user operation hash.

## Example

```
const user = await getCurrentUser();
const smartAccount = user?.evmSmartAccountObjects[0]?.address;
const result = await sendUserOperation({
  evmSmartAccount: smartAccount,
  network: "base-sepolia",
  calls: [{
    to: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    value: 0n,
    data: "0x",
  }],
  dataSuffix: "0x62617365617070070080218021802180218021802180218021", // ERC-8021 attribution
});
console.log("User Operation Hash:", result.userOperationHash);

```