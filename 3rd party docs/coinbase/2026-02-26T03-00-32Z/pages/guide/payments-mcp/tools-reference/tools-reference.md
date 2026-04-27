# tools reference

Your AI agent can use these tools to manage your wallet and make payments. You talk to your agent in natural language—the agent uses these tools behind the scenes.

### Get wallet address

Returns your wallet address. **Try:**

```
What's my wallet address?

```

* * *

### Get wallet balance

Shows your token balances (USDC, etc.). **Try:**

```
What's my wallet balance?

```

* * *

### Show wallet app

Opens the wallet interface where you can:

-   Sign in with email/OTP
-   Browse the **Bazaar** tab to explore x402 services, filter by category/price, and copy prompts
-   Add funds via Coinbase Onramp
-   View transaction history
-   Configure spending limits

**Try:**

* * *

### Check sign-in status

Checks if you’re signed in and provides next steps if not. **Try:**

**Note:** This runs automatically when your agent starts.

### List Bazaar resources

Discovers available x402 services (APIs, data sources, etc.) that your agent can pay for. **Try:**

```
What x402 services are available?

```

```
List services for onchain token data

```

**Returns:** Service names, descriptions, pricing, and quality scores.

* * *

### Get details about a resource

Gets full documentation for a specific service including API structure, parameters, and payment requirements. **Try:**

```
Get details for the weather API

```

**Note:** Your agent calls this before making payments to understand how to use the service.

* * *

### Make an x402 request

Makes an API call and handles payment automatically. This is how your agent pays for services. **Try:**

```
Get the most up to date crypto and AI news

```

```
Scrape product reviews for iPhone 15

```

**What happens:**

1.  Agent discovers payment requirements
2.  Sends USDC payment
3.  Calls the API
4.  Returns results

**Payment details:**

-   Uses USDC on Base network
-   No gas fees (sponsored)
-   Must be within your spending limits

* * *

### Check payment requirements for a resource

Checks what payment an endpoint requires without actually paying. Useful for non-bazaar services. **Try:**

```
What does this API cost to use?

```

**Note:** For bazaar services, use `bazaar_get_resource_details` instead; it includes more info.

## What agents CAN’T do

These features exist in the wallet UI, but agents cannot access them. Only you can:

### Set spending limits

Configure max spend per call and per session in the wallet UI. Agents respect these limits but can’t change them.

### Transfer funds

Send USDC to other addresses. Agents can only pay for x402 services, not send to arbitrary addresses.

### Add funds

Use Coinbase Onramp to add USDC. Agents can’t initiate onramp flows.

## Next steps

-   [Quickstart](https://developer.chrome.com/payments-mcp/quickstart): Set up Payments MCP
-   [Examples](https://developer.chrome.com/payments-mcp/examples): More use cases
-   [FAQ](https://developer.chrome.com/payments-mcp/faq): Common questions