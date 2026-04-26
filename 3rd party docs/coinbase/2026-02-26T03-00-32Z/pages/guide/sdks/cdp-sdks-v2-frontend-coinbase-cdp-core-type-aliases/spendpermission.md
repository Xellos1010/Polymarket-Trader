# spendpermission

```
type SpendPermission = {
  account: EvmAddress;
  spender: EvmAddress;
  token: EvmAddress;
  allowance: bigint;
  period: number;
  start: number;
  end: number;
  salt: bigint;
  extraData: Hex;
};

```

A spend permission.

## Properties

Property

Type

Description

`account`

[`EvmAddress`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/EvmAddress)

The account address that owns the tokens

`spender`

[`EvmAddress`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/EvmAddress)

Entity that can spend account’s tokens. Can be either a Smart Account or an EOA.

`token`

[`EvmAddress`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/EvmAddress)

The token address.

`allowance`

`bigint`

The allowance for the spend permission.

`period`

`number`

The period in seconds for the spend permission.

`start`

`number`

The start timestamp for the spend permission.

`end`

`number`

The end timestamp for the spend permission.

`salt`

`bigint`

The salt for the spend permission.

`extraData`

[`Hex`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/Hex)

The extra data for the spend permission.