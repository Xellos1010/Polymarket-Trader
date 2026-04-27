# search for service

## Overview

Search and browse the x402 bazaar marketplace for paid API services. Use when you or the user want to find available services, discover APIs, or need an external service to accomplish a task. No authentication or balance is required for searching.

## Commands

### Search the bazaar

Find paid services by keyword:

```
npx awal@latest x402 bazaar search <query> [-k <n>] [--force-refresh] [--json]

```

Option

Description

`-k, --top <n>`

Number of results (default: 5)

`--force-refresh`

Re-fetch resource index from CDP API

`--json`

Output as JSON

Results are cached locally at `~/.config/awal/bazaar/` and auto-refresh after 12 hours.

### List bazaar resources

Browse all available resources:

```
npx awal@latest x402 bazaar list [--network <network>] [--full] [--json]

```

Option

Description

`--network <name>`

Filter by network (base, base-sepolia)

`--full`

Show complete details including schemas

`--json`

Output as JSON

### Inspect payment requirements

Check an endpoint’s x402 payment requirements without paying:

```
npx awal@latest x402 details <url> [--json]

```

Auto-detects the correct HTTP method by trying each until it gets a 402 response, then displays price, accepted payment schemes, network, and input/output schemas.

## Examples

```
# Search for weather-related paid APIs
npx awal@latest x402 bazaar search "weather"
# Search with more results
npx awal@latest x402 bazaar search "sentiment analysis" -k 10
# Browse all bazaar resources with full details
npx awal@latest x402 bazaar list --full
# Check what an endpoint costs
npx awal@latest x402 details https://example.com/api/weather

```

## Next steps

Once you’ve found a service, use the [pay-for-service](https://developer.chrome.com/agentic-wallet/skills/pay-for-service) skill to make a paid request.

## Error handling

Error

Resolution

”CDP API returned 429”

Rate limited; cached data will be used if available

”No X402 payment requirements found”

URL may not be an x402 endpoint