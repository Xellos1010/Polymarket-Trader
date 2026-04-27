# systems operations

## Deployment

The deployment schedules for different components vary and may change without notice.

API

Schedule

FIX

Monday, Thursday at 2PM ET

WebSocket

Monday, Wednesday, Thursday at 2PM ET

REST

Monday, Wednesday, Thursday at 2PM ET

## Production URLs

Use the following URLs to connect to Coinbase Exchange production APIs. See [Sandbox URLs](https://developer.chrome.com/exchange/introduction/sandbox) for testing.

API

URL

REST API

`https://api.exchange.coinbase.com`

Websocket Feed

`wss://ws-feed.exchange.coinbase.com`

Websocket Direct Feed

`wss://ws-direct.exchange.coinbase.com`

FIX 5.0 API - Order Entry

`tcp+ssl://fix-ord.exchange.coinbase.com:6121`

FIX 5.0 API - Market Data Snapshot Enabled

`tcp+ssl://fix-md.exchange.coinbase.com:6121`

FIX 5.0 API - Market Data Snapshot Disabled

`tcp+ssl://fix-md.exchange.coinbase.com:6122`

FIX 5.0 API - Dedicated Drop Copy

`tcp+ssl://fix-dc.exchange.coinbase.com:6122`

## MiCA Production URLs

MiCA clients must use the following URLs to connect to Coinbase Exchange production APIs. See [Sandbox URLs](https://developer.chrome.com/exchange/introduction/sandbox) for testing.

API

URL

REST API

`https://api-us.dma.prime.coinbase.com`

Websocket Feed

`wss://ws-us.dma.prime.coinbase.com`

Websocket Direct Feed

`wss://ws-us-direct.dma.prime.coinbase.com`

FIX 5.0 API - Order Entry

`tcp+ssl://fix-us.dma.prime.coinbase.com:7110`

FIX 5.0 API - Market Data Snapshot Enabled

`tcp+ssl://fix-us.dma.prime.coinbase.com:7120`

FIX 5.0 API - Market Data Snapshot Disabled

`tcp+ssl://fix-us.dma.prime.coinbase.com:7121`

FIX 5.0 API - Dedicated Drop Copy

`tcp+ssl://fix-dc.dma.prime.coinbase.com:7122`

## Availability Zones

The infrastructure for the US Spot Exchange is hosted in **US-EAST-1 (AWS)** within multiple availability zones.

Product

Availability Zone ID

FIX Order Gateways

use1-az4

Order Entry Gateway

use1-az4

Trade Engine

use1-az4

Web Socket Market Data

use1-az4

FIX Market Data

use1-az4

## System Components

### REST Entry Gateways

-   Requests are routed through Cloudflare.
-   Requests are processed on a FIFO basis with no queuing.
-   REST requires additional authentication because it’s stateless (as opposed to FIX order gateways, which authenticate during login).

### FIX Order Gateways

-   Each instance contains a per-user product based queue.
-   Each per-user product-based queue can hold a maximum of 50 queued requests before requests are rejected.
-   Each per-user product-based queue is processed on a FIFO basis.

### Order Entry Gateway (Risk System)

-   Each instance processes requests from FIX Order Gateways and REST in real time with no queuing.
-   System performs real-time risk checks and account collateralization.

### Trade Engine

-   Clustered service that guarantees FIFO sequencing at a product level.
-   Processes all requests from Order Entry Gateway.
-   Publishes market data to WebSocket / FIX Market Data.

### Market Data (Websocket & FIX)

-   Each instance can process all market data requests across all products.
-   Messages are distributed to customers randomly, and there is no intended benefit to being “first to subscribe”.