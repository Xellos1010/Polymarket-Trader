# sendevmeoausdc

```
function sendEvmEoaUsdc(options: SendEvmEoaUsdcOptions): Promise<SendEvmEoaUsdcResult>;

```

Sends USDC on an EVM network from an EOA (Externally Owned Account).

## Parameters

Parameter

Type

Description

`options`

[`SendEvmEoaUsdcOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SendEvmEoaUsdcOptions)

The options for sending USDC.

## Returns

`Promise`<[`SendEvmEoaUsdcResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SendEvmEoaUsdcResult)\> The result of sending USDC.

## Example

```
const user = await getCurrentUser();
const evmAccount = user?.evmAccountObjects[0]?.address;
const result = await sendEvmEoaUsdc({
  evmAccount,
  to: "0x1234567890123456789012345678901234567890",
  amount: "25.50",
  network: "base-sepolia",
});
console.log("Transaction Hash:", result.transactionHash);

```