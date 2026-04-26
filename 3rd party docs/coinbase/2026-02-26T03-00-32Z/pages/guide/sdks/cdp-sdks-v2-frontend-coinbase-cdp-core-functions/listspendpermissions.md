# listspendpermissions

```
function listSpendPermissions(options: ListSpendPermissionsOptions): Promise<ListSpendPermissionsResult>;

```

Lists spend permissions for the user’s EVM Smart Account.

## Parameters

Parameter

Type

Description

`options`

[`ListSpendPermissionsOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/ListSpendPermissionsOptions)

The options for listing spend permissions.

## Returns

`Promise`<[`ListSpendPermissionsResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/ListSpendPermissionsResult)\> The result of the spend permissions listing.

## Example

```
const result = await listSpendPermissions({
  evmSmartAccount: "0x1234...",
  network: "base-sepolia",
  pageSize: 10
});
console.log("Found", result.spendPermissions.length, "spend permissions");
for (const permission of result.spendPermissions) {
  console.log("Permission:", permission.permissionHash, "Revoked:", permission.revoked);
}

```