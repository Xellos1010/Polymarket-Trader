# signevmtypeddata

```
function signEvmTypedData(options: SignEvmTypedDataOptions): Promise<SignEvmTypedDataResult>;

```

Signs EIP-712 typed data with an EVM account.

## Parameters

Parameter

Type

Description

`options`

[`SignEvmTypedDataOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SignEvmTypedDataOptions)

The options for the signing.

## Returns

`Promise`<[`SignEvmTypedDataResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SignEvmTypedDataResult)\> The result of the signing.

## Example

```
const user = await getCurrentUser();
const evmAccount = user?.evmAccountObjects[0]?.address;
const result = await signEvmTypedData({
  evmAccount,
  typedData: {
    domain: {
      name: "USDC",
      version: "2",
      chainId: 84532,
      verifyingContract: "0x036CbD53842c5426634e7929541eC2318f3dCF7e", // Base Sepolia USDC
    },
    types: {
      EIP712Domain: [
        { name: "name", type: "string" },
        { name: "version", type: "string" },
        { name: "chainId", type: "uint256" },
        { name: "verifyingContract", type: "address" },
      ],
      TransferWithAuthorization: [
        { name: "from", type: "address" },
        { name: "to", type: "address" },
        { name: "value", type: "uint256" },
        { name: "validAfter", type: "uint256" },
        { name: "validBefore", type: "uint256" },
        { name: "nonce", type: "bytes32" },
      ],
    },
    primaryType: "TransferWithAuthorization",
    message: {
      from: evmAccount,
      to: "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB",
      value: "1000000", // 1 USDC (6 decimals)
      validAfter: 0, // Valid immediately
      validBefore: 2524604400, // Valid until 2050
      nonce: 0
    },
  },
});

```