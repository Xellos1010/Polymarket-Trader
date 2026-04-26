# createspendpermission

```
function createSpendPermission(options: CreateSpendPermissionOptions): Promise<CreateSpendPermissionResult>;

```

Creates a spend permission for the user’s EVM Smart Account.

## Parameters

Parameter

Type

Description

`options`

[`CreateSpendPermissionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/CreateSpendPermissionOptions)

The options for creating the spend permission.

## Returns

`Promise`<[`CreateSpendPermissionResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/CreateSpendPermissionResult)\> The result of the spend permission creation.

## Example

```
const result = await createSpendPermission({
  network: "base-sepolia",
  spender: "0x5678...",
  token: "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE", // ETH
  allowance: "1000000000000000000", // 1 ETH in wei
  period: 86400, // 24 hours
  start: Math.floor(Date.now() / 1000),
  end: Math.floor(Date.now() / 1000) + 86400 * 30, // 30 days from now
  useCdpPaymaster: true
});

```