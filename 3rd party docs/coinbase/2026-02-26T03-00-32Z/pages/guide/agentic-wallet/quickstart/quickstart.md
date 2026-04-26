# quickstart

Get your first agentic wallet running in under 2 minutes.

## Prerequisites

-   [Node.js 24+](https://nodejs.org/en/download)
-   An email address for wallet authentication
-   Install our package built on top of [Vercel AI SDK skills](https://sdk.vercel.ai/docs):

```
npx skills add coinbase/agentic-wallet-skills

```

## CLI commands

The `awal` CLI provides all the tools you need to manage agentic wallets:

Command

Purpose

`npx awal status`

Check server health and auth status

`npx awal auth login <email>`

Send OTP code to email, returns flowId

`npx awal auth verify <flowId> <otp>`

Complete authentication with OTP code

`npx awal balance [--chain]`

Get USDC wallet balance

`npx awal address`

Get wallet address

`npx awal show`

Open the wallet companion window

`npx awal send <amount> <recipient> [--chain]`

Send USDC to an address or ENS name

`npx awal trade <amount> <from> <to>`

Trade tokens on Base

`npx awal x402 bazaar search <query>`

Search for paid API services

`npx awal x402 pay <url>`

Make a paid API request

## Authenticating an agentic wallet

Authentication uses an initiate, then verify process:

### 1\. Initiate login

The following will install the `awal` package and create an agentic wallet mapped to the given email:

```
npx awal auth login user@example.com

```

You should see similar output:

```
✓ Verification code sent!
ℹ Check your email (user@example.com) for a 6-digit code.
Flow ID: 8beba1c2-5674-4f24-a0fa-...
To complete sign-in, run:
  awal auth verify 8beba1c2-5674-4f24-a0fa-... <6-digit-code>

```

### 2\. Verify OTP

To complete agentic wallet authentication, provide the `flowId` from the previous step and the 6-digit verification code:

```
npx awal auth verify <flowId> <otp>

```

You should see similar output:

```
✔ Authentication successful!
Successfully signed in as user@example.com
You can now use wallet commands:
  awal balance
  awal address

```

### 3\. Confirm authentication

You should see similar output:

```
Wallet Server
✓ Running (PID: 61234)
Authentication
✓ Authenticated
Logged in as: user@example.com

```

## Example usage

```
# Check current status
npx awal status
# Start login (sends OTP to email)
npx awal auth login agent@mycompany.com
# Output: flowId: abc123...
# After receiving code, verify
npx awal auth verify abc123 123456
# Confirm authentication
npx awal status
# Check balance
npx awal balance
# Send USDC
npx awal send 1 vitalik.eth
# Trade tokens
npx awal trade 5 usdc eth

```

## JSON output

All commands support `--json` for machine-readable output:

```
npx awal status --json
npx awal auth login user@example.com --json
npx awal balance --json

```

## What to read next