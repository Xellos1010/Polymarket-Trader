# createspendpermissionoptions

```
type CreateSpendPermissionOptions = Omit<SpendPermissionInput, "account"> & {
  evmSmartAccount: EvmAddress;
  network: SpendPermissionNetwork;
  useCdpPaymaster?: boolean;
  paymasterUrl?: string;
};

```

Input for creating a spend permission. The permission owner is automatically detected from the current user.

## Type declaration

### evmSmartAccount

```
evmSmartAccount: EvmAddress;

```

The EVM Smart Account to create the spend permission for.

### network

```
network: SpendPermissionNetwork;

```

The network to create the spend permission on.

### useCdpPaymaster?

```
optional useCdpPaymaster: boolean;

```

Whether to use the CDP Paymaster for the user operation. Only available on Base. Enabled by default on Base Sepolia.

### paymasterUrl?

```
optional paymasterUrl: string;

```

The URL of the paymaster to use for the user operation.