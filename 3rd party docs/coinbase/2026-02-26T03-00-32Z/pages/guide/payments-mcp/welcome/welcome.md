# welcome

## What is Payments MCP?

Payments MCP is an MCP server & companion wallet app that combines [wallets](https://developer.chrome.com/embedded-wallets/welcome), [onramps](https://developer.chrome.com/onramp/coinbase-hosted-onramp/overview), and [payments via x402](https://developer.chrome.com/x402/welcome) into a single solution for agentic commerce. It enables AI agents to autonomously discover and pay for services without API keys, complex seed phrases, or manual intervention.

## Who is this for?

Anyone using AI agents (Claude, ChatGPT, etc.) who wants them to:

-   Pay for premium APIs and data services
-   Complete multi-step workflows autonomously
-   Access web scraping, weather data, and other x402 services

## How does it work?

1.  **Install:** Run `npx @coinbase/payments-mcp`
2.  **Sign in:** Authenticate with email/OTP (creates embedded wallet)
3.  **Add funds:** Use Coinbase Onramp to add USDC
4.  **Start using:** Your agent can now discover and pay for services automatically

**Example:**

```
Based on the latest AI news, give me a report on the top 3 
up and coming tokens on Solana.

```

Your agent will:

-   Discover news, crypto data, and token info services
-   Pay for API calls with USDC
-   Return a report based on premium data

## What can agents do?

**Wallet management:**

-   Check balance
-   Get wallet address
-   Open wallet UI (includes Bazaar explorer to browse services)

**Payments:**

-   Discover x402 services
-   Make automatic payments
-   Access paid APIs

Agents don’t have access to sensitive actions. **You stay in control of:**

-   Spending limits (max per-call, max per-session)
-   Fund transfers
-   Onramp flows

## Supported clients

-   Claude Desktop
-   Claude Code
-   Codex CLI
-   Gemini CLI
-   Cherry Studio
-   and most stdio-compatible MCP clients

## Next steps

-   [Quickstart](https://developer.chrome.com/payments-mcp/quickstart): Get set up in 5 minutes
-   [Tools Reference](https://developer.chrome.com/payments-mcp/tools-reference): See what agents can do
-   [Examples](https://developer.chrome.com/payments-mcp/examples): Real-world use cases
-   [FAQ](https://developer.chrome.com/payments-mcp/faq): Common questions