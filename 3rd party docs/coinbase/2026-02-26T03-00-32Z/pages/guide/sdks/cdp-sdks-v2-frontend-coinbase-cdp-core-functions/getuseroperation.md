# getuseroperation

```
function getUserOperation(options: GetUserOperationOptions): Promise<EvmUserOperation>;

```

Gets a user operation by its hash.

## 

[​](#parameters)

Parameters

Parameter

Type

Description

`options`

[`GetUserOperationOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/GetUserOperationOptions)

The options for getting the user operation.

## 

[​](#returns)

Returns

`Promise`<`EvmUserOperation`\> The user operation details.

## 

[​](#example)

Example

```
const result = await getUserOperation({
  userOperationHash: "0x123...",
  evmSmartAccount: "0xabc...",
  network: "base-sepolia"
});
console.log("User Operation Status:", result.transactionHash);

```