# examples

Real-world examples showing what your agent can do with Payments MCP.

## Trending crypto tokens

**You:**

```
What are the hottest tokens trending on crypto Twitter right now?

```

**Agent:**

```
📈 Top Trending Tokens (24h):
1. $PING - 995 transactions, score: 0.92
   Memecoin with growing Base ecosystem activity
   
2. $VIRTUAL - High social buzz
   AI agent protocol seeing increased mentions
   
3. $AIXBT - 890 transactions, score: 0.87
   AI agent for crypto news and analysis
Total: $0.02 USDC (trending tokens + social data)

```

**How it works:**

1.  Discovers TrendingTokenAgent and Twitter search services
2.  Gets trending tokens from CoinGecko + Twitter ($0.02)
3.  Checks Twitter mentions for context ($0.01)
4.  Returns ranked list with insights

* * *

## Crypto news digest

**You:**

```
Give me the latest crypto and AI news from the past 24 hours

```

**Agent:**

```
🗞️ Latest Crypto & AI News:
**Gloria AI News Feed:**
- Bitcoin ETF inflows hit $500M this week
- Base chain TVL surpasses $2B milestone
- New AI agent protocols launching on Solana
**Web3 Developments:**
- DeFi protocols see 40% volume increase
- Major DAO governance proposals passing
- New NFT marketplace integration
Sources: Gloria AI, TechCrunch API, Web3 News Aggregator
Total: $0.03 USDC

```

**How it works:**

1.  Discovers Gloria AI News and TechCrunch agents
2.  Fetches latest crypto news ($0.01)
3.  Gets AI/tech news ($0.01)
4.  Synthesizes into digest ($0.01)

* * *

## Token analysis

**You:**

```
Analyze the $VIRTUAL token and tell me if it's worth looking into

```

**Agent:**

```
📊 $VIRTUAL Token Analysis:
**Market Data** (via CoinGecko):
- Price: $2.45 (+12% 24h)
- Market Cap: $245M
- 24h Volume: $45M
- Holders: 15,234
**Social Sentiment** (via Twitter Intelligence):
- 847 mentions in past 24h
- Mostly positive sentiment
- Key influencers discussing
**Project Info** (via RootData):
- AI agent protocol on Base
- Recent $5M funding round
- Active development team
Recommendation: Strong fundamentals with growing community. Worth deeper research.
Total: $0.04 USDC

```

* * *

## Service discovery

**Option 1: Ask your agent**

```
What x402 services are available for crypto and AI?

```

**Agent:**

```
Found 141 services in the Bazaar. Here are top options:
📰 Gloria AI News ($0.01)
   • Latest crypto & AI news
   • Score: 0.87, 529 transactions
🤖 AIXBT Agent ($0.01)
   • AI crypto intelligence
   • Score: 0.86, 81 transactions
📊 DEX Screener ($0.01)
   • Live DEX trading data
   • Score: 0.91, 488 transactions
🐦 Twitter Agent ($0.01)
   • Post tweets, search mentions
   • Score: 0.87, 890 transactions
📈 Trending Tokens ($0.02)
   • CoinGecko + Twitter trends
   • Score: 0.85, 17 transactions
💎 RootData ($0.01)
   • Project research & funding
   • Score: 0.88, 703 transactions

```

**Option 2: Browse the Bazaar in the wallet app** Ask your agent: `Show me my wallet` In the wallet, click the **Discover** tab to visually browse all services, filter by network or text search, and click any service to copy a ready-to-use prompt.

* * *

## What to read next

-   [Quickstart](https://developer.chrome.com/payments-mcp/quickstart): Get started
-   [Tools Reference](https://developer.chrome.com/payments-mcp/tools-reference): See all tools
-   [FAQ](https://developer.chrome.com/payments-mcp/faq): Common questions