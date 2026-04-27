# multinetwork

Coinbase Prime supports sending and receiving assets across multiple blockchain networks. This multi-network functionality enables faster and more flexible capital movement for supported assets such as USDC, ETH, and BTC.

## Understanding Multi-Network Balances

Prime maintains consolidated trading balances for each asset, regardless of which network deposits or withdrawals occur on. When you use the Prime Trading Balance, all network-level activity is aggregated into a single asset balance. For example, if you deposit 1 USDC on Ethereum and 1 USDC on Base, you’ll see:

-   Two separate transaction records, each with its own network-specific symbol (`usdc` and `baseusdc`)
-   A combined balance of 2 USDC in your USDC trading balance

Similarly, depositing 1 cbBTC on Base into your Trading Balance will increase your BTC balance by 1 BTC. This approach lets you move capital freely across networks while avoiding operational overhead such as wallet consolidation or managing network-specific fee tokens.

## Supported Assets and Networks

Prime currently supports multi-network functionality for the following assets:

Asset

Network

Network-Scoped Symbol

USDC

Ethereum

`usdc`

USDC

Avalanche C-Chain

`avausdc`

USDC

Optimism

`optusdc`

USDC

Base

`baseusdc`

USDC

Solana

`splusdc`

USDC

Arbitrum

`arbusdc`

ETH

Ethereum

`eth`

ETH

Base

`baseeth`

BTC

Ethereum

`cbbtc`

BTC

Base

`basecbbtc`

Please note not every network will be available across all jurisdictions that Prime supports.

## Network-Scoped Symbols

To distinguish assets on different networks, Prime uses **network-scoped symbols**. These symbols appear in transaction records and API responses to indicate which specific network an asset is associated with. When calling the [List Assets](https://developer.chrome.com/api-reference/prime-api/rest-api/assets/list-assets) endpoint, the response includes `network_details` for each supported network, with a unique `network_scoped_symbol` for each network where the asset is available. For example, a subset of the USDC response includes:

-   `usdc` for Ethereum
-   `baseusdc` for Base
-   `optusdc` for Optimism

It is therefore important that when building reconciliation, accounting, and send/receive functionality, you take into consideration the need to normalize these symbols given the unified nature of the Prime trading balance.

## Working with Network-Specific Data

Many Prime API endpoints now include network information and filtering capabilities:

### Transactions and Activities

When working with transactions, you can typically filter by network-scoped symbols to isolate activity for a specific asset-network pair:

-   **[List Portfolio Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-portfolio-transactions)** - Returns `network` field for onchain transactions
-   **[Get Transaction by ID](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/get-transaction-by-transaction-id)** - Returns `network` field for onchain transactions
-   **[List Entity Activities](https://developer.chrome.com/api-reference/prime-api/rest-api/activities/list-entity-activities)** - Filter activities using `symbols` parameter with network-scoped symbols
-   **[Create Withdrawal](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/create-withdrawal)** - Accepts optional `network` parameter to specify the destination network for onchain withdrawals. When left out, the withdrawal will default to the default network for a given asset, which is specified by data included in [List Assets](https://developer.chrome.com/api-reference/prime-api/rest-api/assets/list-assets).

### Wallets

Wallet endpoints now include network information for vault wallets:

-   **[List Portfolio Wallets](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/list-portfolio-wallets)** - Returns `network` field for vault wallets
-   **[Get Wallet by ID](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/get-wallet-by-wallet-id)** - Returns `network` field for vault wallets
-   **[Create Wallet](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/create-wallet)** - Accepts optional `network` parameter for vault wallets
-   **[Get Wallet Deposit Instructions](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/get-wallet-deposit-instructions)** - Accepts optional `network` parameter and returns `network` field for onchain deposits
-   **[Create Wallet Deposit Address](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/create-wallet-deposit-address)** - Requires network and generates a network-specific deposit address