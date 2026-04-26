# authenticate

## Overview

Sign in to the wallet via email OTP. Use when you or the user want to log in, sign in, connect, or set up the wallet, or when any wallet operation fails with authentication errors. This skill is a prerequisite before sending, trading, or funding.

## Authentication flow

Authentication uses a two-step email OTP process.

### 1\. Initiate login

```
npx awal@latest auth login <email>

```

This sends a 6-digit verification code to the email and outputs a `flowId`.

### 2\. Verify OTP

```
npx awal@latest auth verify <flowId> <otp>

```

Use the `flowId` from step 1 and the 6-digit code from the user’s email to complete authentication.

## Checking authentication status

Displays wallet server health and authentication status, including the wallet address.

## Example session

```
# Check current status
npx awal@latest status
# Start login (sends OTP to email)
npx awal@latest auth login user@example.com
# Output: flowId: abc123...
# After user receives code, verify
npx awal@latest auth verify abc123 123456
# Confirm authentication
npx awal@latest status

```

## CLI commands

Command

Purpose

`npx awal@latest status`

Check server health and auth status

`npx awal@latest auth login <email>`

Send OTP code to email, returns flowId

`npx awal@latest auth verify <flowId> <otp>`

Complete authentication with OTP code

`npx awal@latest balance`

Get USDC wallet balance

`npx awal@latest address`

Get wallet address

`npx awal@latest show`

Open the wallet companion window

## JSON output

All commands support `--json` for machine-readable output:

```
npx awal@latest status --json
npx awal@latest auth login user@example.com --json
npx awal@latest auth verify <flowId> <otp> --json

```