# key addresses

This page lists all deployed addresses for the Custom Stablecoins program on Solana. Reference these when constructing transactions or integrating via CPI.

* * *

## Deployment Status

Network

Status

Use Case

Solana Devnet

✅ Active

Testing and development

Solana Mainnet

Coming soon

Production use

* * *

## Program

Network

Program ID

Solana Devnet

`9vDwZVJXw5nxymWmUcgmNpemDH5EBcJwLNhtsznrgJDH`

* * *

## Token Mints

Network

Token

Mint Address

Solana Devnet

USDC

`4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU`

Solana Devnet

SFTUSD25

`AreQsswF44khJHLjsuWzgboJcG31JvALcWMkMcsVs2uC`

* * *

## Program Derived Addresses (PDAs)

These accounts are deterministically derived from the program ID using the seeds shown. You do not need to store them — derive them at runtime using `PublicKey.findProgramAddressSync`.

Account

Address

Seeds

Role

Pool

`68xwTJYRyUdULbb5ve4JMWTc2xvw9wQmtEj9GRoHSenF`

`["liquidity_pool"]`

Central pool state — holds fee rate, fee recipient, and supported vault list

Whitelist

`8zVAo6z2juAcVMkn3booA4yMGym5BPUJey2iN1Nq7Hvr`

`["address_whitelist"]`

Address allowlist checked against the transaction signer on every swap

Token vaults and vault token accounts are derived per-mint at runtime:

```
// Vault PDA (logical owner for a token's reserves)
const [vault] = PublicKey.findProgramAddressSync(
  [Buffer.from("token_vault"), pool.toBuffer(), mintAddress.toBuffer()],
  programId
);
// Vault Token Account (where reserves actually live)
const [vaultTokenAccount] = PublicKey.findProgramAddressSync(
  [Buffer.from("vault_token_account"), vault.toBuffer()],
  programId
);

```

* * *

## Constant Program Addresses

These well-known Solana program addresses are required as accounts in every swap instruction.

Program

Address

SPL Token Program

`TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`

Associated Token Program

`ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL`

System Program

`11111111111111111111111111111111`