# faq

## General

### What is Payments MCP?

An MCP server + wallet app that lets AI agents autonomously pay for x402 services using USDC.

### Do I need API keys?

No. Authentication happens through your embedded wallet (email/OTP), and payments are automatic via x402.

### What tokens are supported?

USDC on Base network.

### How much does it cost?

-   Payments MCP: Free
-   Wallet creation: Free
-   Gas fees: Free (sponsored)
-   x402 service calls: Varies by service
-   Coinbase Onramp: Standard fees apply

### Is this custodial?

No. You control your embedded wallet through email/OTP. Coinbase doesn’t have custody of your funds.

## Wallet & Payments

### How do I add funds?

1.  Ask your agent: `Show me my wallet`
2.  Sign in with email/OTP
3.  Click **Fund**
4.  Follow Coinbase Onramp flow

### What if I lose access to my email?

Your wallet is tied to your email. Work with your email provider to recover access. Use an email you control.

### How do spending limits work?

Set them manually in the wallet UI:

-   **Max per call:** Max for a single payment (e.g., $0.05)
-   **Max per session:** Total session limit (e.g., $5.00)

Agents respect these limits but can’t change them.

### What if a payment fails?

No funds are deducted. Common reasons:

-   Insufficient balance
-   Network issues
-   Service unavailable

Your agent will notify you and can retry.

### Can I get refunds?

x402 payments are instant and onchain. Refunds depend on the service provider. Contact them directly.

## Services

### What is x402?

An open payment protocol for instant USDC payments over HTTP. Learn more: [x402 docs](https://developer.chrome.com/x402/welcome)

### How do I find services?

**Option 1:** Ask your agent: `What x402 services are available?` **Option 2:** Browse the Bazaar in the wallet app. Ask your agent: `Show me my wallet`, then click the **Discover** tab to filter services and copy ready-to-use prompts.

### Can I add my own x402 service?

Yes! Agents can discover and pay for any x402-compatible service. See: [x402 for sellers](https://developer.chrome.com/x402/quickstart-for-sellers)

### What network is used?

USDC on Base network for fast, low-cost settlement.

## Troubleshooting

### Payments MCP not showing up

1.  Restart your MCP client
2.  Check config: `npx @coinbase/payments-mcp status`
3.  Try: `npx @coinbase/payments-mcp install --force`
4.  Remove Payments MCP and reinstall

### Can’t sign in to wallet

1.  Check your email for OTP
2.  Try again: `Show me my wallet`
3.  Verify your internet connection

### Agent can’t make payments

1.  Check balance: `What's my wallet balance?`
2.  Verify spending limits in wallet UI
3.  Ensure you’re signed in: `Am I signed in?`

### Install failing

Run with verbose mode to see details:

```
npx @coinbase/payments-mcp install --verbose

```

## Getting Help

**Discord:** [discord.gg/invite/cdp](https://discord.gg/invite/cdp) **GitHub Issues:** [github.com/coinbase/payments-mcp/issues](https://github.com/coinbase/payments-mcp/issues) **Docs:**

-   [Quickstart](https://developer.chrome.com/payments-mcp/quickstart)
-   [Tools Reference](https://developer.chrome.com/payments-mcp/tools-reference)
-   [x402 Protocol](https://developer.chrome.com/x402/welcome)

## Security

Report vulnerabilities privately:

-   [Coinbase HackerOne](https://hackerone.com/coinbase)
-   [Security Policy](https://github.com/coinbase/payments-mcp/blob/main/SECURITY.md)