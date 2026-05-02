# sendusdcoptions

```
type SendUsdcOptions = 
  | {
  from: EvmAddress;
  to: EvmAddress;
  amount: string;
  network: SendEvmUsdcNetwork;
  useCdpPaymaster?: boolean;
  paymasterUrl?: string;
}
  | {
  from: SolanaAddress;
  to: SolanaAddress;
  amount: string;
  network: SendSolanaUsdcNetwork;
  createRecipientAta?: boolean;
}
  | {
  to: EvmAddress;
  amount: string;
  network: SendEvmUsdcNetwork;
  useCdpPaymaster?: boolean;
  paymasterUrl?: string;
}
  | {
  to: SolanaAddress;
  amount: string;
  network: SendSolanaUsdcNetwork;
  createRecipientAta?: boolean;
};

```

Request parameters for sending USDC. Uses discriminated unions to prevent invalid parameter combinations at compile-time.

## Type declaration

```
{
  from: EvmAddress;
  to: EvmAddress;
  amount: string;
  network: SendEvmUsdcNetwork;
  useCdpPaymaster?: boolean;
  paymasterUrl?: string;
}

```

### from

Explicitly sending from an EVM address (EOA or Smart Account).

### to

Recipient EVM address.

### amount

The amount of USDC to send (e.g., “1.50”).

### network

```
network: SendEvmUsdcNetwork;

```

EVM network to send on.

### useCdpPaymaster?

```
optional useCdpPaymaster: boolean;

```

Whether to use CDP Paymaster to sponsor gas fees (Smart Accounts only).

### paymasterUrl?

```
optional paymasterUrl: string;

```

Optional custom Paymaster URL (Smart Accounts only).

```
{
  from: SolanaAddress;
  to: SolanaAddress;
  amount: string;
  network: SendSolanaUsdcNetwork;
  createRecipientAta?: boolean;
}

```

### from

Explicitly sending from a Solana address.

### to

Recipient Solana address.

### amount

The amount of USDC to send (e.g., “1.50”).

### network

```
network: SendSolanaUsdcNetwork;

```

Solana network to send on.

### createRecipientAta?

```
optional createRecipientAta: boolean;

```

Whether to create recipient’s ATA if needed (~0.002 SOL cost).

```
{
  to: EvmAddress;
  amount: string;
  network: SendEvmUsdcNetwork;
  useCdpPaymaster?: boolean;
  paymasterUrl?: string;
}

```

### to

Auto-select mode for EVM - sender address will be automatically selected if exactly one account exists.

### amount

The amount of USDC to send (e.g., “1.50”).

### network

```
network: SendEvmUsdcNetwork;

```

EVM network to send on.

### useCdpPaymaster?

```
optional useCdpPaymaster: boolean;

```

Whether to use CDP Paymaster (only used if Smart Account is selected).

### paymasterUrl?

```
optional paymasterUrl: string;

```

Optional custom Paymaster URL (only used if Smart Account is selected).

```
{
  to: SolanaAddress;
  amount: string;
  network: SendSolanaUsdcNetwork;
  createRecipientAta?: boolean;
}

```

### to

Auto-select mode for Solana - sender address will be automatically selected if exactly one account exists.

### amount

The amount of USDC to send (e.g., “1.50”).

### network

```
network: SendSolanaUsdcNetwork;

```

Solana network to send on.

### createRecipientAta?

```
optional createRecipientAta: boolean;

```

Whether to create recipient’s ATA if needed (~0.002 SOL cost).