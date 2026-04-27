# usecreatespendpermissionreturntype

```
type UseCreateSpendPermissionReturnType = {
  createSpendPermission: (options: CreateSpendPermissionOptions) => Promise<CreateSpendPermissionResult>;
  data:   | GetUserOperationResult
     | undefined;
  error: Error | undefined;
  status: Status;
};

```

Return type for the useCreateSpendPermission hook.

## Param

The function to create a spend permission.

## Properties

Property

Type

`createSpendPermission`

(`options`: [`CreateSpendPermissionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/CreateSpendPermissionOptions)) => `Promise`<`CreateSpendPermissionResult`\>

`data`

| [`GetUserOperationResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/GetUserOperationResult) | `undefined`

`error`

`Error` | `undefined`

`status`

[`Status`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/Status)