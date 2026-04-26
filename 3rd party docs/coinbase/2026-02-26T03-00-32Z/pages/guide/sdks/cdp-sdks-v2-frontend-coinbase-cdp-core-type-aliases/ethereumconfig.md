# ethereumconfig

```
type EthereumConfig = 
  | {
  createOnLogin?: "smart";
  enableSpendPermissions?: boolean;
}
  | {
  createOnLogin?: "eoa";
};

```

The configuration for the Ethereum account.

## Type declaration

```
{
  createOnLogin?: "smart";
  enableSpendPermissions?: boolean;
}

```

### createOnLogin?

```
optional createOnLogin: "smart";

```

### enableSpendPermissions?

```
optional enableSpendPermissions: boolean;

```

Whether to enable spend permissions for the smart account. Setting this to true is necessary in order to create spend permissions. Only new accounts can be created with spend permissions enabled. Defaults to false.

```
{
  createOnLogin?: "eoa";
}

```

### createOnLogin?

```
optional createOnLogin: "eoa";

```