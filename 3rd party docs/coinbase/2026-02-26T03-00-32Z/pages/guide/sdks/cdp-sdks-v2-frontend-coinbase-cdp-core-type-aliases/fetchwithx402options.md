# fetchwithx402options

```
type FetchWithX402Options = {
  address?:   | EvmAddress
     | string;
  fetch?: typeof globalThis.fetch;
  maxValue?: bigint;
  paymentRequirementsSelector?: PaymentRequirementsSelector;
  config?: X402Config;
};

```

Options for the fetchWithX402 function.

## Properties

Property

Type

Description

`address?`

| [`EvmAddress`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/EvmAddress) | `string`

The address of the account to use for the fetch function.

`fetch?`

*typeof* `globalThis.fetch`

The fetch function to use for the fetch function.

`maxValue?`

`bigint`

The maximum allowed payment amount in base units (defaults to 0.1 USDC)

`paymentRequirementsSelector?`

`PaymentRequirementsSelector`

A function that selects the payment requirements from the response

`config?`

`X402Config`

Optional configuration for X402 operations (e.g., custom RPC URLs)