# userevokespendpermissionreturntype

```
type UseRevokeSpendPermissionReturnType = {
  revokeSpendPermission: (options: RevokeSpendPermissionOptions) => Promise<RevokeSpendPermissionResult>;
  data:   | GetUserOperationResult
     | undefined;
  error: Error | undefined;
  status: Status;
};

```

Return type for the useRevokeSpendPermission hook.

## Param

The function to revoke a spend permission.

## Properties

Property

Type

`revokeSpendPermission`

(`options`: [`RevokeSpendPermissionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/RevokeSpendPermissionOptions)) => `Promise`<`RevokeSpendPermissionResult`\>

`data`

| [`GetUserOperationResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/GetUserOperationResult) | `undefined`

`error`

`Error` | `undefined`

`status`

[`Status`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/Status)