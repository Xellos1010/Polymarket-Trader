# systems operations

## Production URLs

Use the following URLs to connect to Coinbase Prime production APIs.

API

URL

REST API

`https://api.prime.coinbase.com/v1`

Websocket Feed

`wss://ws-feed.prime.coinbase.com`

FIX 4.2 & 5.0 API

`tcp+ssl://fix.prime.coinbase.com:4198`

The WebSocket feed supports real-time market data and order updates through channels including heartbeats, orders, and Level2 data. See [WebSocket Channels](https://developer.chrome.com/prime/websocket-feed/channels) for complete channel specifications.

## Availability Zones

The infrastructure for Coinbase Prime is hosted in **US-EAST-1 (AWS)**. For information around Coinbase Exchange, a downstream venue for Prime, please visit [Exchange Systems & Operations](https://developer.chrome.com/exchange/introduction/systems-operations).

## System Components

### REST Entry Gateways

-   Requests are routed through Cloudflare.
-   Requests are load-balanced across available instances.
-   REST requires per-request authentication headers and signature; FIX authentication happens at session login.

### FIX Order Gateways

-   Each instance contains a per-user product based queue.
-   Each per-user product-based queue is processed on a FIFO basis.

### Smart Order Router

-   Routes orders across multiple connected liquidity venues to achieve best execution.
-   Aggregates liquidity from multiple venues into unified order books for each trading pair.
-   Supports venue filtering and the option to enable venue fee transparency.