# pay for service

## Overview

Make a paid API request to an x402 endpoint with automatic USDC payment. Use when you or the user want to call a paid API, make an x402 request, or use a paid service. Use [search-for-service](https://developer.chrome.com/agentic-wallet/skills/search-for-service) first to find available endpoints.

## Prerequisites

-   Must be authenticated (`npx awal@latest status` to check)
-   Wallet must have sufficient USDC balance (`npx awal@latest balance` to check)

## Confirming wallet status

If the wallet is not authenticated, refer to the [authenticate-wallet](https://developer.chrome.com/agentic-wallet/skills/authenticate) skill.

## Command syntax

```
npx awal@latest x402 pay <url> [-X <method>] [-d <json>] [-q <params>] [-h <json>] [--max-amount <n>] [--json]

```

## Options

Option

Description

`-X, --method <method>`

HTTP method (default: GET)

`-d, --data <json>`

Request body as JSON string

`-q, --query <params>`

Query parameters as JSON string

`-h, --headers <json>`

Custom HTTP headers as JSON string

`--max-amount <amount>`

Max payment in USDC atomic units (1000000 = $1.00)

`--correlation-id <id>`

Group related operations

`--json`

Output as JSON

## USDC amounts

x402 uses USDC atomic units (6 decimals):

Atomic Units

USD

1000000

$1.00

100000

$0.10

50000

$0.05

10000

$0.01

## Examples

```
# Make a GET request (auto-pays)
npx awal@latest x402 pay https://example.com/api/weather
# Make a POST request with body
npx awal@latest x402 pay https://example.com/api/sentiment -X POST -d '{"text": "I love this product"}'
# Limit max payment to $0.10
npx awal@latest x402 pay https://example.com/api/data --max-amount 100000

```

## Error handling

Error

Resolution

”Not authenticated”

Run `npx awal@latest auth login <email>` first

”No X402 payment requirements found”

URL may not be an x402 endpoint; use [search-for-service](https://developer.chrome.com/agentic-wallet/skills/search-for-service) to find valid endpoints

”Insufficient balance”

Fund wallet with USDC; see [fund](https://developer.chrome.com/agentic-wallet/skills/fund) skill