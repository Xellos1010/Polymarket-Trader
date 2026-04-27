# welcome

Give your AI agent a wallet. Pay for APIs, send money, and trade tokens safely, with built-in spending limits.

## What is Agentic Wallet?

Agentic Wallet gives any AI agent a standalone wallet to hold and spend stablecoins, or trade for other tokens on Base. Built on Coinbase Developer Platform (CDP) infrastructure, agents can authenticate via email OTP, hold USDC, and send, trade, or pay for services without ever accessing private keys.

Comparing AgentKit vs Agentic Wallet

[AgentKit](https://developer.chrome.com/agent-kit/welcome)

Agentic Wallet

**What it is**

SDK/toolkit for onchain actions

Standalone wallet via CLI/MCP

**Integration**

Import into your agent code

Can call CLI or MCP tools (e.g., `npx awal send`)

**Scope**

Full onchain capabilities (deploy contracts, etc.)

Wallet ops: send, trade, x402

**Networks**

Multi-network (EVM + Solana)

Base

## Use cases

## Capabilities

Feature

Description

**Wallet identity**

Self-custody wallet controlled by the agent

**Spending limits**

Configurable caps per session and per transaction

**Gasless trading**

Token swaps on Base without requiring gas

**Skill extensibility**

Add new capabilities via [agentic-wallet-skills](https://github.com/coinbase/agentic-wallet-skills)

**[x402](https://developer.chrome.com/x402/core-concepts/how-it-works) payments**

Machine-to-machine paid API requests

### Security

-   **Key isolation**: Private keys stay in Coinbase infrastructure
-   **Spending guardrails**: Enforce limits before any transaction
-   **KYT screening**: Block high-risk interactions automatically

## Components

### awal CLI

Command-line tool for wallet operations. Use it directly for testing, or let agents invoke it via skills.

```
npx awal status      # Check auth status
npx awal send 1 vitalik.eth   # Send USDC
npx awal trade 5 usdc eth     # Swap tokens

```

### Agent Skills

Instead of manually wiring wallet operations into your agent, install skills and let the agent handle it.

```
npx skills add coinbase/agentic-wallet-skills

```

Skills include: authenticate, fund, send, trade, search-for-service, pay-for-service, monetize-service.

### x402 Integration

Protocol for machine-to-machine payments. Agents can both consume and provide paid APIs with [x402](https://developer.chrome.com/x402/core-concepts/how-it-works), enabling agent-to-agent commerce.

## What to read next