# assets

An asset is a representation of value on a blockchain network. Common types of assets include (fungible) tokens and NFTs (non-fungible tokens). CDP APIs support certain popular assets by their symbols and the vast majority by their contract addresses.

### Assets supported by symbol

The CDP APIs support the following assets on the Base Sepolia & Mainnet networks to be identified by their symbols.

Asset

Type

Base-Mainnet

Base-Sepolia

Description

**Ether**, also known as **ETH**

native

✅

✅

This is the native token of many networks that run on the [Ethereum Virtual Machine (EVM)](https://ethereum.org/en/developers/docs/evm/), including Base. ETH is used to pay for transactions on the network, and the network provides native APIs to send, receive, and otherwise interact with ETH.

**USDC**

ERC‑20

✅

✅

[backed 1:1 by a U.S. Dollar](https://www.circle.com/en/usdc).

**WETH**

ERC-20

✅

✅

[backed 1:1 by ETH](https://basescan.org/token/0x4200000000000000000000000000000000000006).

**DAI**

ERC-20

✅

➖

[Dai Stablecoin](https://basescan.org/token/0x50c5725949a6f0c72e6c4a641f24049a917db0cb) on Base-Mainnet.

**RETH**

ERC-20

✅

➖

[Rocket Pool ETH](https://basescan.org/token/0xb6fe221fe9eef5aba221c348ba20a1bf5e73624c) on Base-Mainnet.

**BRETT**

ERC-20

✅

➖

[Brett](https://basescan.org/token/0x532f27101965dd16442e59d40670faf5ebb142e4) on Base-Mainnet.

**W**

ERC-20

✅

➖

[Wormhole Token](https://basescan.org/token/0xb0ffa8000886e57f86dd5264b9582b2ad87b2b91) on Base-Mainnet.

**CBETH**

ERC-20

✅

➖

[Coinbase Wrapped Ether](https://basescan.org/token/0x2ae3f1ec7f1f5012cfeab0185bfc7aa3cf0dec22) on Base-Mainnet.

**AXL**

ERC-20

✅

➖

[Axelar](https://basescan.org/token/0x23ee2343b892b1bb63503a4fabc840e0e2c6810f) on Base-Mainnet.

**IOTX**

ERC-20

✅

➖

[IoTeX](https://basescan.org/token/0xbcbaf311cec8a4eac0430193a528d9ff27ae38c1) on Base-Mainnet.

**PRIME**

ERC-20

✅

➖

[Prime](https://basescan.org/token/0xfa980ced6895ac314e7de34ef1bfae90a5add21b) on Base-Mainnet.

**AERO**

ERC-20

✅

➖

[Aerodrome](https://basescan.org/token/0x940181a94a35a4569e4529a3cdfb74e38fd98631) on Base-Mainnet.

**RSR**

ERC-20

✅

➖

[Reserve Rights](https://basescan.org/token/0xab36452dbac151be02b16ca17d8919826072f64a) on Base-Mainnet.

**MOG**

ERC-20

✅

➖

[Mog Coin](https://basescan.org/token/0x2da56acb9ea78330f947bd57c54119debda7af71) on Base-Mainnet.

**TBTC**

ERC-20

✅

➖

[Base tBTC v2](https://basescan.org/token/0x236aa50979d5f3de3bd1eeb40e81137f22ab794b) on Base-Mainnet.

**NPC**

ERC-20

✅

➖

[Non-Playable Coin](https://basescan.org/token/0x236aa50979d5f3de3bd1eeb40e81137f22ab794b) on Base-Mainnet.

**YFI**

ERC-20

✅

➖

[Yearn Finance](https://basescan.org/token/0x9eaf8c1e34f05a589eda6bafdf391cf6ad3cb239) on Base-Mainnet.

In addition to Base, CDP APIs also support **ETH** & **USDC** on Ethereum Mainnet, **MATIC** & **USDC** on Polygon Mainnet and **ARB** & **USDC** on Arbitrum Mainnet.

### Assets supported by contract address

Besides the assets listed in the above table, CDP APIs support all other ERC20 tokens using their respective contract addresses.

#### Transfer an ERC20 from a wallet with contract address

The following example demonstrates how to create a transfer for USDC on Base-Sepolia using its contract address. Use [Circle faucet](https://faucet.circle.com/) to fund your wallet.

#### Trade an ERC20 in a wallet with contract address

The following example demonstrates how to create a trade for USDC on Base-Mainnet using its contract address. Remember to fund your wallet with USDC to complete the trade.

## Fetch balances for assets

### Fetch balance for assets identified by symbol

### Fetch balance for assets identified by contract address

## Denominations of ETH

ETH provides 18 places of decimal precision. The smallest amount of sendable ETH is 10\-18, also known as a Wei. Commonly used denominations of ETH:

Denomination

Amount in Wei

Description

Wei

1 Wei

Smallest denomination of ETH

Gwei

109 Wei

Denomination of ETH commonly used for gas (i.e., transaction fee) calculations

Ether / ETH

1018 Wei

Largest denomination of ETH, commonly used for trading

The SDK supports transfers in denominations of Wei, Gwei, and ETH.

-   Typescript
    
-   Python
    

**SDK Documentation**You can refer to the [Asset class SDK docs](https://coinbase.github.io/coinbase-sdk-nodejs/classes/coinbase_asset.Asset.html) for a full list of supported methods.In Node.js, asset IDs are accessed through the assets property of the Coinbase class.

-   ETH’s asset ID is `Coinbase.assets.Eth`
-   USDC’s asset ID is `Coinbase.assets.Usdc`
-   WETH’s asset ID is `Coinbase.assets.Weth`

**SDK Documentation**You can refer to the [Asset class SDK docs](https://coinbase.github.io/cdp-sdk-python/cdp.html#cdp.asset.Asset) for a full list of supported methods.In Python, asset IDs are strings that resemble tickers:

-   ETH’s asset ID is `"eth"`
-   USDC’s asset ID is `"usdc"`
-   WETH’s asset ID is `"weth"`