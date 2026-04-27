# overview

Launch your own branded stablecoin, backed 1:1 by USDC reserves on Coinbase, with instant onchain liquidity. This documentation is for **technical partners** integrating with the Custom Stablecoins swap program. You’ll find everything you need to construct and submit swap transactions from your application or smart contract.

## Use cases

## Key features

-   **1:1 USDC backing**: Fully backed by USDC reserves on Coinbase
-   **Instant swaps**: Single-transaction settlement on Solana
-   **Predictable pricing**: Fixed 1:1 ratio with no slippage surprises
-   **Flexible integration**: API or Cross-Program Invocation (CPI) from Solana programs
-   **Multi-decimal support**: Automatic normalization for 6-9 decimal tokens

## Onchain liquidity

Stableswapper is the onchain liquidity program that powers instant swaps between USDC and any Custom Stablecoin issued through the program. Built on Solana, it provides the settlement layer partners need to let their users move freely between custom tokens and USDC. Learn how to [integrate swaps](https://developer.chrome.com/custom-stablecoins/quickstart) into your application.

## How it works

The Stableswapper program maintains a **liquidity pool** with separate vaults for each supported token.

Swaps settle atomically in a single Solana transaction at a **1:1 ratio before fees**, with automatic decimal normalization between tokens with different precision (6-9 decimals).

## Concepts

Concept

Description

**PDA** (Program Derived Address)

Deterministic on-chain account owned by the program — used for the pool, vaults, and whitelist

**Vault**

A PDA that acts as the logical owner for a token’s reserves within the pool

**Vault Token Account**

The SPL token account that actually holds the token reserves for a given vault

**ATA** (Associated Token Account)

A user’s standard token account for a given mint, derived deterministically from their wallet

**Whitelist**

An optional on-chain list of authorized wallet addresses. When enabled, only listed addresses can execute swaps

**Slippage**

The minimum acceptable output amount (`min_amount_out`). Protects against fee changes between transaction construction and execution

## What to read next