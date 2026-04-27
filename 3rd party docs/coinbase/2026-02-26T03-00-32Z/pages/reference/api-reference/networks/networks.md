# networks

The networks described here are supported by CDP REST and JSON-RPC APIs described in the left-hand nav.

## REST API

A comprehensive list of CDP APIs offered across the networks we support, along with their corresponding testnets.

### Base

### Ethereum

### Solana

### Others

We also support Arbitrum and Polygon.

## JSON-RPC API

[CDP Node](https://developer.chrome.com/data/node/overview) provides free, rate-limited RPC endpoints built for [Base](https://docs.base.org/) and the Base Sepolia testnet. Free users are limited to approximately [50 requests per second](https://developer.chrome.com/data/node/overview#rate-limits). Please reach out in #node on our [CDP Discord](https://discord.com/invite/cdp) to request a limit increase.

### Base

RPC namespace

Functionality

Base Mainnet

Base Sepolia

[`cdp_*`](https://developer.chrome.com/api-reference/json-rpc-api/address-history)

Historical address data

✅

✅

[`pm_*`](https://developer.chrome.com/api-reference/json-rpc-api/paymaster)

Gas sponsorship management (Paymaster)

✅

✅

[`eth_*`](https://developer.chrome.com/api-reference/json-rpc-api/core)

Base-specific EVM functionality

✅

✅

[`web3_*`](https://developer.chrome.com/api-reference/json-rpc-api/core#web3-namespace)

Client information

✅

✅

[`debug_*`](https://developer.chrome.com/api-reference/json-rpc-api/core#debug-namespace)

Debug tools

✅

✅

[`net_*`](https://developer.chrome.com/api-reference/json-rpc-api/core#net-namespace)

Network info

✅

✅

## Network identifiers

The following table shows the network identifiers necessary for constructing requests to various CDP APIs:

Network

EVM Chain ID

HTTP API Identifier

JSON-RPC API Identifier

Arbitrum Mainnet

**42161** (0xa4b1)

`arbitrum-mainnet`

`arbitrum`

Base Mainnet

**8453** (0x2105)

`base-mainnet`

`base`

Base Sepolia

**84532** (0x14a34)

`base-sepolia`

`base-sepolia`

Bitcoin Mainnet

\-

`bitcoin-mainnet`

`bitcoin`

Ethereum Hoodi

**560048** (0x88bb0)

`ethereum-hoodi`

\-

Ethereum Mainnet

**1** (0x1)

`ethereum-mainnet`

`ethereum`

Optimism Mainnet

**10** (0xa)

`optimism-mainnet`

`optimism`

Polygon Mainnet

**137** (0x89)

`polygon-mainnet`

`polygon`

Solana Devnet

\-

`solana-devnet`

\-

Solana Mainnet

\-

`solana-mainnet`

\-

## What to read next

-   [Supported Networks](https://developer.chrome.com/get-started/supported-networks): A more high-level overview of the CDP product suite and supported features.