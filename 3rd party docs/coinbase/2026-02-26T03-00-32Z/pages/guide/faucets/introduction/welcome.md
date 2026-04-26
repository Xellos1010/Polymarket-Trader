# welcome

A **faucet** is a service that dispenses small amounts of cryptocurrency (usually testnet tokens) to developers for testing purposes.

## CDP Faucets

The Coinbase Developer Platform (CDP) SDK provides a Faucet API that allows you to obtain test tokens using either of the following methods:

-   **UI:** Use your CDP account with our [Portal](https://developer.chrome.com/quickstart#ui-cdp-portal) for in-browser Faucet management
-   **Programmatically:** Use our Server Wallet to [create a new account](https://developer.chrome.com/quickstart#1-setup-cdp-client-and-create-an-account), or [bring your own](https://developer.chrome.com/quickstart#4-use-an-external-address-optional)

We currently offer faucets that support either native [ETH](https://ethereum.org/en/developers/docs/intro-to-ethereum/#eth), [ERC-20](https://ethereum.org/en/developers/docs/standards/tokens/erc-20/), or [SPL](https://spl.solana.com/) token standards. Each standard has a claim limit as described below.

## Supported Assets

CDP Faucets support [Base Sepolia](https://sepolia.basescan.org/), [Ethereum Sepolia](https://sepolia.etherscan.io/), and [Solana Devnet](https://explorer.solana.com/) testnets.

### EVM compatible

The following assets are supported on Base Sepolia and Ethereum Sepolia testnets:

Symbol

Token standard

Number of claims per 24 hours

Testnet funds per claim

ETH

Native

1000

0.0001 ETH

USDC

ERC-20

10

1 USDC

EURC

ERC-20

10

1 EURC

cbBTC

ERC-20

100

0.0001 cbBTC

### Solana

The following assets are supported on Solana Devnet:

Symbol

Token standard

Number of claims per 24 hours

Devnet funds per claim

USDC

SPL

10

1 USDC

SOL

SPL

10

0.00125 SOL

## What to read next

-   [**Faucet Quickstart**](https://developer.chrome.com/quickstart): Create a faucet request using the CDP SDK
-   [**Server Wallet**](https://developer.chrome.com/server-wallets/v2/introduction/welcome): Create and manage accounts using the CDP SDK