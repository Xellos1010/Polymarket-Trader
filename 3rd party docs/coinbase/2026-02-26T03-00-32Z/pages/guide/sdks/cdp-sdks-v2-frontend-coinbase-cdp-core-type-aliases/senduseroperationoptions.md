# senduseroperationoptions

```
type SendUserOperationOptions = {
  evmSmartAccount: EvmAddress;
  network: SendEvmTransactionWithEndUserAccountBodyNetwork;
  calls: EvmCall[];
  useCdpPaymaster?: boolean;
  paymasterUrl?: string;
  dataSuffix?: string;
};

```

Request parameters for sending a user operation.

## Properties

Property

Type

Description

`evmSmartAccount`

[`EvmAddress`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/EvmAddress)

The EVM Smart Account to send the user operation with.

`network`

[`SendEvmTransactionWithEndUserAccountBodyNetwork`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SendEvmTransactionWithEndUserAccountBodyNetwork)

The network to send the user operation on.

`calls`

[`EvmCall`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/EvmCall)\[\]

The calls to make from the user operation.

`useCdpPaymaster?`

`boolean`

Whether to use the CDP Paymaster for the user operation.

`paymasterUrl?`

`string`

The URL of the paymaster to use for the user operation. Get your free Base paymaster URL [from the CDP Portal](https://portal.cdp.coinbase.com/products/node).

`dataSuffix?`

`string`

Optional hex-encoded data suffix to append to the user operation callData for attribution (ERC-8021). The suffix will be appended to the callData of the user operation.