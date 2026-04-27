# sendsolanausdcoptions

```
type SendSolanaUsdcOptions = {
  solanaAccount: SolanaAddress;
  to: SolanaAddress;
  amount: string;
  network: SendSolanaUsdcNetwork;
  createRecipientAta?: boolean;
};

```

Request parameters for sending Solana USDC.

## Properties

Property

Type

Description

`solanaAccount`

[`SolanaAddress`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SolanaAddress)

The Solana account to send USDC from.

`to`

[`SolanaAddress`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SolanaAddress)

The recipient address (base58 encoded).

`amount`

`string`

The amount of USDC to send in human-readable format (e.g., “1.50” for 1.5 USDC).

`network`

[`SendSolanaUsdcNetwork`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SendSolanaUsdcNetwork)

The network to send USDC on.

`createRecipientAta?`

`boolean`

Whether to automatically create an Associated Token Account (ATA) for the recipient if it doesn’t exist. **Default** false **Important**: Creating an ATA requires 0.002 SOL ($0.20) for rent exemption, which the sender pays. - Set to `true` if you want to ensure the transaction succeeds by creating the recipient’s ATA if needed - Set to `false` (default) to only send if the recipient already has a USDC token account If `false` and the recipient doesn’t have an ATA, the transaction will fail with a clear error message.