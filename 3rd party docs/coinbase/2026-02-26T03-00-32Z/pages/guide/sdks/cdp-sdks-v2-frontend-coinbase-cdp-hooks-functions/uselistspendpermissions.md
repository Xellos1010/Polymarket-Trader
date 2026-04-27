# uselistspendpermissions

```
function useListSpendPermissions(options: UseListSpendPermissionsOptions): UseListSpendPermissionsReturnType;

```

Hook that lists spend permissions for an EVM Smart Account with refetch capability. This hook follows a query-style pattern for one-time data fetching.

## Parameters

Parameter

Type

Description

`options`

[`UseListSpendPermissionsOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/UseListSpendPermissionsOptions)

Configuration object with properties: evmSmartAccount (auto-detected if not provided), network (defaults to “base-sepolia”), pageSize (defaults to 20), pageToken (for pagination), and enabled (whether the query should run, defaults to true when account is available).

## Returns

[`UseListSpendPermissionsReturnType`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/UseListSpendPermissionsReturnType) Query result with data, loading state, error information, and refetch function

## Example

```
function SpendPermissionsList() {
  const { data, error, status, refetch } = useListSpendPermissions();
  if (status === "pending") {
    return <div>Loading spend permissions...</div>;
  }
  if (status === "error") {
    return <div>Error: {error?.message}</div>;
  }
  return (
    <div>
      <button onClick={refetch}>Refresh</button>
      <h3>Spend Permissions ({data?.spendPermissions.length})</h3>
      {data?.spendPermissions.map((permission) => (
        <div key={permission.permissionHash}>
          <p>Hash: {permission.permissionHash}</p>
          <p>Spender: {permission.permission.spender}</p>
          <p>Token: {permission.permission.token}</p>
          <p>Allowance: {permission.permission.allowance}</p>
          <p>Revoked: {permission.revoked ? "Yes" : "No"}</p>
        </div>
      ))}
    </div>
  );
}

```