# reference

Technical reference for the Stableswapper swap instruction. For setup and examples, see [Quickstart](https://developer.chrome.com/custom-stablecoins/quickstart) or [Examples](https://developer.chrome.com/custom-stablecoins/examples).

* * *

**Program ID:** `9vDwZVJXw5nxymWmUcgmNpemDH5EBcJwLNhtsznrgJDH` ([Key Addresses](https://developer.chrome.com/custom-stablecoins/key-addresses))

### Parameters

Parameter

Type

Description

`amount_in`

`u64`

Amount of input token (in token’s native decimals)

`min_amount_out`

`u64`

Minimum acceptable output (slippage protection)

### Required Accounts

Account

Type

Description

`pool`

Read-only

Liquidity pool PDA (seed: `"liquidity_pool"`)

`inVault`

Read-only

Vault PDA for input token

`outVault`

Read-only

Vault PDA for output token

`inVaultTokenAccount`

Writable

Token account holding input token reserves

`outVaultTokenAccount`

Writable

Token account holding output token reserves

`userFromTokenAccount`

Writable

User’s token account for input token

`toTokenAccount`

Writable

Destination token account for output token

`feeRecipientTokenAccount`

Writable

Fee recipient’s token account for input token

`feeRecipient`

Read-only

Fee recipient authority (validated against pool)

`fromMint`

Read-only

Mint of the input token

`toMint`

Read-only

Mint of the output token

`user`

Signer, Writable

User wallet signing the transaction

`whitelist`

Read-only

Address whitelist PDA (seed: `"address_whitelist"`)

`tokenProgram`

Read-only

SPL Token Program

`associatedTokenProgram`

Read-only

Associated Token Program

`systemProgram`

Read-only

System Program

* * *

## Transaction Structure

A Solana transaction contains one or more instructions. Each instruction specifies:

-   **Program ID** — the on-chain program to execute
-   **Accounts** — all accounts the instruction reads from or writes to
-   **Instruction Data** — the encoded parameters for the instruction

**Account types:**

-   **Signer**: Must sign the transaction
-   **Writable**: Will be modified by the instruction
-   **Read-only**: Only read from, not modified

* * *

## What to read next