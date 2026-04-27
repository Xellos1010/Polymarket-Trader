# trade

## Overview

Trade tokens on Base network. Use when you or the user want to trade, exchange, buy, sell, or convert between tokens.

## Prerequisites

-   Must be authenticated (`npx awal@latest status` to check)
-   Wallet must have sufficient balance of the source token

## Confirming wallet status

If the wallet is not authenticated, refer to the [authenticate-wallet](https://developer.chrome.com/agentic-wallet/skills/authenticate) skill.

## Command syntax

```
npx awal@latest trade <amount> <from> <to> [options]

```

## Arguments

Argument

Description

`amount`

Amount to trade (see Amount Formats below)

`from`

Source token: alias (usdc, eth, weth) or contract address

`to`

Destination token: alias (usdc, eth, weth) or contract address

## Amount formats

Format

Example

Description

Dollar prefix

`$1.00`, `$0.50`

USD notation

Decimal

`1.0`, `0.50`, `0.001`

Human-readable with decimal point

Whole number

`5`, `100`

Interpreted as whole tokens

Atomic units

`500000`

Large integers treated as atomic units

## Options

Option

Description

`-s, --slippage <n>`

Slippage tolerance in basis points (100 = 1%)

`--json`

Output result as JSON

## Token aliases

Alias

Token

Decimals

Address

usdc

USDC

6

0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913

eth

ETH

18

0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE

weth

WETH

18

0x4200000000000000000000000000000000000006

## Examples

```
# Trade $1 USDC for ETH
npx awal@latest trade $1 usdc eth
# Trade 0.50 USDC for ETH
npx awal@latest trade 0.50 usdc eth
# Trade 0.01 ETH for USDC
npx awal@latest trade 0.01 eth usdc
# Trade with custom slippage (2%)
npx awal@latest trade $5 usdc eth --slippage 200
# Trade using contract addresses
npx awal@latest trade 100 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913 0x4200000000000000000000000000000000000006
# Get JSON output
npx awal@latest trade $1 usdc eth --json

```

## Error handling

Error

Resolution

”Not authenticated”

Run `npx awal@latest auth login <email>` first

”Invalid token”

Use a valid alias (usdc, eth, weth) or 0x address

”Cannot trade a token to itself”

From and to must be different

”Trade failed: TRANSFER\_FROM\_FAILED”

Insufficient balance or approval issue

”No liquidity”

Try a smaller amount or different token pair

”Amount has X decimals but token only supports Y”

Too many decimal places