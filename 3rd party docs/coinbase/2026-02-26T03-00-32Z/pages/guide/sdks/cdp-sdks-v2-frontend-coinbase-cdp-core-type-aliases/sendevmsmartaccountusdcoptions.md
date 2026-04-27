# sendevmsmartaccountusdcoptions

```
type SendEvmSmartAccountUsdcOptions = {
  evmSmartAccount: EvmAddress;
  to: EvmAddress;
  amount: string;
  network: SendEvmUsdcNetwork;
  useCdpPaymaster?: boolean;
  paymasterUrl?: string;
};

```

Request parameters for sending EVM USDC from a Smart Account.

## Properties

Property

Type

Description

`evmSmartAccount`

[`EvmAddress`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/EvmAddress)

The EVM Smart Account to send USDC from.

`to`

[`EvmAddress`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/EvmAddress)

The recipient address.

`amount`

`string`

The amount of USDC to send in human-readable format (e.g., “1.50” for 1.5 USDC).

`network`

[`SendEvmUsdcNetwork`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SendEvmUsdcNetwork)

The network to send USDC on.

`useCdpPaymaster?`

`boolean`

Whether to use CDP Paymaster to sponsor gas fees.

`paymasterUrl?`

`string`

Optional custom Paymaster URL to use for gas sponsorship.