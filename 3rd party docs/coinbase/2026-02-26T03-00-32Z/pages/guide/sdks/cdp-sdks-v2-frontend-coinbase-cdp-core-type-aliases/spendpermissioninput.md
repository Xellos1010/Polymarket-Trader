# spendpermissioninput

```
type SpendPermissionInput = Omit<SpendPermission, "token" | "period" | "start" | "end" | "salt" | "extraData"> & {
  token:   | "eth"
     | "usdc"
     | EvmAddress;
  period?: number;
  periodInDays?: number;
  start?: Date;
  end?: Date;
  salt?: bigint;
  extraData?: Hex;
};

```

Dev-friendly input for creating a spend permission.

## Type declaration

### token

```
token: 
  | "eth"
  | "usdc"
  | EvmAddress;

```

Token symbol (“eth”, “usdc”) or contract address.

### period?

Time duration for resetting used allowance on a recurring basis (seconds)

### periodInDays?

```
optional periodInDays: number;

```

Time duration for resetting used allowance on a recurring basis (days) This can be used instead of `period` to specify a human-friendly value, like `periodInDays: 7`.

### start?

The start timestamp for the spend permission. Defaults to now.

### end?

The end timestamp for the spend permission. Defaults to max uint48 (no expiration).

### salt?

The salt for the spend permission. Defaults to random salt.

The extra data for the spend permission. Defaults to “0x”.