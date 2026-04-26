# revokespendpermissionoptions

```
type RevokeSpendPermissionOptions = Omit<CoreRevokeSpendPermissionOptions, "evmSmartAccount"> & {
  evmSmartAccount?: EvmAddress;
};

```

Options for the useRevokeSpendPermission hook.

## Type declaration

### evmSmartAccount?

```
optional evmSmartAccount: EvmAddress;

```

## Param

The EVM Smart Account to revoke the spend permission for.

## Param

The network to revoke the spend permission on.

## Param

The hash of the spend permission to revoke.

## Param

Whether to use the CDP Paymaster for the user operation.

## Param

The paymaster URL for the spend permission.