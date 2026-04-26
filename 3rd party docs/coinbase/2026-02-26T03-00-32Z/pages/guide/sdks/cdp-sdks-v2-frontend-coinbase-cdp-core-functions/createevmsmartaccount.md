# createevmsmartaccount

```
function createEvmSmartAccount(options?: CreateEvmSmartAccountOptions): Promise<`0x${string}`>;

```

Creates an EVM Smart Account for the current user. Multiple EVM Smart Accounts can be created per user. If no owner is provided, a new EVM EOA account will be created automatically as the owner.

## Parameters

Parameter

Type

Description

`options?`

[`CreateEvmSmartAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/CreateEvmSmartAccountOptions)

Options for creating the EVM Smart Account.

## Returns

`Promise`<`` `0x${string}` ``\> The address of the newly created EVM Smart Account.

## Throws

Error if the user is not signed in.

## Throws

Error if the provided owner already controls a smart account.