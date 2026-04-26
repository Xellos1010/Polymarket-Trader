# send

## Overview

Send USDC to an Ethereum address or ENS name. Use when you or the user want to send money, pay someone, transfer USDC, tip, donate, or send funds to a wallet address or .eth name.

## Prerequisites

-   Must be authenticated (`npx awal@latest status` to check)
-   Wallet must have sufficient USDC balance (`npx awal@latest balance` to check)

## Confirming wallet status

If the wallet is not authenticated, refer to the [authenticate-wallet](https://developer.chrome.com/agentic-wallet/skills/authenticate) skill.

## Command syntax

```
npx awal@latest send <amount> <recipient> [--chain <chain>] [--json]

```

## Arguments

Argument

Description

`amount`

Amount to send: `$1.00`, `1.00`, or atomic units (1000000 = $1). Numbers > 100 without decimals are treated as atomic units.

`recipient`

Ethereum address (0x…) or ENS name (vitalik.eth)

## Options

Option

Description

`--chain <name>`

Blockchain network: `base` (default) or `base-sepolia`

`--json`

Output result as JSON

## Examples

```
# Send $1.00 USDC to an address
npx awal@latest send 1 0x1234...abcd
# Send $0.50 USDC to an ENS name
npx awal@latest send 0.50 vitalik.eth
# Send with dollar sign prefix
npx awal@latest send "$5.00" 0x1234...abcd
# Send on Base Sepolia testnet
npx awal@latest send 1 0x1234...abcd --chain base-sepolia
# Get JSON output
npx awal@latest send 1 vitalik.eth --json

```

## ENS resolution

ENS names are automatically resolved to addresses via Ethereum mainnet. The command will:

1.  Detect ENS names (any string containing a dot that isn’t a hex address)
2.  Resolve the name to an address
3.  Display both the ENS name and resolved address in the output

## Error handling

Error

Resolution

”Not authenticated”

Run `npx awal@latest auth login <email>` first

”Insufficient balance”

Check balance with `npx awal@latest balance` and fund wallet

”Could not resolve ENS name”

Verify the ENS name exists

”Invalid recipient”

Must be valid 0x address or ENS name