# fund

## Overview

Add money to the wallet via Coinbase Onramp. Use when you or the user want to fund, deposit, top up, load, buy USDC, add funds, or onramp. Also use when the wallet has insufficient balance for a send or trade operation.

## Prerequisites

-   Must be authenticated (`npx awal@latest status` to check)
-   Coinbase Onramp is available in supported regions (US, etc.)

## Confirming wallet status

If the wallet is not authenticated, refer to the [authenticate-wallet](https://developer.chrome.com/agentic-wallet/skills/authenticate) skill.

## Opening the funding interface

This opens the wallet companion window where users can:

1.  Select a preset amount (10,10, 20, $50) or enter a custom amount
2.  Choose their preferred payment method
3.  Complete the purchase through Coinbase Pay

## Payment methods

Method

Description

**Apple Pay**

Fast checkout with Apple Pay (where available)

**Coinbase**

Transfer from existing Coinbase account

**Card**

Debit card payment

**Bank**

ACH bank transfer

## Funding flow

1.  Run `npx awal@latest show` to open the wallet UI
2.  Instruct the user to click the Fund button
3.  User selects amount and payment method in the UI
4.  User completes payment through Coinbase Pay (opens in browser)
5.  USDC is deposited to the wallet once payment confirms

## Alternative: Direct transfer

Users can send USDC on Base directly to the wallet address:

## Checking balance after funding

## Notes

-   Funding goes through Coinbase’s regulated onramp
-   Processing time varies by payment method:
    -   **Instant**: Card, Apple Pay
    -   **1-3 days**: Bank transfer
-   Funds are deposited as USDC on Base network