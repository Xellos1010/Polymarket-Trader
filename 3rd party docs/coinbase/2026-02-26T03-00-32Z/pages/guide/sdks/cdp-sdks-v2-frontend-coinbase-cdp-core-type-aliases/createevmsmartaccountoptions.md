# createevmsmartaccountoptions

```
type CreateEvmSmartAccountOptions = {
  owner?: EvmAddress;
  enableSpendPermissions?: boolean;
};

```

Options for creating a Smart Account.

## Properties

Property

Type

Description

`owner?`

[`EvmAddress`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/EvmAddress)

Optional owner address for the smart account. If not provided, a new EOA will be created as the owner. If the owner already controls a smart account, an error will be thrown.

`enableSpendPermissions?`

`boolean`

Whether to enable spend permissions for the smart account. Setting this to true is necessary in order to create spend permissions. Only new accounts can be created with spend permissions enabled. Defaults to false.