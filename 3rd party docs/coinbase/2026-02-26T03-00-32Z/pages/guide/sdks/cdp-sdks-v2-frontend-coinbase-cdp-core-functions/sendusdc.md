# sendusdc

```
function sendUsdc(options: SendUsdcOptions): Promise<SendUsdcResult>;

```

Sends USDC on EVM or Solana networks with automatic address type detection. Automatically determines if the sender is an EOA, Smart Account, or Solana account and routes to the appropriate sending function. If `from` is not provided, automatically selects the address only when there is exactly one account to avoid ambiguity. If multiple accounts exist, explicit selection is required.

## Parameters

Parameter

Type

Description

`options`

[`SendUsdcOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SendUsdcOptions)

The options for sending USDC. See [SendUsdcOptions](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SendUsdcOptions) for all available parameters.

## Returns

`Promise`<[`SendUsdcResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SendUsdcResult)\> A discriminated union result indicating the account type and transaction identifier.

## Example

```
// Simplest usage - auto-selects sender when user has exactly one account
const result = await sendUsdc({
  to: "0x1234567890123456789012345678901234567890",
  amount: "10.00",
  network: "base-sepolia",
});
// Send from Smart Account with gas sponsorship
const smartResult = await sendUsdc({
  from: user.evmSmartAccountObjects[0].address,
  to: "0x1234567890123456789012345678901234567890",
  amount: "10.00",
  network: "base",
  useCdpPaymaster: true,
});
// Result: { type: "evm-smart", userOpHash: "0x..." }
// Send from Solana account
const solanaResult = await sendUsdc({
  from: user.solanaAccountObjects[0].address,
  to: "ExXhNkgYf6efh7YyqDRVxPZuzafobao1A74drUdp8trd",
  amount: "10.00",
  network: "solana-devnet",
  createRecipientAta: true,
});
// Result: { type: "solana", transactionSignature: "..." }

```

## Throws

If addresses are invalid, mismatched types, user doesn’t own the sender account, or no accounts available.