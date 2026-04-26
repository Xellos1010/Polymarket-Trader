# revokespendpermissionoptions

```
type RevokeSpendPermissionOptions = {
  evmSmartAccount: EvmAddress;
  network: SpendPermissionNetwork;
  permissionHash: string;
  useCdpPaymaster?: boolean;
  paymasterUrl?: string;
};

```

Input for revoking a spend permission.

## Properties

Property

Type

Description

`evmSmartAccount`

[`EvmAddress`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/EvmAddress)

The EVM Smart Account to revoke the spend permission for.

`network`

[`SpendPermissionNetwork`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SpendPermissionNetwork)

The network to revoke the spend permission on.

`permissionHash`

`string`

The hash of the spend permission to revoke.

`useCdpPaymaster?`

`boolean`

Whether to use the CDP Paymaster for the user operation. Only available on Base. Enabled by default on Base Sepolia.

`paymasterUrl?`

`string`

The URL of the paymaster to use for the user operation.