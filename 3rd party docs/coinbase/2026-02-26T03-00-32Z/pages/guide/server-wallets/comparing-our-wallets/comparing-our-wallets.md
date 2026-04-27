# comparing our wallets

Coinbase offers three wallet products to support different developer needs:

-   [**Embedded Wallet**](https://developer.chrome.com/embedded-wallets/welcome): User-controlled, app-specific wallet. Broad network and auth method support.
-   [**Server Wallet**](https://developer.chrome.com/server-wallets/v2/introduction/welcome): Developer-controlled wallet for app backends, onchain automation, and onchain agents.
-   [**Agentic Wallet**](https://developer.chrome.com/agentic-wallet/welcome): Wallet infrastructure for AI agents, accessible via agent skills/MCP with pre-built skills for payments, trading, and x402.

This guide compares these options and helps you decide which is best for your use case.

## Choosing the right wallet

Use Case

Recommended Solution

Giving each user a wallet tied to your app with web2-style auth methods

Embedded Wallet

Onboarding non-crypto users without requiring wallet setup

Embedded Wallet

Running onchain AI agents or trading bots with programmatic control

Server Wallet or Agentic Wallet

Managing app-wide funds like fees, rewards, or treasury

Server Wallet

Enabling AI agents to autonomously send, trade, and pay for services

Agentic Wallet

## Wallet feature comparison

Feature

Embedded Wallet

Server Wallet

Agentic Wallet

**Custody**

End user

Developer

Agent (agent-authenticated)

**Network support**

All EVM networks + Solana

All EVM networks + Solana

Base

**Wallet types**

EVM EOA and Smart accounts + Solana accounts

EOA and Smart Accounts (4337) for EVM; Solana accounts

Embedded wallet via agent skill/MCP

**Authentication**

User Auth (email OTP, social login, etc.)

Developer-held Wallet Secret

Email OTP

**Private key management**

Private keys secured in Coinbase systems

Private keys secured in Coinbase systems

Private keys secured in Coinbase systems

**Transaction signing**

User-initiated signing via SDK

Developer-initiated signing via API

Agent-initiated via CLI/skills

**Primary interface**

React SDK

REST API / SDKs

CLI (`npx awal`) + Agent Skills

**x402 integration**

✅ [Guide](https://developer.chrome.com/embedded-wallets/x402-payments)

—

✅ Built-in skills

**Gasless trading**

Via Paymaster

Via Paymaster

✅ Built-in on Base

## What to read next

-   **Embedded Wallet**: [Quickstart](https://developer.chrome.com/embedded-wallets/quickstart)
-   **Server Wallet**: [Quickstart](https://developer.chrome.com/server-wallets/v2/introduction/quickstart) | [Security](https://developer.chrome.com/server-wallets/v2/introduction/security) | [Smart Accounts](https://developer.chrome.com/server-wallets/v2/evm-features/smart-accounts)
-   **Agentic Wallet**: [Quickstart](https://developer.chrome.com/agentic-wallet/quickstart) | [Skills](https://developer.chrome.com/agentic-wallet/skills/overview)